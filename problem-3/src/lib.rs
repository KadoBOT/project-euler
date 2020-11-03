#[cfg(test)]
mod tests {
    use crate::largest_prime_factor;
    #[test]
    fn it_works() {
        assert_eq!(6857, largest_prime_factor());
    }
}


pub fn largest_prime_factor() -> i64 {
    let mut num = 600_851_475_143;
    let mut factor = 0;

    let mut counter = 3;
    while counter * counter <= num {
        if num % counter == 0 {
            num /= counter;
            factor = counter;
        } else {
            counter += 2;
        }
    }

    if num > factor {
        factor = num;
    }

    factor
}
