extern crate tiny_http;

use tiny_http::{Response, Server};

fn main() {
    let server = Server::http("0.0.0.0:3000").unwrap();

    for request in server.incoming_requests() {
        let response = Response::from_string("hello world");
        request.respond(response).unwrap();
    }
}
