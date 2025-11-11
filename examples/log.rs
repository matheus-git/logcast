/*!
Using integration with [log](https://docs.rs/log/latest/log/index.html)
How to run:
1) Open another terminal and run a program that listens for TCP connections on port 8080, such as ```ncat -l --keep-open 8080```
2) Run 
```shell
$ cargo run --example log
```
*/
use std::time::Duration;

use logcast::init_on_addr;
fn main() {
    init_on_addr("127.0.0.1:8080");
    log::info!("The logger seems to work");
    std::thread::sleep(Duration::from_secs(1)); // If the main thread finishes before the logs
    // are sent the logs cannot be delivered
}
