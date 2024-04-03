use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        handle_connection(stream?)
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
                        .lines()
                        .map(|result| result.unwrap())
                        .take_while(|line| !line.is_empty())
                        .collect();
    
    let response = "HTTP/1.1 200 Ok\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}