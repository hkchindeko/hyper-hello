use std::convert::Infallible;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, world".into()))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // We'll bind to 127.0.0.1:3000
    let addr = ([127,0,0,1], 3000).into();

    // A 'Service' is needed for ever connection, so this
    // creates one from our 'hello_world' function.
    let make_svc = make_service_fn(|_conn| {
        // service_fn converts our function into a 'Service'
        async { Ok::<_, Infallible>(service_fn(hello_world)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    // if let Err(e) = server.await {
    //     eprintln!("Server error: {}", e);
    // }
    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}