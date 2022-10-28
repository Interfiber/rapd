use std::net::TcpListener;
use std::thread;

pub struct RapdServer {
    port: i32,
}

impl RapdServer {
    pub fn new(p: i32) -> RapdServer {
        RapdServer { port: p }
    }

    pub fn start(&self) {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = match TcpListener::bind(addr.to_string()) {
            Ok(t) => t,
            Err(err) => {
                error!("Failed to bind to address! Error: {}", err);
                std::process::exit(0);
            }
        };
        info!("Started server at: {}", addr);
        for stream in listener.incoming() {
            // handle connection
            thread::Builder::new()
                .name(String::from("client_handler"))
                .spawn(move || crate::client::handle_client(stream.unwrap()))
                .expect("Failed to spawn client handler thread!");
        }
    }
}
