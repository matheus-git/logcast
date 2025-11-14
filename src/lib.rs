//! # logcast
//!
//! ![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
//!
//! A helper that sends logs over TCP, for programs without terminal output, such as TUIs.
//!
//! ## Example
//!
//! ![logcast](https://raw.githubusercontent.com/matheus-git/logcast/main/screenshots/logcast.gif)
//!
//! ## Usage
//!
//! ### Integrate with the [log](https://docs.rs/log/latest/log/index.html) crate
//! See `examples/log.rs` for an example of integration with the [log](https://docs.rs/log/latest/log/index.html) crate.
//! 
//! ```ignore
//! // src/main.rs
//! use logcast::init_on_addr;
//! 
//! init_on_addr("127.0.0.1:8080");
//! log::info!("The logger seems to work");
//! ```
//! 
//! Example output:
//! 
//! ```shell
//! $ ncat -l --keep-open 8080
//! INFO:systemd_manager_tui::terminal::components::details -- Test
//! INFO:systemd_manager_tui::terminal::components::details -- Service { name: "bluetooth.service", description: "Bluetooth service", state: ServiceState { load: "loaded", active: "active", sub: "running", file: "enabled" } }
//! ```
//! 
//! ---
//! 
//! Another option is to create your own macro.
//!
//! ### Create Macro
//! 
//! ```ignore
//! // src/macros.rs
//! macro_rules! log {
//!     ($($arg:tt)*) => {{
//!         crate::LOGGER.log(&format!($($arg)*));
//!     }};
//! }
//! ```
//! 
//! ###  Make the macro available globally and create the LOGGER
//! 
//! ```ignore
//! // src/main.rs
//! #[macro_use]
//! mod macros;
//! 
//! use std::sync::LazyLock;
//! use logcast::Logger;
//! 
//! pub static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new("127.0.0.1:8080"));
//! ```
//! 
//! ### Use macro with log!
//! 
//! ```ignore
//! log!("Test");
//! log!("{:?}", service);
//! ```
//! ### Output
//! To view the logs, open another terminal and run a program that listens for TCP connections, such as ```ncat -l --keep-open 8080```, as shown in the example below.
//!```shell
//!     └─$ ncat -l --keep-open 8080
//!     [2025-11-10 20:55:04] Test
//!     [2025-11-10 20:55:04] Service { name: "cron.service", description: "Regular background program processing daemon", state: ServiceState { load: "loaded", active: "active", sub: "running", file: "enabled" } }
//!```
//! ## 📝 License
//!
//! This project is open-source under the MIT License.

mod log;
pub use log::init_on_addr;

use chrono::Local;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

pub struct Logger {
    tx: Sender<String>,
}

impl Logger {
    pub fn new(address: &str) -> Self {
        let (tx, rx) = mpsc::channel::<String>();
        let addr = address.to_string();

        thread::spawn(move || {
            let mut stream = tcp_connect(&addr);
            while let Ok(msg) = rx.recv() {
                send_str_retry(&mut stream, &addr, &msg);
            }
        });

        Self { tx }
    }

    ///
    /// Logs message formatted with a timestamp
    ///
    pub fn log(&self, message: &str) {
        let formatted = format_log(message);
        self.log_raw(formatted);
    }

    ///
    /// Logs the message without formatting
    ///
    pub fn log_raw(&self, message: impl ToString) {
        if let Err(e) = self.tx.send(message.to_string()) {
            eprintln!("Error sending message to queue: {e}");
        }
    }
}

///
/// Sends the message using given TcpStream. Attemtpts to reconnect and tries again if fails first time
///
fn send_str_retry(stream: &mut TcpStream, addr: &str, message: &str) {
    if let Err(e) = stream.write_all(message.as_bytes()) {
        eprintln!("Error sending log: {e}, reconnecting...");
        *stream = tcp_connect(addr);
        if let Err(e) = stream.write_all(message.as_bytes()) {
            eprintln!("Failed to send log after reconnect: {e}");
        }
    }
}
///
/// Attemtpts to connect to addr once a second until succeeds. Returns TcpStream on success
///
fn tcp_connect(addr: &str) -> TcpStream {
    loop {
        match TcpStream::connect(addr) {
            Ok(s) => break s,
            Err(e) => {
                eprintln!("Error connecting to server: {e}, retrying in 1s...");
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

///
/// Formats the log message (adds timestamps and escapes)
///
fn format_log(message: &str) -> String {
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
    format!("\x1b[90m{timestamp}\x1b[0m {message}\n")
}
