use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::Session;
use cdrs::cluster::TcpConnectionPool;
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;

use cdrs::frame::Frame;
use cdrs::types::prelude::*;

type CqlSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

pub fn create_keyspace(session: &CqlSession) -> Result<Frame> {
    let create_keyspace_sql = "CREATE KEYSPACE IF NOT EXISTS test_ks \
     WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '1' };";

    session.query(create_keyspace_sql)
}

pub fn create_table(session: &CqlSession) -> Result<Frame> {
    let create_table_cql = "CREATE TABLE IF NOT EXISTS test_ks.dots \
        (\
          dot_id int PRIMARY KEY, \
          author text, \
          subject text, \
          comment text, \
          attribute text, \
          rating int, \
          importance int\
        );";

    session.query(create_table_cql)
}
