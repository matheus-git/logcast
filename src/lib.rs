use std::io;
use std::net::UdpSocket;

pub fn send_udp(message: &str) -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let destino = "127.0.0.1:8080";
    socket.send_to(format!("{}\n", message).as_bytes(), destino)?;
    Ok(())
}
