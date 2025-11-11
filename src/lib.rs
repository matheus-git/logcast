use std::sync::mpsc::{self, Sender};
use std::thread;
use std::net::TcpStream;
use std::io::Write;
use std::time::Duration;
use chrono::Local;

pub struct Logger {
    tx: Sender<String>,
}

impl Logger {
    pub fn new(address: &str) -> Self {
        let (tx, rx) = mpsc::channel::<String>();
        let addr = address.to_string();

        thread::spawn(move || {
            let mut stream = loop {
                match TcpStream::connect(&addr) {
                    Ok(s) => break s,
                    Err(e) => {
                        eprintln!("Error connecting to server: {}, retrying in 1s...", e);
                        thread::sleep(Duration::from_secs(1));
                    }
                }
            };

            while let Ok(msg) = rx.recv() {
                let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
                let formatted = format!("\x1b[90m{}\x1b[0m {}\n", timestamp, msg);

                if let Err(e) = stream.write_all(formatted.as_bytes()) {
                    eprintln!("Error sending log: {}, reconnecting...", e);
                    stream = loop {
                        match TcpStream::connect(&addr) {
                            Ok(s) => break s,
                            Err(e) => {
                                eprintln!("Error reconnecting to server: {}, retrying in 1s...", e);
                                thread::sleep(Duration::from_secs(1));
                            }
                        }
                    };
                    // Após reconectar, tenta enviar novamente
                    if let Err(e) = stream.write_all(formatted.as_bytes()) {
                        eprintln!("Failed to send log after reconnect: {}", e);
                    }
                }
            }
        });

        Self { tx }
    }

    pub fn log(&self, message: &str) {
        if let Err(e) = self.tx.send(message.to_string()) {
            eprintln!("Error sending message to queue: {}", e);
        }
    }
}
