use std::fs::File;
use std::cell::RefCell;
use std::io::Write;
use std::thread;
use std::cmp::max;

// todo: make it configurable
const BUFFER_SIZE: usize = 16384;
const NORMAL_LOG_SIZE: usize = 64;

pub struct Collector {
    buffer: Vec<u8>,
    to_file: RefCell<File>,
}

thread_local!(static COLLECTOR: RefCell<Collector> = RefCell::new(Collector {
    buffer: Vec::with_capacity(BUFFER_SIZE + NORMAL_LOG_SIZE),
    to_file: RefCell::new(File::create(format!("./{:?}.log", thread::current().id())).unwrap()),
}));

pub struct LogItem {
    content: Vec<u8>,
}

impl LogItem {
    pub fn new() -> Self {
        LogItem {
            content: Vec::with_capacity(NORMAL_LOG_SIZE),
        }
    }

    pub fn u64(&mut self, mut n: u64) -> &mut Self {
        // max length of a u64 is 20:         18442240474082181120
        let mut current_cursor = self.content.len() + 20 - 1;
        self.content.extend_from_slice(b"                    ");
        while n != 0 {
            self.content[current_cursor] = b'0' + (n % 10) as u8;
            n /= 10;
            current_cursor -= 1;
        }
        self
    }

    pub fn str(&mut self, s: &str) -> &mut Self {
        self.content.extend_from_slice(s.as_bytes());
        self
    }

    pub fn char(&mut self, ch: char) -> &mut Self {
        self.content.push(ch as u8);
        self
    }
}

impl Default for LogItem {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for LogItem {
    fn from(mut s: String) -> Self {
        s.reserve(max(NORMAL_LOG_SIZE as isize - s.len() as isize, 0) as usize);
        Self {
            content: s.into_bytes()
        }
    }
}

pub fn log(item: LogItem) {
    COLLECTOR.with(move |collector| {
        let mut collector = collector.borrow_mut();
        collector.buffer.extend_from_slice(&item.content);
        if collector.buffer.len() > BUFFER_SIZE {
            collector.to_file.borrow_mut().write_all(&collector.buffer).unwrap();
            collector.buffer.clear();
        }
    });
}

pub fn log_str(content: &str) {
    COLLECTOR.with(move |collector| {
        let mut collector = collector.borrow_mut();
        collector.buffer.extend_from_slice(content.as_bytes());
        if collector.buffer.len() > BUFFER_SIZE {
            collector.to_file.borrow_mut().write_all(&collector.buffer).unwrap();
            collector.buffer.clear();
        }
    });
}

pub fn flush() {
    COLLECTOR.with(move |collector| {
        let mut collector = collector.borrow_mut();
        collector.to_file.borrow_mut().write_all(&collector.buffer).unwrap();
        collector.buffer.clear();
    });
}
