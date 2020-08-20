#[macro_use]
extern crate log;

use log::{Log, Metadata, Record};
use tokio::sync::mpsc::{channel, Sender};
use tokio::io;
use std::time::Instant;
use tokio::io::AsyncWriteExt;

const DUMP_SIZE: usize = 1_048_576;

pub struct Flog {
    tx: Sender<Vec<u8>>,
    start: Instant,
}

impl Flog {
    pub fn new() -> Flog {
        let (tx, mut rx) = channel::<Vec<u8>>(128);
        tokio::spawn(async move {
            let mut buffers = [Vec::with_capacity(4096), Vec::with_capacity(4096)];
            let mut current_buffer_index = 0;
            while let Some(mut item) = rx.recv().await {
                if buffers[current_buffer_index].len() + item.len() > DUMP_SIZE {
                    io::stdout().write_all(&buffers[current_buffer_index]).await.unwrap();
                    buffers[current_buffer_index].clear();
                    current_buffer_index = (current_buffer_index + 1) % 2;
                }
                buffers[current_buffer_index].append(&mut item);
            }
        });
        Flog { tx, start: Instant::now() }
    }
}

impl Log for Flog {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        let mut tx = self.tx.clone();
        let now = Instant::now();
        let duration = now.duration_since(self.start);
        let content = format!("{} {}\n", duration.as_nanos(), record.args());
        tokio::spawn(async move {
            tx.send(content.into_bytes()).await.unwrap();
        });
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::Flog;
    use log::LevelFilter;
    use tokio::runtime::Runtime;

    #[test]
    fn it_works() {
        let mut rt = Runtime::new().unwrap();
        rt.block_on(async {
            log::set_boxed_logger(Box::new(Flog::new())).unwrap();
            log::set_max_level(LevelFilter::Trace);
            for i in 0..1_0000_0000 {
                info!("{}", i);
            }
        });
    }
}
