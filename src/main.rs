// use std::error::Error;
use std::net::SocketAddr;
use std::collections::HashMap;

use serde::Deserialize;
// use serde_json;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri};
use hyper::{Method, StatusCode};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(move |conn: &AddrStream| {
        println!("Connection from: {}", conn.remote_addr());
        async { Ok::<_, hyper::Error>(service_fn(routes_svc)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn routes_svc(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, _) => {
            *response.body_mut() =
                Body::from("This service does not support GET requests. Please POST instead");
        }
        (&Method::POST, "/") => {
            let r = handle_request(req).await;
            match r {
                Ok(r) => {
                    // Need to do some more error catching here
                    response = r;
                }
                Err(_) => *response.status_mut() = StatusCode::BAD_REQUEST,
            }
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    Ok(response)
}

#[derive(Debug, Deserialize)]
struct RequestData {
    method: String,
    uri: String,
    headers: String,
    body: String,
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let bytes = hyper::body::to_bytes(req.into_body()).await?;
    let data = bytes.iter().cloned().collect::<Vec<u8>>();
    let rd: RequestData =
        serde_json::from_str(&String::from_utf8_lossy(&data)).expect("JSON wasn't well formatted.");

    // Build request
    let mut new_request = Request::builder()
        .uri(&rd.uri)
        .body(&rd.body);

    let headers_hash: HashMap<String, String> = serde_json::from_str(&rd.headers).expect("Headers JSON wasn't well formatted");
    // Iterate over every header in rd.headers and add it to new_request
    for header in headers_hash.iter() {
        new_request.header()
    }

    let mut remote_data = String::from("");
    match rd.method.as_str() {
        "GET" => {
            remote_data = get_data(&rd).await?;
        }
        _ => {
            // If not GET request then just return empty JSON
            remote_data = String::from("{}")
        }
    }

    Ok(Response::new(Body::from(remote_data)))
}

async fn get_data(rd: &RequestData) -> Result<String, hyper::Error> {
    let client = Client::new();
    let uri = rd.uri.parse::<hyper::Uri>().unwrap();
    let headers = rd.headers.clone();
    let mut resp = client.get(uri).await?;

    let bytes = hyper::body::to_bytes(resp.into_body())
        .await?
        .iter()
        .cloned()
        .collect::<Vec<u8>>();
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

async fn send_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::empty()))
}
