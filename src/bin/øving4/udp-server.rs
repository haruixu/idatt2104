use idatt2104::ThreadPool;
use std::{
    io::Read,
    net::{SocketAddr, UdpSocket},
};

///Server is the listener, client is the sender. The main gist behind the udp-implementation is
///that in the std-library is to send a dynamic message, then receive them in static
///buffers, counting amt of bytes actually sent, and putting these bytes into a string or string
///slice
fn main() -> std::io::Result<()> {
    {
        let socket_adr = "localhost:8080";
        let socket = UdpSocket::bind("localhost:8081")?;

        let pool = ThreadPool::new(1);
        //Capturing message in byte slice
        let welcome_msg_bytes: &[u8] = b"Welcome to the calcuator. Please enter the equation in this format: <num><operator><num> followed by an ENTER-click\n 
        Example: 3+2\n 
        Note the lack of spaces, that you only can use +/- and that you only can use two numbers";

        socket.send_to(welcome_msg_bytes, socket_adr)?;

        pool.execute(|| respond(socket));
    }
    //Socket dropped after closure
    Ok(())
}

fn respond(socket: UdpSocket) {
    loop {
        //receive data
        let mut buf = [0; 25];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        let equation = std::str::from_utf8(&buf[..amt]).expect("Should be valid utf8");
        println!("Client sent: {equation}");

        let operator_option = equation.chars().find(|&c| c == '+' || c == '-');

        if let Some(op) = operator_option {
            let numbers = equation
                .split(op)
                .map(|string| string.trim())
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            if numbers.len() != 2 {
                write_response(
                    &socket,
                    src,
                    "Please input two numbers and a single operator",
                );
                continue;
            }

            let (num1, num2) = (numbers[0], numbers[1]);

            match op {
                '+' => {
                    let sum = num1 + num2;
                    write_response(&socket, src, format!("{num1}+{num2}={sum}").as_str());
                }
                '-' => {
                    let diff = num1 - num2;
                    write_response(&socket, src, format!("{num1}-{num2}={diff}").as_str());
                }
                _ => panic!(),
            };
        } else {
            write_response(&socket, src, "Only + and - can be used as operator");
        }
    }
}

fn write_response(socket: &UdpSocket, socket_adr: SocketAddr, response: &str) {
    let buf = response.as_bytes();
    socket.send_to(buf, socket_adr).unwrap();
}
