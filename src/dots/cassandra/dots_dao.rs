use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::Session;
use cdrs::cluster::TcpConnectionPool;
use cdrs::frame::Frame;
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;
use cdrs::types::prelude::*;

use crate::dots::data_model::Dot;

type CqlSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

pub fn insert_dot(session: &CqlSession, dot: Dot) -> Result<Frame> {
    let insert_dot_cql = "\
      INSERT INTO test_ks.dots \
        (dot_id, author, subject, comment, attribute, rating, importance)\
      VALUES\
        (?, ?, ?, ?, ?, ?, ?)";
    session.query_with_values(insert_dot_cql, dot.into_query_values())
}
