use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("input/test").unwrap();
    let sum = sum_of_calibration_values(&contents);
    println!("The sum of the calibration values is {}", sum);
}

fn sum_of_calibration_values(contents: &str) -> u32 {
    contents.lines().map(parse_calibration_value).sum()
}

fn parse_calibration_value(line: &str) -> u32 {
    let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
    let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
    number.parse::<u32>().unwrap()
}
