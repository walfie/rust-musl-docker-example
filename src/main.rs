extern crate hyper;
extern crate hyper_tls;

use hyper::client::ResponseFuture;
use hyper::rt::Future;
use hyper::service::Service;
use hyper::{Body, Client, Request, Server, Uri};
use hyper_tls::HttpsConnector;

fn main() {
    // Needs to be 0.0.0.0 and not 127.0.0.1, or it won't work with Docker
    let bind_address = "0.0.0.0:8080".parse().unwrap();

    let connector = HttpsConnector::new(1).expect("failed to create HttpsConnector");

    let service = ProxyService {
        client: Client::builder().build(connector),
        base_uri: "https://httpbin.org".parse().unwrap(),
    };

    let server = Server::bind(&bind_address)
        .serve(move || Ok::<_, hyper::Error>(service.clone()))
        .map_err(|e| eprintln!("error: {}", e));

    println!("Server listening on {}", bind_address);
    hyper::rt::run(server);
}

#[derive(Clone)]
struct ProxyService {
    client: Client<HttpsConnector<hyper::client::HttpConnector>>,
    base_uri: Uri,
}

impl Service for ProxyService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = ResponseFuture;

    fn call(&mut self, incoming: Request<Self::ReqBody>) -> Self::Future {
        let uri = format!("{}{}", self.base_uri, incoming.uri().path())
            .parse()
            .expect("invalid URI");

        let (mut parts, body) = incoming.into_parts();
        parts.uri = uri;
        parts.headers.remove(hyper::header::HOST);
        let outgoing = Request::from_parts(parts, body);

        println!("Sending: {:?}", outgoing);

        self.client.request(outgoing)
    }
}
