use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::{Instant};
use flog::{log, flush};
use std::fs::File;
use std::io::Write;
use std::f64::consts::PI;


fn fib(i: usize) -> usize {
    if i < 2 { 1 } else { fib(i - 1) + fib(i - 2) }
}

pub fn heavy_cpu(i: usize) -> f64 {
    let data = [
        i as f64,
        (i as f64).sin(),
        (i as f64).cos(),
        (i as f64).tan(),
        (i as f64).log(2.0),
        (i as f64).log(10.0),
        (i as f64).exp(),
        (i as f64).acos(),
        (i as f64).asin(),
        (i as f64).powf(PI),
        (i as f64).powf(PI * 2.0),
        (fib(i % 20) as f64).sin().acos().tan(),
    ];
    data.iter().sum()
}

fn pure(i: usize) {
    for i in 0..i {
        heavy_cpu(i);
    }
}

#[allow(dead_code)]
fn use_print(i: usize) {
    let mut f = File::create("log.log").unwrap();
    let start_time = Instant::now();
    for i in 0..i {
        writeln!(f, "[{:?}] {}", start_time.elapsed(), heavy_cpu(i)).unwrap();
    }
}

fn use_log(i: usize) {
    let start_time = Instant::now();
    for i in 0..i {
        log(&format!("[{:?}] {}\n", start_time.elapsed(), heavy_cpu(i)))
    }
    flush();
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("log");
    for i in [1024, 2048, 4096].iter() {
        // Warning: this is usually too slow to run.
        // group.bench_with_input(BenchmarkId::new("print", i), i,
        //                        |b, i| b.iter(|| print_single_thread(*i)));
        group.bench_with_input(BenchmarkId::new("highcpu", i), i,
                               |b, i| b.iter(|| pure(*i)));
        group.bench_with_input(BenchmarkId::new("highcpu_log", i), i,
                               |b, i| b.iter(|| use_log(*i)));
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
