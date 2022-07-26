use std::env;
use std::time::Instant;

fn main() -> Result<(), &'static str> {
    let mut args = env::args();
    args.next();

    let n: u64 = match args.next() {
        Some(s) => match s.trim().parse() {
            Ok(x) => x,
            Err(_) => return Err("Failed to parse number!"),
        },
        None => return Err("Please specify a number !"),
    };

    let mut fibo1: u64 = 0;

    let mut fibo2: u64 = 1;

    let now = Instant::now();
    if n == 0 {
        println!("fibo(0) = 0")
    } else {
        for _ in 0..(n - 1) {
            (fibo1, fibo2) = (fibo2, (fibo1 + fibo2) % MODULUS);
        }
        println!("fibo({}) = {}", &n, &fibo2)
    };

    let elapsed_time = now.elapsed();

    println!(
        "Calculations took {}.{}",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis()
    );

    Ok(())
}

const MODULUS: u64 = (10 as u64).pow(9) + 7;
