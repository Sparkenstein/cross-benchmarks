extern crate env_logger;
extern crate futures;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;

use futures::future;
use tokio_minihttp::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;

struct HelloWorld;

impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&self, _request: Request) -> Self::Future {
        let mut resp = Response::new();
        resp.body("Hello, world!");
        future::ok(resp)
    }
}

fn main() {
    drop(env_logger::init());
    let addr = "0.0.0.0:8080".parse().unwrap();
    TcpServer::new(Http, addr)
        .serve(|| Ok(HelloWorld));
}

/*
debug:
Running 30s test @ http://localhost:8080/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.66ms  623.53us  41.83ms   99.40%
    Req/Sec     5.33k   177.18     5.74k    78.08%
  637167 requests in 30.05s, 62.59MB read
Requests/sec:  21206.41
Transfer/sec:      2.08MB

Release:
Running 30s test @ http://localhost:8080/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   767.68us  495.34us  30.60ms   99.66%
    Req/Sec    39.88k     1.58k   46.58k    86.17%
  4762521 requests in 30.03s, 467.82MB read
Requests/sec: 158615.27
Transfer/sec:     15.58MB

*/