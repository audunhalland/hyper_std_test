use std::convert::Infallible;

//use async_std::task;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}

fn main() {
        // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async {
            Ok::<_, Infallible>(service_fn(hello))
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    async_std::task::block_on(server).unwrap();
}
