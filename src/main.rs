use std::net::UdpSocket;
use std::io;

fn main() -> io::Result<()> {
    // Cria o socket local (0.0.0.0:0 usa uma porta aleatória)
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    
    // Endereço do servidor UDP (onde você quer enviar)
    let destino = "127.0.0.1:8080";

    // Mensagem que será enviada
    let mensagem = "Volta aaakkkk:w!\n";

    // Envia os bytes da string
    socket.send_to(mensagem.as_bytes(), destino)?;

    println!("📤 Enviado para {}: {}\n", destino, mensagem);

    Ok(())
}
