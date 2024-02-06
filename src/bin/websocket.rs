use std::thread;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let port = "127.0.0.1:8080";
    let listener = TcpListener::bind(port).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        let response = "HTTP/1.1 200 OK\r\n\r\n";

        //Here, we send the data back to the client
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
struct Worker {
    id: u32,
    //thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: u32, number: u32) -> Self {
        Worker { id }
    }
}

mod client {

    fn create_connection() {}
}

mod server {

    fn read_request() {}
}
