#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use crate::smallest_multiple;

    #[test]
    fn it_works() {
        assert_eq!(232792560, smallest_multiple());
    }

    #[bench]
    fn bench_it_works(b: &mut Bencher) {
        b.iter(|| smallest_multiple());
    }
}

pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

pub fn smallest_multiple() -> i64 {
    let mut result = 2;
    for i in 3..=20 {
        result = lcm(result, i);
    }

    result
}
