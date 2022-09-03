use async_std::io;
use async_std::io::{ReadExt, WriteExt};
use async_std::net::TcpStream;

use std::net::Shutdown;

#[async_std::main]
async fn main() {
    let host = "worldtimeapi.org";
    let path = "/api/timezone/Europe/London";
    let response = send_request(host, 80, path).await;
    println!("Output is `{}`", response.unwrap());
}

async fn send_request(host: &str, port: u16, path: &str) -> io::Result<String> {
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    let mut socket = TcpStream::connect((host, port)).await?;
    socket.set_nodelay(true)?;

    socket.write_all(request.as_bytes()).await?;
    // thread::sleep(Duration::from_secs(1));
    socket.shutdown(Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;

    Ok(response)
}
