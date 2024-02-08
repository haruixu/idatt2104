use std::{
    io::{self, prelude::*, Write},
    net::TcpStream,
};

fn main() {
    let port = "localhost:8080";
    //Fails if server not running
    let mut stream = TcpStream::connect(port).unwrap();

    let mut buffer = [0; 1024];

    let size = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..size]);
    println!("Server says: {message}");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        //Fails if server CTRL-C
        stream.write_all(input.as_bytes()).unwrap();
        stream.flush().unwrap();

        let size = stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer[..size]);
        println!("Server says: {message}");
    }
}
