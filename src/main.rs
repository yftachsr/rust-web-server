use std::{io::{BufRead, BufReader, Read}, net::{TcpListener, TcpStream}};

fn main() {
    let listener = TcpListener::bind("localhost:8080").expect("Unable to bind address");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { 
                println!("Connection established");
                handle_connection(stream); 
            }
            Err(e) => { eprintln!("{}", e); }
        }
    }
}

fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
                                            .lines()
                                            .map(|r| r.unwrap())
                                            .take_while(|line| !line.is_empty()).collect();
    stream.bytes();
    println!("Request: {:#?}", http_request);
}
