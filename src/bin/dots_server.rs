use dots_poc::proto::dots_server::DotsServer;

use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;

use dots_poc::dots::dots_service::DefaultDotsService;
use tonic::transport::Server;

#[tokio::main(core_threads = 256)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = NodeTcpConfigBuilder::new("127.0.0.1:9042", NoneAuthenticator {})
        .max_size(20)
        .build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let session: Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>> =
        new_session(&cluster_config, RoundRobin::new())?;

    let addr = "[::1]:50051".parse()?;
    let dot_service = DefaultDotsService::new(session);

    Server::builder()
        .concurrency_limit_per_connection(20)
        .add_service(DotsServer::new(dot_service))
        .serve(addr)
        .await?;

    Ok(())
}
