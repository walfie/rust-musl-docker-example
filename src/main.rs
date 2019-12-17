use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Server};
use hyper_tls::HttpsConnector;
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let base_client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

    let make_svc = make_service_fn(|_socket: &AddrStream| {
        let client = base_client.clone();

        async {
            Ok::<_, Infallible>(service_fn(move |incoming: Request<Body>| {
                let uri = format!("https://httpbin.org{}", incoming.uri().path())
                    .parse()
                    .expect("invalid URI");

                let (mut parts, body) = incoming.into_parts();
                parts.uri = uri;
                parts.headers.remove(hyper::header::HOST);
                let outgoing = Request::from_parts(parts, body);

                client.request(outgoing)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
