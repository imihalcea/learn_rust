use crate::http::{ParseError, Request, Response, StatusCode};

pub trait Handler{
    fn handle_request(&self, request:Request) -> Response;

    fn handle_bad_request(&self, _:ParseError) -> Response{
        Response::new(StatusCode::BadRequest, None)
    }
}
