use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename: &str = args.get(1).map(String::as_ref).unwrap_or("input/sample");

    let contents = fs::read_to_string(filename).unwrap();
    let sum = sum_of_part_numbers(&contents);
    println!("The sum of part numbers is {}", sum);

    let sum = sum_of_gear_ratios(&contents);
    println!("The sum of gear ratios is {}", sum);
}

fn sum_of_part_numbers(contents: &str) -> u32 {
    let tokens = parse_tokens(contents);

    let mut symbol_locations: HashSet<Location> = HashSet::new();
    for token in tokens.iter() {
        if token.is_symbol() {
            for location in token.locations() {
                symbol_locations.insert(location);
            }
        }
    }

    let part_numbers = tokens
        .iter()
        .filter_map(|token| part_number(token, &symbol_locations));
    part_numbers.sum()
}

fn sum_of_gear_ratios(contents: &str) -> u32 {
    let tokens = parse_tokens(contents);

    let mut number_locations: HashMap<Location, Token> = HashMap::new();
    for token in tokens.iter() {
        if token.is_number() {
            for location in token.locations() {
                number_locations.insert(location, *token);
            }
        }
    }

    let gears = tokens
        .iter()
        .filter_map(|token| gear(token, &number_locations));

    gears.map(|g| g.ratio()).sum()
}

fn parse_tokens(contents: &str) -> Vec<Token> {
    contents
        .lines()
        .enumerate()
        .flat_map(|(row, line)| parse_line(row as u32, line))
        .collect()
}

fn parse_line(row: u32, line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut char_iter = line.chars().enumerate().peekable();

    while let Some((col, c)) = char_iter.next() {
        let col = col as u32;

        match c {
            '.' => continue,
            '1'..='9' => {
                let mut number_string = String::from(c);
                while let Some((_, c)) = char_iter.next_if(|(_, c)| c.is_ascii_digit()) {
                    number_string.push(c);
                }

                let number: u32 = number_string.parse().unwrap();
                let end = col + number_string.len() as u32 - 1;

                tokens.push(Token {
                    col_start: col,
                    col_end: end,
                    row,
                    value: TokenValue::Number(number),
                });
            }
            '*' | '#' | '+' | '$' | '-' | '@' | '=' | '%' | '/' | '&' => tokens.push(Token {
                col_start: col,
                col_end: col,
                row,
                value: TokenValue::Symbol(c),
            }),
            _ => panic!("unknown symbol: {}", c),
        }
    }

    tokens
}

fn part_number(token: &Token, symbol_locations: &HashSet<Location>) -> Option<u32> {
    if token
        .surrounding_locations()
        .any(|l| symbol_locations.contains(&l))
    {
        token.number()
    } else {
        None
    }
}

fn gear(token: &Token, number_locations: &HashMap<Location, Token>) -> Option<Gear> {
    if let Some('*') = token.symbol() {
        let surrounding_numbers = token
            .surrounding_locations()
            .filter_map(|l| number_locations.get(&l));
        let mut deduped_part_numbers: HashMap<u64, u32> = HashMap::new();

        for number_token in surrounding_numbers {
            let mut hasher = DefaultHasher::new();
            number_token.hash(&mut hasher);
            let hashed_value = hasher.finish();

            deduped_part_numbers.insert(hashed_value, number_token.number().unwrap());
        }

        if deduped_part_numbers.len() == 2 {
            let mut values = deduped_part_numbers.values();
            let adjacent_part_numbers = [*values.next().unwrap(), *values.next().unwrap()];

            Some(Gear {
                adjacent_part_numbers,
            })
        } else {
            None
        }
    } else {
        None
    }
}

struct Gear {
    adjacent_part_numbers: [u32; 2],
}

impl Gear {
    fn ratio(&self) -> u32 {
        self.adjacent_part_numbers[0] * self.adjacent_part_numbers[1]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
struct Token {
    row: u32,
    col_start: u32,
    col_end: u32,
    value: TokenValue,
}

impl Token {
    fn is_symbol(&self) -> bool {
        matches!(self.value, TokenValue::Symbol(_))
    }

    fn is_number(&self) -> bool {
        matches!(self.value, TokenValue::Number(_))
    }

    fn symbol(&self) -> Option<char> {
        match self.value {
            TokenValue::Symbol(s) => Some(s),
            _ => None,
        }
    }

    fn number(&self) -> Option<u32> {
        match self.value {
            TokenValue::Number(n) => Some(n),
            _ => None,
        }
    }

    fn locations(&self) -> impl Iterator<Item = Location> + '_ {
        (self.col_start..=self.col_end).map(|col| Location {
            row: self.row,
            column: col,
        })
    }

    fn surrounding_locations(&self) -> impl Iterator<Item = Location> + '_ {
        let min_row = self.row.saturating_sub(1);
        let max_row = self.row.saturating_add(1);
        let min_col = self.col_start.saturating_sub(1);
        let max_col = self.col_end.saturating_add(1);

        (min_row..=max_row)
            .flat_map(move |row| (min_col..=max_col).map(move |column| Location { row, column }))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
enum TokenValue {
    Symbol(char),
    Number(u32),
}

#[derive(Hash, Eq, PartialEq)]
struct Location {
    row: u32,
    column: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_sample_for_sum_of_part_numbers() {
        let contents = fs::read_to_string("input/sample").unwrap();

        let sum = sum_of_part_numbers(&contents);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn validate_sample_for_sum_of_gear_ratios() {
        let contents = fs::read_to_string("input/sample").unwrap();

        let sum = sum_of_gear_ratios(&contents);
        assert_eq!(sum, 467835);
    }

    #[test]
    fn parse_tokens_from_input() {
        let input = "467..114..\n...*......\n..35..633.\n";

        let expected = vec![
            Token {
                row: 0,
                col_start: 0,
                col_end: 2,
                value: TokenValue::Number(467),
            },
            Token {
                row: 0,
                col_start: 5,
                col_end: 7,
                value: TokenValue::Number(114),
            },
            Token {
                row: 1,
                col_start: 3,
                col_end: 3,
                value: TokenValue::Symbol('*'),
            },
            Token {
                row: 2,
                col_start: 2,
                col_end: 3,
                value: TokenValue::Number(35),
            },
            Token {
                row: 2,
                col_start: 6,
                col_end: 8,
                value: TokenValue::Number(633),
            },
        ];

        let actual = parse_tokens(input);
        assert_eq!(actual, expected);
    }
}
