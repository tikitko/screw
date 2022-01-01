use crate::routing::{Request, RequestResponseConverter, Response};
use async_trait::async_trait;

pub struct DefaultConverter;

#[async_trait]
impl RequestResponseConverter<Request, Response> for DefaultConverter {
    type Request = Request;
    type Response = Response;
    async fn convert_request(&self, request: Self::Request) -> Request {
        request
    }
    async fn convert_response(&self, response: Response) -> Self::Response {
        response
    }
}
