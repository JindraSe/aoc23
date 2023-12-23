use std::{env::args, fs::read_to_string, path::Path};

#[derive(Clone)]
struct Card {
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
}

impl Card {
    fn count_matches(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|num| self.numbers_you_have.contains(num))
            .count()
            .try_into()
            .unwrap()
    }

    fn score(&self) -> u32 {
        match self.count_matches() {
            0 => 0,
            x => 2_u32.pow(x - 1),
        }
    }
}

fn parse_card(line: &str) -> Card {
    let mut split_numbers = line.split(": ").skip(1).next().unwrap().split(" | ");

    Card {
        winning_numbers: split_numbers
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| str::parse(s).unwrap())
            .collect(),
        numbers_you_have: split_numbers
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| str::parse(s).unwrap())
            .collect(),
    }
}

fn load_data(input_path: &Path) -> Vec<Card> {
    read_to_string(input_path)
        .expect("File not found")
        .lines()
        .map(|line| parse_card(line))
        .collect()
}

fn run_game(cards: &Vec<Card>) -> u32 {
    let mut counts: Vec<u32> = cards.iter().map(|_| 1).collect();

    for (i, card) in cards.iter().enumerate() {
        for j in (i + 1)..(i + 1 + card.count_matches() as usize) {
            counts[j] += counts[i];
        }
    }

    return counts.iter().sum();
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument!");
    }

    let input_path = Path::new(&args[1]);
    let cards = load_data(input_path);

    println!(
        "Total number of points: {}",
        cards.iter().map(|card| card.score()).sum::<u32>()
    );

    println!("The total number of scratchcards is: {}", run_game(&cards));
}
