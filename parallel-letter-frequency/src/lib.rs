use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

// my times - tried some community solutions (only safe), 
// but if I want to use channels, this is the time I'm going to get
// test bench_large_parallel   ... bench:     165,093 ns/iter (+/- 25,879)
// test bench_large_sequential ... bench:     316,714 ns/iter (+/- 19,144)
// test bench_small_parallel   ... bench:      46,569 ns/iter (+/- 18,586)
// test bench_small_sequential ... bench:      11,322 ns/iter (+/- 968)
// test bench_tiny_parallel    ... bench:      36,120 ns/iter (+/- 16,928)
// test bench_tiny_sequential  ... bench:          56 ns/iter (+/- 2)

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let slice_size = input.len() / worker_count;
    let remainder = input.len() % worker_count;
    let mut all_results: HashMap<char, usize> = HashMap::new();
    let (tx,rx) = mpsc::channel();

    for id in 0..worker_count {
        let phrase = input[id*slice_size..(id+1)*slice_size].iter().fold(String::new(), |mut acc, &x| {
            acc.push_str(x);
            acc
        });
        let thread_tx = tx.clone();
        thread::spawn(move || {
            let mut result = HashMap::new();
            for c in phrase.to_lowercase().to_string().chars() {
                if c.is_alphabetic() {
                    let count = result.entry(c).or_insert(0);
                    *count += 1;
                }
            }
            thread_tx.send(result).unwrap();
        });
    }
    drop(tx);

    // handle remainder
    if remainder > 0 {
        let phrase = input[worker_count*slice_size..].iter().fold(String::new(), |mut acc, &x| {
            acc.push_str(x);
            acc
        });
        for c in phrase.to_lowercase().to_string().chars() {
            if c.is_alphabetic() {
                let count = all_results.entry(c).or_insert(0);
                *count += 1;
            }
        }
    }
    for received in rx {
        for (key, value) in received {
            let count = all_results.entry(key).or_insert(0);
            *count += value;
        }
    }
    
    // for phrase in input {
    //     for c in phrase.to_lowercase().to_string().chars() {
    //         if c.is_alphabetic() {
    //             let count = all_results.entry(c).or_insert(0);
    //             *count += 1;
    //         }
    //     }
    // }
    all_results
}
