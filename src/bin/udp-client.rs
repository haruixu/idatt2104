use std::io::prelude::*;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket_adr = "localhost:8080";
    let socket = UdpSocket::bind(socket_adr)?;

    let mut buf = [0; 1024];

    let (_, src) = socket.recv_from(&mut buf)?;
    let msg = std::str::from_utf8(&buf).expect("Should be valid utf8");

    println!("Server sent: {msg}");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.as_str().trim().as_bytes();
        socket.send_to(input, &src)?;

        let mut buf = [0; 10];

        let (amt, src) = socket.recv_from(&mut buf)?;

        let equation = std::str::from_utf8(&buf).expect("Should be valid utf8");

        println!("{equation}");
    }
    //Socket dropped after closure
}
