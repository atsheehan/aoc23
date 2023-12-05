use std::fs;

fn main() {
    let contents = fs::read_to_string("input/sample").unwrap();
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
    line.replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
        .chars()
        .filter(|c| c.is_ascii_digit())
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
