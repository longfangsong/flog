use std::f64::consts::PI;
use std::fs::File;
use std::io::{SeekFrom, Read, Seek};
use std::thread;
use std::time::Instant;
use flog::{log, flush};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::io::Write;
use std::sync::{Mutex, Arc};

fn prepare() {
    let mut file = File::create("./input.bin").unwrap();
    for i in 0..32768u64 {
        file.write_all(&i.to_le_bytes()).unwrap();
    }
    File::create("./output.txt").unwrap();
}

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

fn mixed(from: &mut File, to: &mut File, i: usize) {
    let mut buffer = [0u8; 8];
    from.seek(SeekFrom::Start(i as u64 * 8)).unwrap();
    from.read_exact(&mut buffer).unwrap();
    let n = u64::from_le_bytes(buffer);
    write!(to, "result is: {}\n", heavy_cpu(n as usize)).unwrap();
}

fn pure(thread_count: usize, i: usize) {
    let from = Arc::new(Mutex::new(File::open("./input.bin").unwrap()));
    let to = Arc::new(Mutex::new(File::create("./output.txt").unwrap()));
    let mut threads = Vec::new();
    for _tid in 0..thread_count {
        let from = from.clone();
        let to = to.clone();
        threads.push(thread::spawn(move || {
            for i in 0..i / thread_count {
                let mut from = from.lock().unwrap();
                let mut to = to.lock().unwrap();
                mixed(&mut from, &mut to, i);
            }
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
}

fn use_print(thread_count: usize, i: usize) {
    let from = Arc::new(Mutex::new(File::open("./input.bin").unwrap()));
    let to = Arc::new(Mutex::new(File::create("./output.txt").unwrap()));

    let f = Arc::new(Mutex::new(File::create("log.log").unwrap()));
    let start_time = Instant::now();

    let mut threads = Vec::new();
    for _tid in 0..thread_count {
        let from = from.clone();
        let to = to.clone();
        let f = f.clone();
        threads.push(thread::spawn(move || {
            for i in 0..i / thread_count {
                let mut from = from.lock().unwrap();
                let mut to = to.lock().unwrap();
                let mut f = f.lock().unwrap();
                writeln!(f, "[{:?}] {} start", start_time.elapsed(), i).unwrap();
                mixed(&mut from, &mut to, i);
                writeln!(f, "[{:?}] {} end", start_time.elapsed(), i).unwrap();
            }
        }))
    }
    for thread in threads {
        thread.join().unwrap();
    }
}

fn use_log(thread_count: usize, i: usize) {
    let from = Arc::new(Mutex::new(File::open("./input.bin").unwrap()));
    let to = Arc::new(Mutex::new(File::create("./output.txt").unwrap()));
    let start_time = Instant::now();
    let mut threads = Vec::new();
    for _tid in 0..thread_count {
        let from = from.clone();
        let to = to.clone();
        threads.push(thread::spawn(move || {
            for i in 0..i / thread_count {
                let mut from = from.lock().unwrap();
                let mut to = to.lock().unwrap();
                log(&format!("[{:?}] {} start", start_time.elapsed(), i));
                mixed(&mut from, &mut to, i);
                log(&format!("[{:?}] {} end", start_time.elapsed(), i));
            }
        }))
    }
    for thread in threads {
        thread.join().unwrap();
    }
    flush();
}

fn bench(c: &mut Criterion) {
    prepare();
    let mut group = c.benchmark_group("log");
    for thread_count in [8usize, 128].iter() {
        for i in [4096usize, 16384].iter() {
            group.bench_with_input(BenchmarkId::new("fork_join", format!("{}_{}", thread_count, i)),
                                   &(thread_count, i),
                                   |b, (tc, i)| b.iter(|| pure(**tc, **i)));
            // group.bench_with_input(BenchmarkId::new("fork_join_print", format!("{}_{}", thread_count, i)),
            //                        &(thread_count, i),
            //                        |b, (tc, i)| b.iter(|| use_print(**tc, **i)));
            group.bench_with_input(BenchmarkId::new("fork_join_log", format!("{}_{}", thread_count, i)),
                                   &(thread_count, i),
                                   |b, (tc, i)| b.iter(|| use_log(**tc, **i)));
        }
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
