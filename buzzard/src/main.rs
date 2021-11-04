#![feature(entry_insert)]
#![allow(dead_code)]

use env_logger::{Builder, Target};

use app::controllers::*;
use server::Server;
use router::Router;

mod server;
mod http;
mod app;
mod router;
mod handler;


fn main() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    let server = Server::new(String::from("127.0.0.1:8080"));
    let router = Router::new();
    server.run(router);
}



