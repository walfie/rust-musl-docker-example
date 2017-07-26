extern crate hyper;
extern crate futures;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use futures::future;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    let port = 8080;

    // Needs to be 0.0.0.0 and not 127.0.0.1, or it won't work with Docker
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    let server = Http::new().bind(&addr, || Ok(Server)).unwrap();

    println!("Server listening on port {}", port);
    server.run().unwrap();
}

struct Server;
impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        println!("{:?}", req);

        let content = format!("{:#?}", req);
        let response = Response::new()
            .with_header(ContentLength(content.len() as u64))
            .with_body(content);

        future::ok(response)
    }
}
