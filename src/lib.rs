use std::io;
use std::net::UdpSocket;
use chrono::Local;

pub fn send_log(message: &str) -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let destino = "127.0.0.1:8080";

    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
    let formatted = format!("{} {}\n", timestamp, message);

    socket.send_to(formatted.as_bytes(), destino)?;
    Ok(())
}
