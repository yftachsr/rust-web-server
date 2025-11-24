use std::{io::Read, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("localhost:8080").expect("Unable to bind address");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { 
                println!("Connection established");
                println!("{:?}", stream.bytes());
                //handle_connection(stream); 
            }
            Err(e) => { eprintln!("{}", e); }
        }
    }
}
