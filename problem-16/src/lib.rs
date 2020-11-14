#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1366, power_digit_sum());
    }
}

pub fn power_digit_sum() -> i32 {
    let mut result = vec![2];
    for _ in 0..999 {
        let len = result.len();
        let mut carry = 0;
        for j in 0..len {
            result[j] = result[j] * 2 + carry;
            carry = 0;
            if result[j] >= 10 {
                result[j] %= 10;
                if result.iter().nth(j+1).is_none() {
                    result.push(1);
                } else {
                    carry = 1;
                }
            }
        }
    }

    result.iter().sum()
}
