# logcast

![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

A simple helper that sends logs over UDP, for programs without terminal output, such as TUIs.

## Usage

    cargo add logcast

I recommend creating a macro. Specify the address and port where the client will listen for logs. To view the logs, open another terminal and run a program that listens for UDP messages without exiting, such as ```ncat -ul --keep-open 8080```, as shown in the example below.

    use logcast::send_log;
    
    macro_rules! log {
        ($msg:expr) => {
            if let Err(e) = send_log($msg, "127.0.0.1:8080") {
                eprintln!("Error: {}", e);   
            }
        };
    }

    log!("Test");
    log!(&format!("{:?}", service));

Output:

    └─$ ncat -ul --keep-open 8080 
    [2025-11-10 20:55:04] Test
    [2025-11-10 20:55:04] Service { name: "cron.service", description: "Regular background program processing daemon", state: ServiceState { load: "loaded", active: "active", sub: "running", file: "enabled" } }

## 📝 License

This project is open-source under the MIT License.
