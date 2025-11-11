use std::io::{self, Write};
use std::net::TcpStream;
use chrono::Local;

pub fn send_log(message: &str, address: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(address)?;

    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
    let formatted = format!("\x1b[90m{}\x1b[0m {}\n", timestamp, message);

    stream.write_all(formatted.as_bytes())?;
    Ok(())
}
