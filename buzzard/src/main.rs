use log::{info};
use env_logger::{Builder, Target};

fn main() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    let server = Server::new(String::from("127.0.0.1"), 8080);
    server.run();
}

struct Server {
    address:String,
    port:u16
}

impl Server {
    fn new(address:String, port:u16) -> Self {
        Server{address, port}
    }

    fn run(self){
        info!("Buzzard is running on {}:{}", self.address, self.port);
    }
}

enum Method{
    GET,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

struct Request{
    path:String,
    query_string:Option<String>,
    method:Method,
}