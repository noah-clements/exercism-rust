pub fn nth(n: usize) -> usize {
    let mut total_primes = 0;
    let mut s = (n + 2) * 2;
    let mut primes: Vec<bool> = vec![true,];
    while total_primes < n + 2 {
        primes = get_primes(s);
        total_primes = primes.iter().filter(|&x| *x).count() - 2;
        s *= 2;
    }
    println!("{:?}", primes);
    primes.iter().enumerate().filter(|&(_, &x)| x).skip(2).nth(n).unwrap().0
}

fn get_primes(s: usize) -> Vec<bool> {
    let mut sieve = vec![true; s];
    for i in 2..s {
        if sieve[i] {
            for j in (i*i..s).step_by(i) {
                sieve[j] = false;
            }
        }
    }
    sieve
}

