# Sample Rust with GRPC and Cassandra

## Prerequisites
You should have cassandra running either in a container or natively on your machine
You should also have the rust toolchain installed.

## Commands

All connections to cassandra in this project log in with no credentials.

### dot_table_setup

This target will set up the keyspace and the base dots table.

### dots_server

This target will start up the grpc service.

### dots_client

This target will start up the client and send requests to the service.