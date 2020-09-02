use std::fs::File;
use std::cell::RefCell;
use std::io::Write;
use std::thread;

// todo: make it configurable
const BUFFER_SIZE: usize = 16384;

pub struct Collector {
    buffer: String,
    to_file: RefCell<File>,
}

thread_local!(static COLLECTOR: RefCell<Collector> = RefCell::new(Collector {
    buffer: String::with_capacity(BUFFER_SIZE),
    to_file: RefCell::new(File::create(format!("./{:?}.log", thread::current().id())).unwrap()),
}));

pub fn log(content: &str) {
    COLLECTOR.with(move |collector| {
        let mut collector = collector.borrow_mut();
        collector.buffer.push_str(content);
        if collector.buffer.len() > BUFFER_SIZE {
            collector.to_file.borrow_mut().write_all(collector.buffer.as_bytes()).unwrap();
            collector.buffer.clear();
        }
    });
}

pub fn flush() {
    COLLECTOR.with(move |collector| {
        let mut collector = collector.borrow_mut();
        collector.to_file.borrow_mut().write_all(collector.buffer.as_bytes()).unwrap();
        collector.buffer.clear();
    });
}
