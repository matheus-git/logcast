# logcast

![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

A simple helper that sends logs over TCP, for programs without terminal output, such as TUIs.

## Example

![logcast](https://raw.githubusercontent.com/matheus-git/logcast/main/screenshots/logcast.gif)

## Usage

See `examples/log.rs` for an example of integration with the [log](https://docs.rs/log/latest/log/index.html) crate.

### Add logcast

    cargo add logcast

### Create Macro
This code defines a global, thread-safe TCP logger using a singleton (LOGGER) initialized lazily.

```rust
// src/macros/log/mod.rs
use logcast::Logger;
use std::sync::LazyLock;

pub static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new("127.0.0.1:8080"));

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::LOGGER.log(&format!($($arg)*));
    }};
}
```

### Import LOGGER
Import LOGGER in main.rs to allow the macro to access it from any module.

```rust
// src/main.rs
use macros::log::LOGGER;
```

### Import macro

```rust
use crate::log;

log!("Test");
log!("{:?}", service);
// or
crate::log!("Test");
crate::log!("{:?}", service);
```

### Output
To view the logs, open another terminal and run a program that listens for TCP connections, such as ```ncat -l --keep-open 8080```, as shown in the example below.

```shell
└─$ ncat -l --keep-open 8080 
[2025-11-10 20:55:04] Test
[2025-11-10 20:55:04] Service { name: "cron.service", description: "Regular background program processing daemon", state: ServiceState { load: "loaded", active: "active", sub: "running", file: "enabled" } }
```

## 📝 License

This project is open-source under the MIT License.
