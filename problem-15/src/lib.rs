#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(137846528820, lattice_paths());
    }
}


fn factor(start: u128, end: u128) -> u128 {
    if start == end {
        return start;
    }
    start * factor(start - 1, end)
}
pub fn lattice_paths() -> u128 {
    factor(40, 21) / factor(20, 1)
}
