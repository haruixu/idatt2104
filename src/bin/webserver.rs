use idatt2104::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use build_html::*;

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
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = http_request.first().unwrap();

    let (status_line, content) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", create_content(&http_request)),
        _ => (
            "HTTP/1.1 404 NOT FOUND",
            fs::read_to_string("data/404.html").unwrap(),
        ),
    };

    let response: String = format!("{status_line}\r\n\r\n{content}");
    //Here, we send the data back to the client
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn create_content(http_request: &Vec<String>) -> String {
    HtmlPage::new()
        .with_title("Din noob")
        .with_header(1, "Du kalrte det!!")
        .with_paragraph("dfklsjldfs")
        .with_container(
            http_request
                .iter()
                .skip(1)
                .fold(Container::new(ContainerType::UnorderedList), |a, n| {
                    a.with_paragraph(n)
                }),
        )
        .to_html_string()
}
