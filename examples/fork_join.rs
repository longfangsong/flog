use std::f64::consts::PI;
use std::fs::File;
use std::io::{SeekFrom, Read, Seek};
use std::thread;
use std::time::Instant;
use flog::{log_str, flush, LogItem, log};
use std::io::Write;
use std::sync::{Mutex, Arc};

const N: usize = 1 << 22;

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
        (fib(i % 25) as f64).sin().acos().tan(),
    ];
    data.iter().sum()
}

fn mixed(from: Arc<Mutex<File>>, to: Arc<Mutex<File>>, i: usize) {
    let mut obj = LogItem::new();
    let start_time = minstant::now();

    let mut from = from.lock().unwrap();
    let mut buffer = [0u8; 8];
    from.seek(SeekFrom::Start(i as u64 * 8)).unwrap();
    from.read_exact(&mut buffer).unwrap();
    drop(from);
    let n = u64::from_le_bytes(buffer);
    obj.u64(i as u64).str(" read ").u64(i as u64).u64(minstant::now() - start_time).char('\n');

    let start_time = minstant::now();
    let result = heavy_cpu(n as usize);
    obj.u64(i as u64).str(" heavy_cpu ").u64(i as u64).u64(minstant::now() - start_time).char('\n');
    let start_time = minstant::now();
    let mut to = to.lock().unwrap();
    write!(to, "result is: {}\n", result).unwrap();
    obj.u64(i as u64).str(" write ").u64(i as u64).u64(minstant::now() - start_time).char('\n');

    log(obj);
}

fn use_log(thread_count: usize, i: usize) {
    let from = Arc::new(Mutex::new(File::open("./input.bin").unwrap()));
    let to = Arc::new(Mutex::new(File::create("./output.txt").unwrap()));
    let start_time = minstant::now();
    let mut threads = Vec::new();
    for _tid in 0..thread_count {
        let from = from.clone();
        let to = to.clone();
        threads.push(thread::spawn(move || {
            for i in 0..i / thread_count {
                let from = from.clone();
                let to = to.clone();

                let mut obj = LogItem::new();
                obj.char('[').u64(minstant::now() - start_time).str("] ").u64(i as u64).str(" start\n");
                log(obj);

                mixed(from, to, i);

                let mut obj = LogItem::new();
                obj.char('[').u64(minstant::now() - start_time).str("] ").u64(i as u64).str(" end\n");
                log(obj);
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
    let guard = pprof::ProfilerGuard::new(100).unwrap();
    use_log(128, N);
    if let Ok(report) = guard.report().build() {
        let file = File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
    };
}