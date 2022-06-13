use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn crible(n: &usize) -> Vec<usize> {
    if n < &2 {
        return vec![];
    };
    let capacity = ((*n as f64) / (*n as f64).ln()) as usize;
    // Préaloccation d'après le théorème des nombres premiers
    let mut result = Vec::with_capacity(capacity);
    let root = (*n as f64).sqrt() as usize;

    'collecting: for k in 2..=*n {
        for j in result.iter() {
            if k % j == 0 {
                continue 'collecting;
                // Found a divider
            }
            if j > &root {
                break; // No possible divider left
            }
        }
        result.push(k) // That's a prime number!
    }

    result
}

fn main() -> Result<(), String> {
    let mut args = env::args();
    args.next();

    let n = match args.next() {
        Some(arg) => match arg.trim().parse() {
            Ok(x) => x,
            Err(_) => return Err(String::from("Failed to parse number!")),
        },
        None => return Err(String::from("Missing required boundary Argument")),
    };

    let mut output = match args.next() {
        Some(arg) => match File::create(arg) {
            Ok(file) => file,
            Err(e) => return Err(format!("Failed to open file: {}", e)),
        },
        None => return Err(String::from("Missing required filename argument")),
    };

    let now = Instant::now();
    let result = crible(&n);

    let elapsed_time = now.elapsed();
    eprintln!(
        "Running the algorithm for n={} took {}.{} seconds",
        &n,
        elapsed_time.as_secs(),
        elapsed_time.subsec_micros()
    );

    let res = String::from_iter(result.iter().map(|i| format!("{}\n", i)));

    match write!(output, "{}", res) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to write to file: {}", e)),
    };

    Ok(())
}
