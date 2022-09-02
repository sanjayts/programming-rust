use async_std::io;
use async_std::io::{ReadExt, WriteExt};
use async_std::net::TcpStream;
use core::time::Duration;
use std::net::Shutdown;
use std::thread;

#[async_std::main]
async fn main() {
    let url = "http://worldtimeapi.org/api/timezone/Europe/London";
    let client = surf::Client::new();
    let response = client.get(url).recv_string().await.unwrap();
    println!("Output is `{}`", response);
}
