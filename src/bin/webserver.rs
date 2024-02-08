use idatt2104::ThreadPool;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use typed_html::dom::DOMTree;
use typed_html::{html, text};

fn main() {
    //Alternative: 127.0.0.1:8080
    let port = "localhost:8080";
    let listener = TcpListener::bind(port).unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }
    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = http_request.first().unwrap();
    println!("{request_line}");

    //Shitty ass code, because the same page is returned as long as the request is valid
    let status_line = "HTTP/1.1 200 OK";

    let page: DOMTree<String> = html!(
    <html>
        <head>
            <title>"mai title"</title>
        </head>
        <body>
            <h1>"Yes, kom deg vekk herifra"</h1>
            <ul>
                { http_request
                    .iter()
                    .skip(1)
                    .map(|line| html!(
                        <li> { text!("{}", line) }</li>
                    ))
                }
            </ul>
        </body>
    </html>
    );

    let page_str = page.to_string();
    println!("{status_line}\r\n{page_str}");

    //let response = format!("{status_line}\r\n\r\n");
    let response = "HTTP/1.1 200 OK\r\n";

    //Here, we send the data back to the client
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
