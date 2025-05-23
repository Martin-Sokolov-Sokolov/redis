use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 512];

    loop {
        let size = stream.read(&mut buf).unwrap();

        if size == 0 {
            break;
        }
        
        stream.write_all(b"+PONG\r\n").unwrap();
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
