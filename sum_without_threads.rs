use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    
    let n = 1000000000;
    let mut sum: u64 = 0;

    for i in 1..=n {
        sum += i;
    }
    println!("Sum of numbers from 1 to {}: {}", n, sum);
    
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
}