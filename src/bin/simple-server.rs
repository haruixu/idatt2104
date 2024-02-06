use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let welcome_msg = "Welcome to the calcuator. Please enter the equation in this format: <num><operator><num> followed by an ENTER-click\n 
        Example: 3+2\n 
        Note the lack of spaces, that you only can use +/- and only use two numbers";
    stream.write_all(welcome_msg.as_bytes()).unwrap();

    loop {
        let equation = stream.read(&mut buffer).unwrap();

        //iteratore over chars() - find +/-
        //use if let Some(num) = operator, split at operator index, then parse the two numbers
    }
}

fn main() {
    let port = "localhost:8080";
    let listener = TcpListener::bind(port).unwrap();

    for stream in listener.incoming() {
        handle_request(stream.unwrap());
    }
}
