#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(97, nth_prime());
    }
}

const TOTAL: i32 = 10_001;

fn is_prime(prime: i32) -> bool {
    match prime {
        _ if prime == 1 => false,
        _ if prime < 4 => true,
        _ if prime % 2 == 0 => false,
        _ if prime < 9 => true,
        _ if prime % 3 == 0 => false,
        _ => {
            let greatest_prime_factor = (prime as f64).sqrt().floor() as i32;
            let mut f = 5;
            while f <= greatest_prime_factor {
                if prime % f == 0 || prime % (f+2) == 0 {
                    return false;
                }
                f += 6;
            }
            true
        }
    }
}

pub fn nth_prime() -> i32 {
    let mut prime = 1;
    let mut counter = 1;
    while counter != TOTAL {
        prime += 2;
        if is_prime(prime) {
            counter += 1;
        }
    }
    prime
}
