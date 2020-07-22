#![feature(async_closure)]

use std::time::Instant;

use futures::stream::futures_unordered::FuturesUnordered;
use futures::stream::StreamExt;
use tokio::task;

use dots_poc::proto::dots_client::DotsClient;
use dots_poc::proto::DotCreateRequest;

fn dot_worker(
    client: DotsClient<tonic::transport::Channel>,
    worker_id: i32,
    dots_to_create: i32,
) -> task::JoinHandle<()> {
    println!(
        "About to start thread {0} to create {1}",
        worker_id, dots_to_create
    );
    let task = task::spawn(async move {
        // println!("Task started for worker {0}", worker_id);
        let mut dot_client = client.clone();

        let mut created = 0;
        while created <= dots_to_create {
            let now = Instant::now();

            match create_dot(&mut dot_client, worker_id, created).await {
                Ok(_) => {
                    // println!("Worker {0} created dot #{1}!", worker_id, created);
                    created += 1;
                    println!("Request took {0}", now.elapsed().as_millis());
                }
                Err(_) => println!("Error"),
            }
        }
        println!("Task completed");
    });

    task
}

async fn create_dot(
    client: &mut DotsClient<tonic::transport::Channel>,
    worker: i32,
    num: i32,
) -> Result<(), String> {
    // println!("Firing off request");
    let request = tonic::Request::new(DotCreateRequest {
        worker,
        author: format!("author{0}-{1}", worker, num),
        subject: format!("subject{0}-{1}", worker, num),
        attribute: format!("attribute{0}-{1}", worker, num),
        comment: format!("comment{0}-{1}", worker, num),
        rating: 5,
        importance: 3,
    });

    let result = client.create_dot(request).await;
    result
        .map(|_| ())
        .map_err(|_| String::from("Unknown error"))
}

#[tokio::main(core_threads = 32)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DotsClient::connect("http://[::1]:50051").await?;

    let now = Instant::now();

    (1..50)
        .collect::<Vec<i32>>()
        .into_iter()
        .map(|worker| dot_worker(client.clone(), worker, 1000))
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await;

    println!("{}", now.elapsed().as_millis());
    Ok(())
}
