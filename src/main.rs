use std::net::SocketAddr;
use std::error::Error;

use serde::Deserialize;
// use serde_json;

use hyper::{Body, Request, Response, Client, Server, Uri};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};
use hyper::server::conn::AddrStream;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(move |conn: &AddrStream| {
        println!("Connection from: {}", conn.remote_addr());
        async { Ok::<_, hyper::Error>(service_fn(routes)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn routes(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, _) => {
            *response.body_mut() = Body::from("This service does not support GET requests. Please POST instead");
        },
        (&Method::POST, "/") => {
            let r = handle_request(req).await;
            match r {
                Ok(r) => {
                    // Might need to do some more error catching here
                    response = r;
                }, 
                Err(_) => {
                    *response.status_mut() = StatusCode::BAD_REQUEST
                }
            }
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };
    Ok(response)
}

#[derive(Debug, Deserialize)]
struct RequestData {
    uri: String,
    headers: String,
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let bytes = hyper::body::to_bytes(req.into_body()).await?;
    let data = bytes.iter().cloned().collect::<Vec<u8>>();
    let rd: RequestData = serde_json::from_str(&String::from_utf8_lossy(&data)).expect("JSON wasn't well formatted.");

    let remote_data = get_data(&rd).await?;
    Ok(Response::new(Body::from(format!("{}", remote_data))))
}

async fn get_data (rd: &RequestData) -> Result<String, hyper::Error> {
    let client = Client::new();
    let uri = rd.uri.parse::<hyper::Uri>().unwrap();
    let mut resp = client.get(uri).await?;

    let bytes = hyper::body::to_bytes(resp.into_body())
        .await?
        .iter()
        .cloned()
        .collect::<Vec<u8>>();
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}
