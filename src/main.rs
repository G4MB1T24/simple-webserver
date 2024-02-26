use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};
fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    let mut uri = String::new(); 
    let mut response: String = String::new(); 
    for mut stream in listener.incoming().flatten() {
        let mut rdr = BufReader::new(&mut stream);
        loop {
            let mut lmfao = String::new();
            rdr.read_line(&mut lmfao).unwrap();
            if lmfao.trim().is_empty() {
                break;
            }
            let request_parts: Vec<&str> = lmfao.split_whitespace().collect();
            if request_parts.len() >= 2 && request_parts[0] == "GET" {
                uri = request_parts[1].to_string(); // Store the URI in the mutable string
                println!("Requested URI: {}", uri);
            }
        }
        response = format!("HTTP/1.1 200 OK\r\n\r\n{}", uri); // Interpolate uri using {}
        stream.write_all(response.as_bytes()).unwrap();
    }
}
