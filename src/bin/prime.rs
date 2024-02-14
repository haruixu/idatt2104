use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    //create base mutex for vector with reference count (arc)
    let primes_mutex_arc = Arc::new(Mutex::new(vec![]));

    let start_nr = 10;
    let end_nr = 100_000;
    let nr_threads = 30;
    let mut threads = Vec::new();

    for thread in 0..nr_threads {
        let arc_mutex_copy = Arc::clone(&primes_mutex_arc);

        // move forces ownership of all variables in closure into the closure
        // i is copied into thread
        threads.push(thread::spawn(move || {
            let start_nr = thread + start_nr;

            let mut primes = arc_mutex_copy.lock().unwrap();

            (start_nr..=end_nr)
                .into_iter()
                .step_by(nr_threads)
                .filter(|num| is_prime(*num))
                .for_each(|num| primes.push(num));
        }));
    }

    for thread in threads {
        let _ = thread.join(); // let _ means that the return value should be ignored
    }

    let mut primes = primes_mutex_arc.lock().unwrap();
    primes.sort();
    println!("{:?}", primes);
}

fn is_prime(number: usize) -> bool {
    //    for i in (3..=(number as f64).sqrt() as usize).step_by(2) {
    //        if number%i == 0 {
    //            return false;
    //        }
    //    }
    if number % 2 == 0 {
        return false;
    }
    let is_prime = (3..=(number as f64).sqrt() as usize)
        .into_iter()
        .step_by(2)
        .all(|num| number % num != 0);
    is_prime
}
