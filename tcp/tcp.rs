// import necessary modules from rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    //this is a buffer to read data from the client
    let mut buffer = [0; 1024];

    //reads from stream and stores it in buffer variable &
    //expects for error
    stream.read(&mut buffer).expect("Failed to red from client");

    //this line converts the data in the buffer into a UTF-8 encoded string.
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    let response = "Hello world".as_bytes();
    stream.write(response).expect("failed to handle response");
}

//entry point
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind to address");
    println!("Server Listening on localhost:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let _ = std::thread::spawn(|| handle_client(stream));
            }

            //std error print
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e)
            }
        }
    }
}
