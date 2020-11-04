#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(25164150, sum_square_difference());
    }
}

fn find_sum_of_squares(num: i32) -> i32 {
    (num * ((num + 1) * (2 * num + 1))) / 6
}

fn find_square_of_sum(num: i32) -> i32 {
    let sum = num * (num + 1) / 2;
    sum * sum
}


pub fn sum_square_difference() -> i32 {
    let square_of_sum = find_square_of_sum(100);
    let sum_of_squares = find_sum_of_squares(100);
    square_of_sum - sum_of_squares
}
