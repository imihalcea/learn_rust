use std::collections::HashMap;
use crate::app::controllers::ProductController;
use crate::http::{Method, ParseError, Request, Response, StatusCode};
use crate::handler::Handler;

pub struct Router{
    product_crtl:ProductController
}

impl Handler for Router{
    fn handle_request(&self, request: Request) -> Response {
        match (request.method(), request.path()) {
            (Method::GET, p) if p.matches_exactly("/article") => Self::create_response(self.product_crtl.get_all(HashMap::new())),
            (_,__) => Response::new(StatusCode::NotFound,None)
        }
    }
}


impl  Router {
    pub fn new() -> Router{
        Self{
           product_crtl:ProductController::new()
        }
    }

    fn create_response(result:Result<String, String>) -> Response{
        match result {
            Ok(json) => Response::new(StatusCode::Ok, Some(json)),
            Err(err) => Response::new(StatusCode::InternalServerError, Some(err))
        }
    }
}
