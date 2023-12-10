use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename: &str = args.get(1).map(String::as_ref).unwrap_or("input/sample");

    let contents = fs::read_to_string(filename).unwrap();
    let sum = sum_of_calibration_values(&contents);
    println!("The sum of the calibration values is {}", sum);
}

fn sum_of_calibration_values(contents: &str) -> u32 {
    contents.lines().map(parse_calibration_value).sum()
}

fn parse_calibration_value(line: &str) -> u32 {
    let digits: Vec<char> = tokenize_digits(line);
    let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
    number.parse::<u32>().unwrap()
}

fn tokenize_digits(line: &str) -> Vec<char> {
    (0..line.len())
        .filter_map(|i| {
            let substring = &line[i..];
            let next_char = substring.chars().next().unwrap();

            if next_char.is_ascii_digit() {
                Some(next_char)
            } else if substring.starts_with("one") {
                Some('1')
            } else if substring.starts_with("two") {
                Some('2')
            } else if substring.starts_with("three") {
                Some('3')
            } else if substring.starts_with("four") {
                Some('4')
            } else if substring.starts_with("five") {
                Some('5')
            } else if substring.starts_with("six") {
                Some('6')
            } else if substring.starts_with("seven") {
                Some('7')
            } else if substring.starts_with("eight") {
                Some('8')
            } else if substring.starts_with("nine") {
                Some('9')
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_digits_from_line() {
        let examples = [
            ("1", vec!['1']),
            ("234", vec!['2', '3', '4']),
            ("2a3b4c", vec!['2', '3', '4']),
            ("threeafourbfivec", vec!['3', '4', '5']),
            ("one2three4andfive", vec!['1', '2', '3', '4', '5']),
            ("eightwothree", vec!['8', '2', '3']),
        ];

        for (input, expected) in examples.into_iter() {
            let actual = tokenize_digits(input);
            assert_eq!(actual, expected);
        }
    }
}
