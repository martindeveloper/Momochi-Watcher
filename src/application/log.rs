extern crate simplelog;

use self::simplelog::{Config, TermLogger, LogLevelFilter};
use std::io::stdout;
use std::io::stderr;
use std::io::Write;

pub fn init_logger() {
    let logger = TermLogger::init(LogLevelFilter::Info, Config::default()).unwrap();
}

pub fn flush_logger() {
    let out_flush_result = stdout().flush();

    if out_flush_result.is_err() {
        error!("Failed to flush stdout");
    }

    let err_flush_result = stderr().flush();

    if err_flush_result.is_err() {
        error!("Failed to flush stderr");
    }
}
