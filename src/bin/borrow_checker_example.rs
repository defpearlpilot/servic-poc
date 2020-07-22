use cdrs::authenticators::NoneAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;

use cdrs::error::Error;
use cdrs::frame::Frame;
use dots_poc::dots::cassandra::setup::create_table;

struct S {
    v: i32,
}

fn transfer_ownership(v: S) {
    println!("I own this now {}", v.v)
}

fn return_ownership(v: S) -> S {
    println!("I am returning ownership {}", v.v);
    v
}

fn borrow_reference(v: &S) {
    println!("I borrowed this {}", v.v)
}

fn change_me(v: &mut S) {
    v.v = 2
}

fn main() {
    let s = S { v: 0 };
    let mut m = S { v: 0 };
    let t = S { v: 0 };

    borrow_reference(&s);
    transfer_ownership(s);

    change_me(&mut m);
    println!("My value is now {0}", &m.v);
    // borrow_reference(&s)
    // transfer_ownership(s);

    let o = return_ownership(t);
    println!("My value is now {0}", o.v);
    // println!("My value is now {0}", t.v);
}
