#![allow(dead_code)]
use env_logger::{Builder, Target};

use server::Server;

mod server;
mod http;


fn main() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}



