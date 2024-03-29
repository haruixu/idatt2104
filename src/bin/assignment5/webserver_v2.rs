use idatt2104::ThreadPool;
use std::{
    fs::{self, File},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process::{Command, Output},
};

fn main() {
    let port = "127.0.0.1:3000";
    let listener = TcpListener::bind(port).unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }
    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(&mut stream);

    //Get request body
    let mut request_line = String::new();
    let _ = reader.read_line(&mut request_line);

    let mut name = String::new();
    loop {
        let r = reader.read_line(&mut name).unwrap();
        if r < 3 {
            //detect empty line
            break;
        }
    }
    let mut size = 0;
    let linesplit = name.split("\n");
    for l in linesplit {
        if l.starts_with("Content-Length") {
            let sizeplit = l.split(":");
            for s in sizeplit {
                if !(s.starts_with("Content-Length")) {
                    size = s.trim().parse::<usize>().unwrap(); //Get Content-Length
                }
            }
        }
    }
    let mut buffer = vec![0; size]; //New Vector with size of Content
    reader.read_exact(&mut buffer).unwrap(); //Get the Body Content

    //Handle request line
    let (status_line, content) = match &request_line[..] {
        "POST /compile HTTP/1.1\r\n" => ("HTTP/1.1 201 CREATED", run_code_virtualized(buffer)),
        _ => (
            "HTTP/1.1 404 NOT FOUND",
            fs::read_to_string("data/404.html").unwrap(),
        ),
    };

    //Browser sends OPTIONS request + POST request, instead of just POST as postman does
    //OPTIONS request looks for matching Access-Control-Allow-Origin header in response.
    //      ^^^According to stackoverflow, but i didnt need any allow-access-control-methods header
    //         What I did need was Access-Control-Allow-Credentials: true for cookies
    //Need the response to be 2xx, have the origin (frontend url) allowed, and used http request
    //methods allowed for the backend to throw a CORS ruckus.
    let header: &str =
        "Access-Control-Allow-Origin: http://localhost:5173\r\nAccess-Control-Allow-Methods: POST, OPTIONS\r\nAccess-Control-Allow-Credentials: true";
    println!("{content}");
    //Format response
    let response: String = format!("{status_line}\r\n{header}\r\n\r\n{content}");
    //Here, we send the data back to the client
    stream.write_all(response.as_bytes()).unwrap();
    println!("Sent response");
    stream.flush().unwrap();
}

fn run_code_virtualized(body: Vec<u8>) -> String {
    //Read code
    let code: String = String::from_utf8(body).expect("Should parse to string");
    let file_path: &str = "src/bin/temp.rs";

    //Write to file
    let mut file: File = File::create(file_path).expect("Should create file");
    file.write_all(code.as_bytes())
        .expect("Should write to file");

    let _: Output = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg("rust-image")
        .arg(".")
        .output()
        .expect("Couldn't compile");

    println!("Docker build finished");
    let docker_run_child: Output = Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("rust-image")
        .output()
        .expect("Couldn't compile");

    println!("Docker run finished");

    //Match result
    let mut result: String = String::new();

    if !docker_run_child.stdout.is_empty() {
        if let Ok(output) = String::from_utf8(docker_run_child.stdout) {
            result = output;
        }
    } else {
        if let Ok(output) = String::from_utf8(docker_run_child.stderr) {
            result = output;
        }
    }
    result
}

fn run_code(body: Vec<u8>) -> String {
    //Read code
    let code: String = String::from_utf8(body).expect("Should parse to string");
    let file_path: &str = "src/bin/temp.rs";

    //Write to file
    let mut file: File = File::create(file_path).expect("Should create file");
    file.write_all(code.as_bytes())
        .expect("Should write to file");

    let cargo_child: Output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("temp")
        .output()
        .expect("Couldn't compile");

    //Match result
    let mut result: String = String::new();

    if !cargo_child.stdout.is_empty() {
        if let Ok(output) = String::from_utf8(cargo_child.stdout) {
            result = output;
        }
    } else {
        if let Ok(output) = String::from_utf8(cargo_child.stderr) {
            result = output;
        }
    }
    result
}
