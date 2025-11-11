use std::sync::mpsc::{self, Sender};
use std::thread;
use std::net::TcpStream;
use std::io::Write;
use chrono::Local;

pub struct Logger {
    tx: Sender<String>,
}

impl Logger {
    pub fn new(address: &str) -> Self {
        let (tx, rx) = mpsc::channel::<String>();
        let addr = address.to_string();

        thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                match TcpStream::connect(&addr) {
                    Ok(mut stream) => {
                        let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
                        let formatted = format!("\x1b[90m{}\x1b[0m {}\n", timestamp, msg);
                        if let Err(e) = stream.write_all(formatted.as_bytes()) {
                            eprintln!("Error sending log: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Error connecting to server: {}", e),
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
