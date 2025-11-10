use std::io;
use std::net::UdpSocket;
use chrono::Local;

pub fn send_log(message: &str, address: &str) -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
    let formatted = format!("\x1b[90m{}\x1b[0m {}\n", timestamp, message);

    socket.send_to(formatted.as_bytes(), address)?;
    Ok(())
}
