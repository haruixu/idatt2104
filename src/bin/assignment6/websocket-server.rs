use idatt2104::ThreadPool;
use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
};

struct Websocket {
    stream: TcpStream,
    magic_string: String,
}

impl Websocket {
    fn new(stream: TcpStream) -> Self {
        let magic_string = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11".into();
        Websocket {
            stream,
            magic_string,
        }
    }

    fn handshake(&self) -> Result<(), std::io::Error> {
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

        let key = headers.get("Sec-WebSocket-Key".into());
        dbg!(key);

        //let Sec-WebSocket-Accept: &str = Base64encode(sha1(key + self.magic_string));

        //**handshake
        // X Må reade request , Lese upgrade feltet
        //Oppgradere til websocket - dekode keyen
        //
        //**Meldinger
        //Få klient til å encode melding
        //Demaskere data i body
        Ok(())
    }
}
fn handle_request(stream: TcpStream) {
    let ws: Websocket = Websocket::new(stream.try_clone().unwrap());
    match ws.handshake() {
        Ok(()) => println!("Connection succeeded"),
        Err(err) => println!("Connection failed: {}", err),
    }
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
    fn base() {
        assert_eq!(1, 1);
    }
}
