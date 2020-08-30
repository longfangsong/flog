use std::fs::File;
use std::io::{Write, SeekFrom, Read, Seek};
use std::f64::consts::PI;
use std::time::Instant;
use flog::{log, flush};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn fib(i: u64) -> u64 {
    if i < 2 { 1 } else { fib(i - 1) + fib(i - 2) }
}

fn prepare() {
    let mut file = File::create("./input.bin").unwrap();
    for i in 0..32768u64 {
        file.write_all(&i.to_le_bytes()).unwrap();
    }
    File::create("./output.txt").unwrap();
}

pub fn heavy_cpu(i: u64) -> f64 {
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
        (fib(i % 30) as f64).sin().acos().tan(),
    ];
    data.iter().sum()
}

fn mixed(from: &mut File, to: &mut File, i: usize) {
    let mut buffer = [0u8; 8];
    from.seek(SeekFrom::Start(i as u64 * 8)).unwrap();
    from.read_exact(&mut buffer).unwrap();
    let n = u64::from_le_bytes(buffer);
    write!(to, "result is: {}\n", heavy_cpu(n)).unwrap();
}

fn pure(i: usize) {
    let mut from = File::open("./input.bin").unwrap();
    let mut to = File::create("./output.txt").unwrap();
    for i in 0..i {
        mixed(&mut from, &mut to, i);
    }
}

fn use_print(i: usize) {
    let mut from = File::open("./input.bin").unwrap();
    let mut to = File::create("./output.txt").unwrap();
    let mut f = File::create("log.log").unwrap();
    let start_time = Instant::now();
    for i in 0..i {
        writeln!(f, "[{:?}] {} start", start_time.elapsed(), i).unwrap();
        mixed(&mut from, &mut to, i);
        writeln!(f, "[{:?}] {} end", start_time.elapsed(), i).unwrap();
    }
}

fn use_log(i: usize) {
    let mut from = File::open("./input.bin").unwrap();
    let mut to = File::create("./output.txt").unwrap();
    let start_time = Instant::now();
    for i in 0..i {
        log(&format!("[{:?}] {} start", start_time.elapsed(), i));
        mixed(&mut from, &mut to, i);
        log(&format!("[{:?}] {} end", start_time.elapsed(), i));
    }
    flush();
}


fn bench(c: &mut Criterion) {
    prepare();
    let mut group = c.benchmark_group("log");
    for i in [1024, 2048, 4096, 8192].iter() {
        group.bench_with_input(BenchmarkId::new("mixed", i), i,
                               |b, i| b.iter(|| pure(*i)));
        group.bench_with_input(BenchmarkId::new("mixed_print", i), i,
                               |b, i| b.iter(|| use_print(*i)));
        group.bench_with_input(BenchmarkId::new("mixed_log", i), i,
                               |b, i| b.iter(|| use_log(*i)));
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);