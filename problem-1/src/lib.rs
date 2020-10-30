#[cfg(test)]
mod tests {
    use crate::sum_of_multiples;

    #[test]
    fn it_works() {
        assert_eq!(233168, sum_of_multiples());
    }
}

fn find_sum(n: i32) -> i32 {
    let p = 999 / n;
    n*(p*(p+1)/2)

}

pub fn sum_of_multiples() -> i32 {
    find_sum(5)+ find_sum(3) - find_sum(15)
}
