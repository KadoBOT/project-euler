#[cfg(test)]
mod tests {
    use super::summation_of_primes;

    #[test]
    fn it_works() {
        assert_eq!(142913828922, summation_of_primes() );
    }
}

const MAX: i64 = 2_000_000;

fn is_prime(n: i64) -> bool {
    match n {
        _ if n < 4 => true,
        _ if n % 2 == 0 => false,
        _ if n % 3 == 0 => false,
        _ if n < 9 => true,
        _ => {
            let r = (n as f64).sqrt() as i64;
            let mut f = 5;
            while f <= r {
                if n % f == 0 || n % (f+2) == 0 {
                    return false
                }
                f += 6;
            }
            return true
        }
    }
}

pub fn summation_of_primes() -> i64 {
    let mut sum = 2;
    let mut cur = 3;

    while cur < MAX {
        if is_prime(cur) {
            sum += cur;
        }
        cur += 2;
    }
    sum
}
