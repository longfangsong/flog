#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::io::Write;

// todo: make it configurable
const BUFFER_SIZE: usize = 16384;

lazy_static! {
  static ref LOG_FILE: Arc<Mutex<File>> = Arc::new(Mutex::new(File::create("./log.log").unwrap()));
}

pub struct Collector {
    buffer: String,
    to_file: Arc<Mutex<File>>,
}

thread_local!(static COLLECTOR: RefCell<Collector> = RefCell::new(Collector {
    buffer: String::with_capacity(BUFFER_SIZE),
    to_file: LOG_FILE.clone(),
}));

pub fn log(content: &str) {
    COLLECTOR.with(move |collector| {
        let mut collector = collector.borrow_mut();
        collector.buffer.push_str(content);
        if collector.buffer.len() > BUFFER_SIZE {
            collector.to_file.lock().unwrap().write_all(collector.buffer.as_bytes()).unwrap();
            collector.buffer.clear();
        }
    });
}

pub fn flush() {
    COLLECTOR.with(move |collector| {
        let mut collector = collector.borrow_mut();
        collector.to_file.lock().unwrap().write_all(collector.buffer.as_bytes()).unwrap();
        collector.buffer.clear();
    });
}

#[cfg(test)]
mod tests {}
