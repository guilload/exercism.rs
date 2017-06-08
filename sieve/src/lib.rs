pub fn primes_up_to(n: u32) -> Vec<u32> {
    let mut primes = (2..n + 1).collect::<Vec<u32>>();

    let mut i = 0;

    while i < primes.len() {
        let current = primes[i];
        primes.retain(|&p| p == current || p % current != 0);
        i += 1;
    }

    primes
}
