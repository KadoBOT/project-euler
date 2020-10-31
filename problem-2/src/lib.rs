#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn it_works() {
        assert_eq!(44, fibonacci(35));
    }

    #[test]
    fn it_works_with_four_millions() {
        assert_eq!(4613732, fibonacci(4_000_000))
    }
}

pub fn fibonacci(n: i64) -> i64 {
    let mut fib_3 = 2;
    let mut fib_6 = 0;

    let mut result = 2;
    let mut sum = 0;

    while result <= n {
        sum += result;

        result = 4*fib_3+fib_6;
        fib_6 = fib_3;
        fib_3 = result;
    }

    sum
}
