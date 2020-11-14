use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(837799, longest_collatz_sequence());
    }
}

const MAX: i64 = 1_000_000;


pub fn longest_collatz_sequence() -> i64 {
    let mut cached_chains = HashMap::new();

    let calculate_odd = |n: i64| 3 * n + 1;
    let calculate_even = |n: i64| n / 2;

    let mut value = 2;
    let mut chains = 2;

    for i in 2..MAX {
        let mut cur_chains = 1;
        let mut j = i;
        while j != 1 {
            if cached_chains.contains_key(&j) {
                cur_chains += cached_chains.get(&j).unwrap();
                break;
            }
            if j % 2 == 0 {
                cur_chains += 1;
                j = calculate_even(j);
            } else {
                cur_chains += 2;
                j = calculate_odd(j) / 2;
            }
        }
        cached_chains.insert(i, cur_chains);

        if cur_chains > chains {
            value = i;
            chains = cur_chains;
        }
    }

    value
}
