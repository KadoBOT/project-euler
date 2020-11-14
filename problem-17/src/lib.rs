#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(21124, number_letter_counts(1000));
    }
}

fn letter_count(i: i32) -> i32 {
    match i {
        1 | 2 | 6 | 10 => 3,
        4 | 5 | 9 => 4,
        3 | 7 | 8 | 40 | 50 | 60 => 5,
        11 | 12 | 20 | 30 | 80 | 90 => 6,
        15 | 16 | 70 => 7,
        13 | 14 | 18 | 19 => 8,
        17 => 9,
        1000 => 11,
        _ if i % 100 == 0 => letter_count(i / 100) + 7,
        _ if i > 100 => letter_count(i / 100) + 10 + letter_count(i % 100),
        _ if i < 40 || i > 80 => 6 + letter_count(i % 10),
        _ if i > 70 => 7 + letter_count(i % 10),
        _ if i > 40 => 5 + letter_count(i % 10),
        _ => 0
    }
}
pub fn number_letter_counts(n: i32) -> i32 {
    let mut result = 0;
    for i in 1..=n {
        result += letter_count(i);
    }
    result
}
