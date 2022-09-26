use std::convert::Infallible;
use std::net::SocketAddr;

use futures::{executor};
// use futures::task::{Poll, Context};

use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn: &AddrStream| async {
        Ok::<_, Infallible>(service_fn(handler))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, _) => {
            *response.body_mut() = Body::from("This endpoint only supports POST requests");
        }
        (&Method::POST, _) => {
            // Do HTTP proxy stuff here
            if let Ok(r) = executor::block_on(proxy_request_from_body(req.body())) {
                *response.body_mut() = r.into_body();
            } else {
                println!("Something went wrong!");
            }
        }
        _ => {
            *response.body_mut() = Body::from("Something went wrong");
        }
    }

    Ok(response)
}

async fn proxy_request_from_body(body: &Body) -> Result<Response<Body>, Infallible> {
    // future::ready(Ok(Response::new(Body::from("TEST"))))
    Ok(Response::new(Body::from("TEST")))
}
