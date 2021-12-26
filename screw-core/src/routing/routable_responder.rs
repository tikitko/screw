use super::router::Router;
use crate::routing::Request;
use crate::server::Responder;
use hyper::http::Extensions;
use hyper::Body;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

pub struct RoutableResponder {
    pub(crate) remote_addr: SocketAddr,
    pub(crate) extensions: Arc<Extensions>,
    pub(crate) router: Arc<Router>,
}

impl Responder for RoutableResponder {
    type ResponseFuture = Pin<Box<dyn Future<Output = hyper::Response<Body>> + Send>>;

    fn response(&mut self, http_request: hyper::Request<Body>) -> Self::ResponseFuture {
        let remote_addr = self.remote_addr;
        let extensions = self.extensions.clone();
        let router = self.router.clone();

        Box::pin(async move {
            let request = Request {
                remote_addr,
                extensions,
                http: http_request,
            };
            let response = router.process(request).await;
            response.http
        })
    }
}