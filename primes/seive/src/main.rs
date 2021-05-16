fn main() {
    let primes = seive(100_000_000);
    //println!("primes: {:?}", primes);
    println!("no. primes {}", primes.len())
}

fn seive(n: usize) -> Vec<usize> {

    // candidates vector as bools (mutable) length = n; all values = true
    let mut candidates = vec![true; n];
    candidates[0] = false;
    candidates[1] = false;

    // limit is sqrt(n) 
    let limit = (n as f64).sqrt() as usize + 1;

    // eliminate all factors of i
    for i in 2..limit {
        if candidates[i] {
            for j in (i+i..candidates.len()).step_by(i) {
                candidates[j] = false;
            }   
        }
    }

    primes_from_bool_vec(&candidates)
}

fn primes_from_bool_vec(candidates: &Vec<bool>) -> Vec<usize> {
    // store primes 
    let mut primes: Vec<usize> = Vec::new();
    for i in 2..candidates.len() {
        if candidates[i] {
            primes.push(i);
        }
    }
    primes
}

fn primes_from_bool_vec2(candidates: &Vec<bool>) -> Vec<usize> {
    // store primes 
    let mut primes: Vec<usize> = Vec::new();
    for (i, &value) in candidates.iter().enumerate() {
        if value {
            primes.push(i);
        }
    }
    primes
}
