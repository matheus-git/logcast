use log::Log;

use crate::Logger;

/// 
/// Inits Logger on given addr and sets it as the logger for log
///
pub fn init_on_addr(addr: &str) {
    let logger = Logger::new(addr);
    log::set_boxed_logger(Box::new(logger)).expect("Attempted to initialize logger when it was already iniitalized!");
    log::set_max_level(log::LevelFilter::Trace);
}

impl Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
         true // logging everything for now
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let formatted = format!(
            "{}:{} -- {}\n",
            record.level(),
            record.target(),
            record.args()
        ); // Copied from log's example
        self.log_raw(formatted);
    }
    fn flush(&self) {}
}
