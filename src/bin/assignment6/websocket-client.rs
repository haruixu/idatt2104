use std::{
    io::Write,
    net::{Ipv4Addr, SocketAddrV4, TcpStream},
};

fn main() -> Result<(), std::io::Error> {
    //Strictly for mocking a websocket handshake
    {
        let to_socket_adr: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
        let mut stream: TcpStream = TcpStream::connect(to_socket_adr)?;

        let request_line: &str = "GET / HTTP/1.1\r\n";

        let mut headers: Vec<String> = Vec::new();
        add_header(&mut headers, "Host: ntnu.no");
        add_header(&mut headers, "Upgrade: websocket");
        add_header(&mut headers, "Connection: Upgrade");
        add_header(&mut headers, "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==");
        add_header(&mut headers, "Sec-WebSocket-Version: 13");

        let request = create_request(request_line, headers);

        println!("{request}");

        stream.write_all(request.as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}

fn add_header(headers: &mut Vec<String>, header: &str) {
    let crlf_header: String = format!("{}\r\n", header);
    headers.push(crlf_header);
}

fn create_request(request_line: &str, headers: Vec<String>) -> String {
    let mut request: String = String::from(request_line);

    for header in &headers {
        request.push_str(header.as_str());
    }
    request.push_str("\r\n");

    request
}
