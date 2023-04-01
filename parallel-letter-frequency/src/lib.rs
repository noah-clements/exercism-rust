use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

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
