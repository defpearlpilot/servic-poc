use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;

use dots_poc::dots::cassandra::setup::{create_keyspace, create_table};

type CqlSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

fn main() {
    let node = NodeTcpConfigBuilder::new("127.0.0.1:9042", NoneAuthenticator {}).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    let session: CqlSession =
        new_session(&cluster_config, RoundRobin::new()).expect("session should be created");

    match create_keyspace(&session) {
        Ok(_) => {
            println!("Successfully created keyspace!");
        }
        Err(err) => {
            println!("Failed to create keyspace {}", err);
        }
    }

    match create_table(&session) {
        Ok(_) => {
            println!("Successfully created table!");
        }
        Err(err) => {
            println!("Failed to create table {}", err);
        }
    }
}
