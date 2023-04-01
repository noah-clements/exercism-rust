use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::cmp::min;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // let slice_size = input.len() / worker_count;
    // let remainder = input.len() % worker_count;
    let mut all_results: HashMap<char, usize> = HashMap::new();
    let (tx,rx) = mpsc::channel();
    let mut phrases = input.iter();
    while phrases.len() > 0 {
        for id in 0..min(worker_count, phrases.len()) {
            if let Some(phrase) = phrases.next() {
                let thread_tx = tx.clone();
                let phrase = phrase.to_lowercase().to_string();
                thread::spawn(move || {
                    let mut result = HashMap::new();
                    for c in phrase.chars() {
                        if c.is_alphabetic() {
                            let count = result.entry(c).or_insert(0);
                            *count += 1;
                        }
                    }
                    thread_tx.send(result).unwrap();
                    println!("Thread {} finished", id);
                });
            }
        }
    }
    drop(tx);
    for received in rx {
        println!("{:?}", received);
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
