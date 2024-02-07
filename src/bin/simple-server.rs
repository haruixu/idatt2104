use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_request(mut stream: TcpStream) {
    let welcome_msg = "\nWelcome to the calcuator. Please enter the equation in this format: <num><operator><num> followed by an ENTER-click\n 
        Example: 3+2\n 
        Note the lack of spaces, that you only can use +/- and that you only can use two numbers";
    stream.write_all(welcome_msg.as_bytes()).unwrap();
    stream.flush().unwrap();

    loop {
        let mut buffer = [0; 1024];

        //todo blockke på input
        let size = stream.read(&mut buffer).unwrap();
        let equation = String::from_utf8_lossy(&buffer[..size]).trim().to_owned();
        println!("Client sent: {equation}");

        let operator_option = equation.chars().find(|&c| c == '+' || c == '-');

        if let Some(op) = operator_option {
            let numbers = equation
                .split(op)
                .map(|string| string.trim())
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            if numbers.len() != 2 {
                write_error_msg(&stream, "Please input two numbers and a single operator");
                continue;
            }

            let (num1, num2) = (numbers[0], numbers[1]);

            match op {
                '+' => {
                    let sum = num1 + num2;
                    write_error_msg(&stream, format!("{num1}+{num2}={sum}").as_str());
                }
                '-' => {
                    let diff = num1 - num2;
                    write_error_msg(&stream, format!("{num1}-{num2}={diff}").as_str());
                }
                _ => panic!(),
            };
        } else {
            write_error_msg(&stream, " Only + and - can be used as operator");
        }
    }
}

fn write_error_msg(mut stream: &TcpStream, error_msg: &str) {
    stream.write_all(error_msg.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let s = "40".parse().unwrap();
    assert_eq!(40, s);

    let port = "localhost:8080";
    let listener = TcpListener::bind(port).unwrap();

    for stream in listener.incoming() {
        handle_request(stream.unwrap());
    }
}
