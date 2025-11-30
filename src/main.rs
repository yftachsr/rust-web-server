use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread, time::Duration};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("localhost:8080").expect("Unable to bind address");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { 
                println!("Connection established");
                pool.execute(|| {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("{}", e);
                    }
                });
            }
            Err(e) => { eprintln!("{}", e); }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error>{
    let buf_reader = BufReader::new(&stream);
    let request_header= buf_reader
                                            .lines()
                                            .next()
                                            .ok_or(std::io::Error::new(
                                                std::io::ErrorKind::InvalidData,
                                                "No request header"))??;

    let (status_line, html_page) = match &request_header[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "html/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(7));
            ("HTTP/1.1 200 OK", "html/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "html/404.html") 
    }; 
        
    let contents = fs::read_to_string(html_page)?;  
    let length = contents.len();

    let response = format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\r\n
        {contents}"
    );
    stream.write_all(response.as_bytes())?;

    Ok(())

}
