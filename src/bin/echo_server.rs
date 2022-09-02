use std::net::TcpListener;
use std::{io, thread};

/// Run the echo server. This can be easily tested using `socat` as shown below:
/// >> `socat - TCP4:localhost:8080`
fn main() {
    run_server("127.0.0.1:8080").expect("Failed to start server");
}

fn run_server(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    loop {
        let (mut stream, client_addr) = listener.accept()?;
        println!("Accepted connection from client {}", client_addr);
        let mut writer = stream.try_clone()?;

        thread::spawn(move || {
            let bytes_copied = io::copy(&mut stream, &mut writer).expect("Error in client thread");
            println!(
                "Transferred total {} bytes back to the client",
                bytes_copied
            );
        });
    }
}
