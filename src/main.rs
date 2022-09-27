use std::convert::Infallible;
use std::net::SocketAddr;

use futures::executor;

use serde::{Deserialize, Serialize};
// use serde_json::Result;

use hyper::{Body, Request, Response, Server, Method, StatusCode};
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
            if let Ok(r) = executor::block_on(proxy_request_from_body(req)) {
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

#[derive(Debug, Serialize, Deserialize)]
struct RequestData {
    url: String,
    payload: String,
}

async fn proxy_request_from_body(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let rd: RequestData;
    let s: String = "".to_string();

    // Convert body into string
    let bytes = hyper::body::to_bytes(req.into_body()).await;
    // let bytes = executor::block_on(hyper::body::to_bytes(req.body())).unwrap();
    let i = bytes.iter();
    let s = String::from_utf8(i.collect()).expect("");

    // Load JSON string into RequestData struct


    match serde_json::from_str(&s) {
        Ok(j) => { rd = j }
        Err(e) => { eprintln!("Error: {}", e) }
    }
    // Make request for data
    // Error handle responses
    // Return Result

    Ok(Response::new(Body::from("TEST")))
}
