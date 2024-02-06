use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

fn send_equation() {
    //take stdin
    //send data
    //read response
    //write response
}

fn main() {
    let port = "localhost:8080";
    let mut tcp_stream = TcpStream::connect(port).unwrap();
    tcp_stream.write_all("Hello server".as_bytes()).unwrap();
    let mut buffer = [0; 1024];

    loop {
        send_equation();
    }
}
