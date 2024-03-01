use idatt2104::ThreadPool;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};

mod websocket_server {
    use crate::add_header;
    use crate::create_response;
    use base64::prelude::*;
    use sha1::{Digest, Sha1};
    use std::{
        collections::BTreeMap,
        io::{BufRead, BufReader, Read, Write},
        net::TcpStream,
    };

    pub struct Websocket {
        stream: TcpStream,
        magic_string: String,
    }

    // Q: Burde bruke tokio for asio??
    impl Websocket {
        pub fn new(stream: TcpStream) -> Self {
            let magic_string = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11".into();
            Websocket {
                stream,
                magic_string,
            }
        }

        pub fn handshake(&mut self) -> Result<(), std::io::Error> {
            // Parse request
            let reader = BufReader::new(self.stream.try_clone().unwrap());

            let mut request_lines = reader.lines();

            let request_line = request_lines.next().unwrap().unwrap();
            let headers: BTreeMap<String, String> = request_lines
                .take_while(|line| !line.as_ref().unwrap().is_empty())
                .map(|header| {
                    let header = header.unwrap();
                    let (key, value) = header.split_once(": ").unwrap();
                    (key.to_string(), value.to_string())
                })
                .collect();

            println!("reqlien:{request_line}");
            println!("headers: {:#?}", headers);

            // Create response hash
            let key: String = headers.get("Sec-WebSocket-Key".into()).unwrap().clone();
            dbg!(&key);

            let hash: String = self.generate_hash(key);

            // Create response
            let response_line: &str = "HTTP/1.1 101 Switching Protocols";
            let mut headers: Vec<String> = Vec::new();
            add_header(&mut headers, "Upgrade: websocket");
            add_header(&mut headers, "Connection: Upgrade");
            add_header(&mut headers, "Upgrade: websocet");
            add_header(
                &mut headers,
                format!("Sec-WebSocket-Accept: {hash}").as_str(),
            );
            add_header(&mut headers, "Sec-WebSocket-Protocol: chat");

            let response: String = create_response(response_line, headers, None);
            dbg!(&response);

            // Write response
            self.stream.write_all(response.as_bytes()).unwrap();
            self.stream.flush().unwrap();

            Ok(())
        }

        fn generate_hash(&self, key: String) -> String {
            let mut hasher = Sha1::new();
            // Value to be hashed = key + magic string
            let value: String = format!("{key}{}", self.magic_string);
            // Hash the value
            hasher.update(value.as_bytes());
            let hash = hasher.finalize();
            let encoded = BASE64_STANDARD.encode(&hash[..]);

            encoded
        }

        pub fn message(&mut self) {
            loop {
                let mut buf = [0; 128];

                let _ = self.stream.read(&mut buf).unwrap();

                //todo - should I check for 0x81??
                let buf = [0x81, 0x83, 0xb4, 0xb5, 0x03, 0x2a, 0xdc, 0xd0, 0x6a];

                let length = buf[1] & 127;
                let mask_start = 2;
                let data_start = mask_start + 4; //Mask is 4 bytes
                let mut msg: String = String::new();

                for i in data_start..(data_start + length) {
                    let byte =
                        buf[i as usize] ^ buf[(mask_start + ((i - data_start) % 4)) as usize];
                    msg.push(char::from_u32(byte as u32).unwrap());
                }

                println!("{msg}");
                self.broadcast(msg);
            }
        }

        fn broadcast(&mut self, msg: String) {
            //vector av threads eller vector av connections
            //
            //for each thread => self.write_all(msg.as_bytes());
            unimplemented!()
        }
    }
}

fn add_header(headers: &mut Vec<String>, header: &str) {
    let crlf_header: String = format!("{}\r\n", header);
    headers.push(crlf_header);
}

fn create_response(response_line: &str, headers: Vec<String>, content: Option<String>) -> String {
    let mut response: String = format!("{response_line}\r\n");

    for header in &headers {
        response.push_str(header.as_str());
    }
    response.push_str("\r\n");

    if let Some(body) = content {
        response.push_str(body.as_str());
    }

    response
}

fn handle_request(stream: TcpStream) {
    let mut ws: websocket_server::Websocket =
        websocket_server::Websocket::new(stream.try_clone().unwrap());
    match ws.handshake() {
        Ok(()) => println!("Connection succeeded"),
        Err(err) => {
            println!("Connection failed: {}", err)
        }
    }

    ws.message();
}

fn main() -> std::io::Result<()> {
    let socket: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    let listener = TcpListener::bind(socket)?;
    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        pool.execute(|| handle_request(stream.unwrap()));
    }

    Ok(())
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encoding() {
        assert_eq!(1, 1);
    }
}
