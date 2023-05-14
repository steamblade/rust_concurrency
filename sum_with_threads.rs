use std::thread;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let n:u64 = 1000000000;
    let num_threads = 4;
    let mut handles = vec![];
    let mut sum: u64 = 0;

    for i in 0..num_threads {
        let handle = thread::spawn(move || {
            let mut local_sum: u64 = 0;

            for j in (i * n / num_threads + 1)..((i + 1) * n / num_threads + 1) {
                local_sum += j as u64;
            }
            local_sum
        });

        handles.push(handle);
    }

    for handle in handles {
        let local_sum = handle.join().unwrap();
        sum += local_sum;
    }
    println!("Sum of numbers from 1 to {}: {}", n, sum);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
}