#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(31875000, special_pyhagorean_triplet());
    }
}

const A: i64 = 1000 / 3;

// a + b + c = 1000;
// Solving for b: b = 1000 - a - c
// a^2 + b^2 = c^2
// Solving for c (replacing b): c^2 = a^2 + (1000-a-c)^2
// c = (a*a - 1000*a + 500000)/(1000 - a)
// Such that, a < b < c
pub fn special_pyhagorean_triplet() -> i64 {
    for a in 1..A {
        let c = (a * a - 1000 * a + 500000) / (1000 - a);
        let b = 1000 - a - c;
        if b > a && c > b && (a * a + b * b) == c * c {
            return a * b * c;
        }
    }
    0
}
