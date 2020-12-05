use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_score() {
        assert_eq!(49714, calculate_score("COLIN", 938))
    }

    #[test]
    fn it_works() {
        assert_eq!(871198282, names_scores());
    }
}

fn read_file() -> Lines<io::BufReader<File>> {
    let file = File::open("./p022_names.txt").unwrap();
    BufReader::new(file).lines()
}

fn calculate_score(name: &str, pos: usize) -> i32 {
    let score = name
        .chars()
        .fold(0, |acc, ch| ((ch as u8 - b'A') as i32) + 1 + acc);
    score * pos as i32
}

pub fn names_scores() -> i32 {
    let lines = read_file()
        .map(|n| n.unwrap().replace("\"", ""))
        .collect::<String>();

    let mut lines = lines.split(",").collect::<Vec<_>>();
    lines.sort();

    lines
        .iter()
        .enumerate()
        .fold(0, |acc, (i, line)| calculate_score(line, i + 1) + acc)
}
