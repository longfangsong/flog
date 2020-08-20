#[macro_use]
extern crate log;

use log::{Log, Metadata, Record};
use tokio::sync::mpsc::{channel, Sender};
use tokio::io;
use std::mem;
use std::time::Instant;
use tokio::io::AsyncWriteExt;

pub struct Flog {
    tx: Sender<String>,
    start: Instant,
}

impl Flog {
    pub fn new() -> Flog {
        let (tx, mut rx) = channel::<String>(128);
        tokio::spawn(async move {
            let mut buffer = String::with_capacity(4096);
            while let Some(item) = rx.recv().await {
                if buffer.len() + item.len() > 4096 {
                    let to_write =
                        mem::replace(&mut buffer, String::with_capacity(4096))
                            .into_bytes();
                    tokio::spawn(async move { io::stdout().write_all(&to_write).await.unwrap() });
                }
                buffer.push_str(&item);
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
            tx.send(content).await.unwrap();
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
