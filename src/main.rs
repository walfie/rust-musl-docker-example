extern crate hyper;
extern crate futures;
extern crate hyper_tls;
extern crate tokio_core;

use futures::Stream;
use hyper::{Client, Uri};
use hyper::server::{Http, Request, Response, Service};
use hyper_tls::HttpsConnector;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().expect("failed to create Core");
    let handle = core.handle();

    let port = 8080;

    // Needs to be 0.0.0.0 and not 127.0.0.1, or it won't work with Docker
    let bind_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    let listener = tokio_core::net::TcpListener::bind(&bind_address, &handle)
        .expect("failed to bind TcpListener");

    let connector = HttpsConnector::new(4, &handle).expect("failed to create HttpsConnector");

    let client = Client::configure().connector(connector).build(&handle);
    let server = Server {
        client,
        base_uri: "https://httpbin.org".parse().unwrap(),
    };

    let http = Http::new();
    let worker = listener.incoming().for_each(move |(sock, addr)| {
        http.bind_connection(&handle, sock, addr, server.clone());
        Ok(())
    });

    println!("Server listening on port {}", port);
    core.run(worker).unwrap();
}

#[derive(Clone)]
struct Server {
    client: Client<HttpsConnector<hyper::client::HttpConnector>>,
    base_uri: Uri,
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = hyper::client::FutureResponse;

    fn call(&self, incoming: Request) -> Self::Future {
        let uri = format!("{}{}", self.base_uri, incoming.uri().path())
            .parse()
            .expect("invalid URI");
        let mut outgoing = Request::new(incoming.method().clone(), uri);

        {
            let outgoing_headers = outgoing.headers_mut();
            outgoing_headers.clone_from(incoming.headers());
            outgoing_headers.remove::<hyper::header::Host>();
        }

        outgoing.set_version(incoming.version());
        outgoing.set_body(incoming.body());

        println!("Sending: {:?}", outgoing);

        self.client.request(outgoing)
    }
}
