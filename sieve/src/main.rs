use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let max = if (args.len() > 1) { args[1].parse().unwrap() } else { 100 };
    // println!("{:?}", sieve(max));
    for prime in sieve(max) {
        println!("{:?}", prime);
    }
}

fn sieve(max: usize) -> Vec<usize> {
    // let mut numbers = vec![u8; max];
    let mut numbers: Vec<usize> = Vec::new();

    for i in 2..(max) {
        numbers.push(i);
    }

    let mut j: usize;
    let mut num: usize;
    for i in 0..((max as f64).sqrt() as usize) {
        j = 1;
        num = numbers[i];
        if (num == 0) { continue; }
        while ( (i + (num * j)) < numbers.len()) {
            // println!("Setting item {:?} to 0", i + (num * j));
            numbers[i + (num * j)] = 0;
            j += 1;
        }
    }

    let primes: Vec<usize> = numbers.iter()
                .filter(|num| **num != 0)
                .cloned()
                .collect();

    return primes;
}
