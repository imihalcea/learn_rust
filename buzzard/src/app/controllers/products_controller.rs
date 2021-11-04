use crate::http::{Request, Response};
use serde::Serialize;
use std::fmt::Display;

pub struct ProductController {

}
#[derive(Serialize)]
pub struct Product{
    id:uuid::Uuid,
    name:String,
    price:u16
}

impl ProductController {
    pub fn new()->Self{
        return Self{}
    }
    pub fn get_all(&self, _ :&Request) -> Result<String, String>{
        let products = [
            Product{
                id:uuid::Uuid::new_v4(),
                name:"P1".to_string(),
                price:42
            },
            Product{
                id:uuid::Uuid::new_v4(),
                name:"P2".to_string(),
                price:21
            }
        ];

        match serde_json::to_string(&products){
            Ok(json) => Ok(json),
            Err(_) => Err("Serialization error".to_string())
        }
    }
}

