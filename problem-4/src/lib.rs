#[cfg(test)]
mod tests {
    use crate::{ largest_palindrome_product,optimized_largest_palindrome};

    #[test]
    fn it_works() {
        assert_eq!(906609, largest_palindrome_product());
    }

    #[test]
    fn it_works_optimized() {
        assert_eq!(906609, optimized_largest_palindrome());
    }
}

fn rev(n: i32) -> i32 {
    let radix = 10;
    let mut n = n;
    let mut reversed = 0;

    while n != 0 {
        reversed = reversed * radix + n % radix;
        n /= radix;
    }

    reversed
}

// brute force approach:
// for a = {999, 998, ..., 100} and b = {ai, ai-1, ..., 100},
// we find the product of ai * bj. We check if product is a
// palindrome and save the largest one.
pub fn largest_palindrome_product() -> i32 {
    let mut result = 0;

    for i in (100..=999).rev() {
        for j in (100..=i).rev() {
            let cur = i * j;
            if cur == rev(cur) && cur > result {
                result = cur;
            }
        }
    }

    result
}

pub fn optimized_largest_palindrome() -> i32 {
    let mut result = 0;
    for i in (100..=999).rev() {
        let (mut b, db) = if i % 11 == 0 {
            (999, 1)
        } else {
            (990, 11)
        };

        while b >= i {
            if i*b <= result {
                break;
            }

            if i*b == rev(i*b) {
                result = i*b;
            }
            b -= db;
        }
    }

    result
}
