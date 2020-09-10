use std::time::Instant;
use flog::{log_str, flush, LogItem, log};
use std::fs::File;

fn log_single_thread(i: usize) {
    let start_time = Instant::now();
    for i in 0..i {
        let mut obj = LogItem::new();
        obj.char('[').u64(start_time.elapsed().as_nanos() as u64).str("] ").u64(i as u64).char('\n');
        log(obj);
    }
    flush();
}

fn main() {
    let guard = pprof::ProfilerGuard::new(100).unwrap();
    log_single_thread(100_000_000);
    if let Ok(report) = guard.report().build() {
        let file = File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
    };
}