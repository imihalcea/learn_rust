use std::collections::HashMap;
use regex::Regex;
use crate::app::controllers::ProductController;
use crate::http::{Method, ParseError, Request, Response, StatusCode};
use crate::handler::Handler;

type FnApp = dyn Fn(&HashMap<String,String>) -> Result<String, String>;

struct Mapping{
    path:&'static str,
    f:Box<FnApp>
}

impl Mapping {
    fn new(path:&'static str, f: Box<FnApp>) -> Self{
        Self{path,f}
    }

    fn apply(&self, input:&HashMap<String, String>) -> Result<String, String>{
        (self.f)(input)
    }
}

pub struct Router
{
    map_get: Vec<Mapping>
}


impl Handler for Router{
    fn handle_request(&self, request: Request) -> Response {
        let app_result = self.route_request(&request);
        Self::create_response(app_result)
    }
}


impl Router {
    pub fn new() -> Router{
        let mut map_get = vec![];
        map_get.push(Mapping::new("/article",Box::new(|args|ProductController::new().get_all(args))));
        Self{
           map_get
        }
    }

    pub fn route_request(&self, request:&Request) -> Result<String, String>{
        let map = match request.method() {
             Method::GET => Ok(&self.map_get),
             _ => Err(format!("Method is not supported"))
        }?;

        let path = request.path();
        let input = HashMap::new();
        for m in map {
            if path.matches_exactly("/article") {
                return m.apply(&input);
            }
        }
        Err(format!("No mapping found"))
    }

    fn create_response(result: Result<String, String>) -> Response{
        match result {
            Ok(json) => Response::new(StatusCode::Ok, Some(json)),
            Err(err) => Response::new(StatusCode::InternalServerError, Some(err))
        }
    }

}