use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::{Instant, Duration};
use flog::{log, flush};
use std::fs::File;
use std::io::Write;

fn print_single_thread(i: usize) {
    let mut f = File::create("log.log").unwrap();
    let start_time = Instant::now();
    for i in 0..i {
        writeln!(f, "[{:?}] {}", start_time.elapsed(), i).unwrap();
    }
}

fn log_single_thread(i: usize) {
    let start_time = Instant::now();
    for i in 0..i {
        log(&format!("[{:?}] {}\n", start_time.elapsed(), i))
    }
    flush();
}

fn bench_log(c: &mut Criterion) {
    let mut group = c.benchmark_group("log");
    for i in [1024, 2048, 8192].iter() {
        // Warning: this is usually too slow to run.
        // group.bench_with_input(BenchmarkId::new("print", i), i,
        //                        |b, i| b.iter(|| print_single_thread(*i)));
        group.bench_with_input(BenchmarkId::new("log", i), i,
                               |b, i| b.iter(|| log_single_thread(*i)));
    }
    group.finish();
}

criterion_group!(benches, bench_log);
criterion_main!(benches);
