use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Instant;
use flog::{log, flush};
use std::fs::File;
use std::io::{Write, Read, Seek, SeekFrom};

fn prepare() {
    let mut file = File::create("./input.bin").unwrap();
    for i in 0..32768u64 {
        file.write_all(&i.to_le_bytes()).unwrap();
    }
}

fn heavy_io(from: &mut File, to: &mut File, i: usize) {
    let mut buffer = [0u8; 8];
    from.seek(SeekFrom::Start(i as u64 * 8)).unwrap();
    from.read_exact(&mut buffer).unwrap();
    writeln!(to, "result is: {}", u64::from_le_bytes(buffer)).unwrap();
}

fn pure(i: usize) {
    let mut from = File::open("./input.bin").unwrap();
    let mut to = File::create("./output.txt").unwrap();
    for i in 0..i {
        heavy_io(&mut from, &mut to, i);
    }
}

#[allow(dead_code)]
fn use_print(i: usize) {
    let mut from = File::open("./input.bin").unwrap();
    let mut to = File::create("./output.txt").unwrap();
    let mut f = File::create("log.log").unwrap();
    let start_time = Instant::now();
    for i in 0..i {
        writeln!(f, "[{:?}] {} start", start_time.elapsed(), i).unwrap();
        heavy_io(&mut from, &mut to, i);
        writeln!(f, "[{:?}] {} end", start_time.elapsed(), i).unwrap();
    }
}

fn use_log(i: usize) {
    let mut from = File::open("./input.bin").unwrap();
    let mut to = File::create("./output.txt").unwrap();
    let start_time = Instant::now();
    for i in 0..i {
        log(&format!("[{:?}] {} start", start_time.elapsed(), i));
        heavy_io(&mut from, &mut to, i);
        log(&format!("[{:?}] {} end", start_time.elapsed(), i));
    }
    flush();
}

fn bench(c: &mut Criterion) {
    prepare();
    let mut group = c.benchmark_group("log");
    for i in [1024, 2048, 4096, 8192].iter() {
        group.bench_with_input(BenchmarkId::new("high_io", i), i,
                               |b, i| b.iter(|| pure(*i)));
        group.bench_with_input(BenchmarkId::new("high_io_print", i), i,
                               |b, i| b.iter(|| use_print(*i)));
        group.bench_with_input(BenchmarkId::new("high_io_log", i), i,
                               |b, i| b.iter(|| use_log(*i)));
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
