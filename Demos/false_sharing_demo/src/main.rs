use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Instant;

const N_THREADS: usize = 8;
const N_ITERS: u64 = 100_000_000;

// #[repr(align(64))]
#[derive(Default)]
struct MyNumber {
    value: AtomicU64,
}

pub fn main() {
    let array: Vec<MyNumber> = (0..N_THREADS).map(|_| MyNumber::default()).collect();

    let array = Arc::new(array);

    let start = Instant::now();

    let mut handles = Vec::new();

    for i in 0..N_THREADS {
        let array = Arc::clone(&array);

        let handle = thread::spawn(move || {
            let value = &array[i].value;
            for _ in 0..N_ITERS {
                value.fetch_add(1, Ordering::Relaxed);
            }
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Elapsed: {:?}", start.elapsed());
}
