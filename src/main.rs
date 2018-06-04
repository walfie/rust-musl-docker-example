extern crate hyper;
extern crate hyper_tls;

use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Client, Request, Server};
use hyper_tls::HttpsConnector;

fn main() {
    // Needs to be 0.0.0.0 and not 127.0.0.1, or it won't work with Docker
    let bind_address = "0.0.0.0:8080".parse().unwrap();

    let connector = HttpsConnector::new(1).expect("failed to create HttpsConnector");

    let client = Client::builder().build(connector);

    let server = Server::bind(&bind_address)
        .serve(move || {
            let cloned_client = client.clone();

            service_fn(move |incoming: Request<Body>| {
                let uri = format!("https://httpbin.org{}", incoming.uri().path())
                    .parse()
                    .expect("invalid URI");

                let (mut parts, body) = incoming.into_parts();
                parts.uri = uri;
                parts.headers.remove(hyper::header::HOST);
                let outgoing = Request::from_parts(parts, body);

                println!("Sending: {:?}", outgoing);

                cloned_client.request(outgoing)
            })
        })
        .map_err(|e| eprintln!("error: {}", e));

    println!("Server listening on {}", bind_address);
    hyper::rt::run(server);
}
