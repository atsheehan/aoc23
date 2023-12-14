use std::{collections::HashSet, fs};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename: &str = args.get(1).map(String::as_ref).unwrap_or("input/sample");

    let contents = fs::read_to_string(filename).unwrap();
    let sum = sum_of_scratchcard_points(&contents);
    println!("The sum of scratchcard points is {}", sum);
}

fn sum_of_scratchcard_points(contents: &str) -> u32 {
    let cards = parse_cards(contents);
    cards.iter().map(|c| c.points()).sum()
}

fn parse_cards(contents: &str) -> Vec<Card> {
    contents.lines().map(parse_card).collect()
}

fn parse_card(line: &str) -> Card {
    let tokens = line.split(':').collect::<Vec<&str>>();
    let numbers = tokens[1];

    let tokens = numbers.split('|').collect::<Vec<&str>>();
    let winning_numbers = tokens[0]
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let actual_numbers = tokens[1]
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    Card {
        winning_numbers,
        actual_numbers,
    }
}

#[derive(PartialEq, Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    actual_numbers: Vec<u32>,
}

impl Card {
    fn points(&self) -> u32 {
        let num_matches = self
            .actual_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count();

        if num_matches == 0 {
            0
        } else {
            1 << (num_matches - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_sample_for_sum_of_scratchcard_points() {
        let contents = fs::read_to_string("input/sample").unwrap();

        let sum = sum_of_scratchcard_points(&contents);
        assert_eq!(sum, 13);
    }

    #[test]
    fn validate_parsing_cards() {
        let input = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let expected = Card {
            winning_numbers: HashSet::from([1, 21, 53, 59, 44]),
            actual_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
        };

        let actual = parse_card(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn validate_points() {
        let examples = [
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8),
            ("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2),
            ("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2),
            ("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1),
            ("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0),
            ("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0),
        ];

        for (input, expected) in examples.into_iter() {
            let actual = parse_card(input).points();
            assert_eq!(actual, expected);
        }
    }
}
