use std::time::Instant;
use flog::{log, flush};
use std::fs::File;

fn log_single_thread(i: usize) {
    let start_time = Instant::now();
    for i in 0..i {
        log(&format!("[{:?}] {}\n", start_time.elapsed(), i))
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