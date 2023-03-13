use std::time::Duration;

use hyper::Client;
use tower::{retry::Policy, Service, ServiceBuilder, ServiceExt};

fn main() {
    let client = Client::new();

    let service = ServiceBuilder::new()
        .retry(RetryPolicy {})
        .rate_limit(1, Duration::from_secs(1))
        .service(client);

    service.ready();

    // service.call(hyper::Request::new(hyper::Body::empty()));
}

#[derive(Clone)]
struct RetryPolicy {}

impl<Req, Res, E> Policy<Req, Res, E> for RetryPolicy {
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Self>>>;

    fn retry(&self, req: &Req, result: Result<&Res, &E>) -> Option<Self::Future> {
        todo!()
    }

    fn clone_request(&self, req: &Req) -> Option<Req> {
        todo!()
    }
}
