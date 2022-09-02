use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

fn main() {
    let host = "worldtimeapi.org";
    let path = "/api/timezone/America/Argentina/Salta";
    let response = send_request(host, 80, path);
    println!("{} - Output is `{}`", host, response.unwrap());
}

fn send_request(host: &str, port: u16, path: &str) -> io::Result<String> {
    let mut socket = TcpStream::connect((host, port))?;
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);

    socket.set_nodelay(true)?;
    socket.write_all(request.as_bytes())?;
    socket.shutdown(Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response)?;

    Ok(response)
}
