extern crate futures;
extern crate log4rs;
extern crate mta_status;
extern crate net2;
extern crate num_cpus;
extern crate tokio_core;
extern crate hyper;

use tokio_core::reactor::Handle;
use hyper::server::{Request, Response};
use hyper::{Method, StatusCode};
use hyper::server::Service;
use hyper::header::Headers;

use futures::Future;

header! { (AccessControl, "Access-Control-Allow-Origin") => [String] }

pub struct GetStatus {
    _handle: Handle,
}

impl GetStatus {
    pub fn new(handle: Handle) -> Self {
        GetStatus { _handle: handle }
    }
}

impl Service for GetStatus {
    type Request = hyper::server::Request;
    type Response = hyper::server::Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = hyper::server::Response, Error = hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let resp = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                let status = mta_status::get_status(&self._handle).map(|stat| {
                    let mut headers = Headers::new();
                    headers.set(AccessControl("*".to_owned()));

                    resp.with_body(stat).with_headers(headers).with_status(
                        StatusCode::Ok,
                    )
                });
                Box::new(status)
            }
            _ => Box::new(futures::future::ok(
                resp.with_body("no path").with_status(StatusCode::NotFound),
            )),
        }

    }
}
