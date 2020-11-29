#[cfg(test)]
mod tests {
    use crate::factorial_digit_sum;

    #[test]
    fn it_works_example() {
        assert_eq!(27, factorial_digit_sum(10));
    }

    #[test]
    fn it_works() {
        assert_eq!(648, factorial_digit_sum(100));
    }
}

pub fn factorial_digit_sum(n: i32) -> i32 {
    let mut digits = vec![1];

    for i in 2..=n {
        let mut carry = 0;
        let mut temp = Vec::new();

        for d in digits.iter() {
            temp.push((d * i + carry) % 10);
            carry = (d * i + carry) / 10;
        }

        if carry != 0 {
            while carry > 9 {
                temp.push(carry % 10);
                carry /= 10;
            }
            temp.push(carry);
        }

        digits = temp;
    }

    digits.iter().sum()
}
