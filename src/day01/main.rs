use std::{env::args, fs::read_to_string, path::Path};

fn read_lines(input_path: &Path) -> Vec<String> {
    return read_to_string(input_path)
        .expect("Input file Not Found.")
        .lines()
        .map(String::from)
        .filter(|s| !s.is_empty())
        .collect();
}

fn get_calibration_value(line: &str, consider_text: bool) -> u32 {
    let mut digits: Vec<u32> = Vec::new();

    for (i, _) in line.chars().enumerate() {
        let num = get_number(line.get(i..).unwrap(), consider_text);
        if num.is_some() {
            digits.push(num.unwrap());
        }
    }

    return digits[0] * 10 + digits.last().unwrap();
}

fn translate_digit(name: &str) -> u32 {
    return match name {
        "one" => 1, "two" => 2, "three" => 3,
        "four" => 4, "five" => 5, "six" => 6,
        "seven" => 7, "eight" => 8, "nine" => 9,
        _ => 0,
    }
}

fn get_number(substr: &str, consider_text: bool) -> Option<u32> {
    if substr.chars().nth(0)?.is_digit(10) {
        return substr.chars().nth(0).unwrap().to_digit(10);
    }

    if !consider_text {
        return None;
    }

    for num in ["one", "two", "three",
                "four", "five", "six",
                "seven", "eight", "nine"] {
        if substr.starts_with(num) {
            return Some(translate_digit(num));
        }
    }

    return None;
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument!");
    }

    let path_to_input = Path::new(&args[1]);
    let lines = read_lines(path_to_input);

    let sum: u32 = lines
       .iter()
       .map(|line| u32::from(get_calibration_value(line, false)))
       .sum();

    println!("Sum of the calibration values (task 1) is: {}", sum);

    let sum_2: u32 = lines
        .iter()
        .map(|line| u32::from(get_calibration_value(line, true)))
        .sum();

    println!("Som of the calibration values (task 2) is: {}", sum_2);
}
