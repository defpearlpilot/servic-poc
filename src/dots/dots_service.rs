use std::sync::{Arc, Mutex};
use std::time::Instant;

use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::Session;
use cdrs::cluster::TcpConnectionPool;
use cdrs::load_balancing::RoundRobin;
use tonic::{Request, Response, Status};

use crate::dots::cassandra::dots_dao::insert_dot;
use crate::dots::data_model::Dot;
use crate::proto::dots_server::Dots;
use crate::proto::{DotCreateRequest, DotCreateResponse};

pub struct DefaultDotsService {
    session: Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>,
    counter: Arc<Mutex<i32>>,
}

impl DefaultDotsService {
    pub fn new(session: Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>) -> Self {
        DefaultDotsService {
            session,
            counter: Arc::new(Mutex::new(0)),
        }
    }
}

#[tonic::async_trait]
impl Dots for DefaultDotsService {
    async fn create_dot(
        &self,
        request: Request<DotCreateRequest>,
    ) -> Result<Response<DotCreateResponse>, Status> {
        let request_start = Instant::now();
        let create_request = request.into_inner();
        println!("starting task from worker {0}", create_request.worker);

        let mut id = self.counter.lock().unwrap();
        let dot_to_insert: Dot = Dot::new(
            *id,
            create_request.author,
            create_request.subject,
            create_request.attribute,
            create_request.comment,
            create_request.rating,
            create_request.importance,
        );

        *id += 1;

        let insert_start = Instant::now();

        match insert_dot(&self.session, dot_to_insert) {
            Ok(_) => {
                println!(
                    "cassandra insert took: {0} ms",
                    insert_start.elapsed().as_millis()
                );

                let response = DotCreateResponse {
                    dot_id: format!("{0}", 0),
                    attribute: String::from(""),
                    author: String::from(""),
                    subject: String::from(""),
                    comment: String::from(""),
                    importance: 1,
                    rating: 1,
                };

                println!(
                    "completed task from worker {0} in {1} ms",
                    create_request.worker,
                    request_start.elapsed().as_millis()
                );

                Ok(Response::new(response))
            }
            Err(err) => {
                println!("{0}", err);
                let message: String = format!("{0}", err);
                Err(Status::unknown(message))
            }
        }
    }
}
