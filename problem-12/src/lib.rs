#[cfg(test)]
mod tests {
    use crate::highly_divisible_triangular_number;

    #[test]
    fn it_works() {
        assert_eq!(76576500, highly_divisible_triangular_number(500));
    }
}

fn tau(num: i64) -> i64 {
    let mut n = num;
    let mut i = 2;
    let mut p = 1;

    if num == 1 {
        return 1;
    }

    while i* i <= n {
        let mut c = 1;
        while n % i == 0 {
            n /= i;
            c += 1;
        }
        i += 1;
        p *= c;
    }

    if n == num || n > 1 {
        p *= 1 + 1;
    }

    p
}

pub fn highly_divisible_triangular_number(x: i64) -> i64 {
    let mut n = 1;
    let mut d = 1;

    while tau(d) <= x {
        n += 1;
        d += n;
    }

    d
}
