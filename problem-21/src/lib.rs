use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(31626, amicable_numbers(10000));
    }
}

pub fn amicable_numbers(n: i32) -> i32 {
    let mut results = HashMap::new();

    let mut amicable_sum = 0;
    for i in 2..n {
        let mut cur = 2;
        results.insert(i, 1);
        while cur < i / cur {
            if i % cur == 0 {
                results.entry(i).and_modify(|d| *d += cur + (i / cur));
            }
            cur += 1;
        }

        let sum = results.get(&i).unwrap();
        match results.get(sum) {
            Some(d) if d == &i && sum != &i => amicable_sum += sum + d,
            _ => (),
        }
    }

    amicable_sum
}
