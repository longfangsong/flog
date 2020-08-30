use std::f64::consts::PI;
use std::fs::File;
use std::io::{SeekFrom, Read, Seek};
use std::thread;
use std::time::Instant;
use flog::{log, flush};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::io::Write;
use std::sync::{Mutex, Arc, MutexGuard};
use std::borrow::BorrowMut;

const N: usize = 1_048_576;

fn prepare() {
    let mut file = File::create("./input.bin").unwrap();
    for i in 0..N {
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
        (fib(i % 50) as f64).sin().acos().tan(),
    ];
    data.iter().sum()
}

fn mixed(mut from: MutexGuard<File>, mut to: MutexGuard<File>, i: usize) {
    let mut buffer = [0u8; 8];
    from.seek(SeekFrom::Start(i as u64 * 8)).unwrap();
    from.read_exact(&mut buffer).unwrap();
    drop(from);
    let n = u64::from_le_bytes(buffer);
    write!(to, "result is: {}\n", heavy_cpu(n as usize)).unwrap();
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
                let from = from.lock().unwrap();
                let to = to.lock().unwrap();
                log(&format!("[{:?}] {} start", start_time.elapsed(), i));
                mixed(from, to, i);
                log(&format!("[{:?}] {} end", start_time.elapsed(), i));
            }
        }))
    }
    for thread in threads {
        thread.join().unwrap();
    }
    flush();
}

fn main() {
    prepare();
    println!("prepared");
    use_log(128, N);
}