use std::{collections::HashMap, env::args, fs::read_to_string, ops, path::Path};

type PartNumber = u32;
type MachineSymbol = char;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Location {
    x: isize,
    y: isize,
}

impl Location {
    fn new<T, E>(x: T, y: T) -> Location
    where
        T: TryInto<isize, Error = E>,
        E: std::fmt::Debug,
    {
        Location {
            x: x.try_into().expect("Conversion not successful!"),
            y: y.try_into().expect("Conversion not successful!"),
        }
    }
}

impl ops::Add<Location> for Location {
    type Output = Location;

    fn add(self, rhs: Location) -> Location {
        Location {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn number_neighbors(number: PartNumber, location: Location) -> Vec<Location> {
    let mut res: Vec<Location> = Vec::new();
    let digit_count: isize = 1 + <u32 as TryInto<isize>>::try_into(number.ilog10()).unwrap();

    for i in (-1)..digit_count + 1 {
        res.push(location + Location::new(i, 1));
        res.push(location + Location::new(i, -1));
    }

    res.push(location + Location::new(-1, 0));
    res.push(location + Location::new(digit_count, 0));

    return res;
}

fn gear_ratio(location: Location, numbers: &HashMap<Location, PartNumber>) -> Option<u32> {
    let adjacent_numbers: Vec<u32> = numbers
        .iter()
        .filter(|(loc, _)| (loc.y - location.y).abs() < 2)
        .filter(|(loc, n)| number_neighbors(**n, **loc).contains(&location))
        .map(|(_, n)| *n)
        .collect();

    if adjacent_numbers.len() == 2 {
        return Some(adjacent_numbers.iter().product());
    }

    return None;
}

fn load_data(
    input_path: &Path,
) -> (
    HashMap<Location, MachineSymbol>,
    HashMap<Location, PartNumber>,
) {
    let mut symbols: HashMap<Location, MachineSymbol> = HashMap::new();
    let mut numbers: HashMap<Location, PartNumber> = HashMap::new();

    for (y, line) in read_to_string(input_path)
        .expect("File not found")
        .lines()
        .enumerate()
    {
        let mut in_number = false;
        let mut number_start = 0;

        for (x, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                number_start = if in_number { number_start } else { x };
                in_number = true;
                continue;
            }

            if in_number {
                in_number = false;
                numbers.insert(
                    Location::new(number_start, y),
                    str::parse::<u32>(&line[number_start..x]).unwrap(),
                );
            }

            if ch != '.' {
                symbols.insert(Location::new(x, y), ch);
            }
        }

        if in_number {
            numbers.insert(
                Location::new(number_start, y),
                str::parse::<u32>(&line[number_start..]).unwrap(),
            );
        }
    }

    return (symbols, numbers);
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument!");
    }

    let path_to_input = Path::new(&args[1]);
    let (symbols, numbers) = load_data(&path_to_input);

    let part_number_sum: u32 = numbers
        .iter()
        .filter(|(loc, n)| {
            number_neighbors(**n, **loc)
                .iter()
                .any(|loc| symbols.contains_key(loc))
        })
        .map(|(_, n)| *n)
        .sum();

    println!("The sum of all part numbers is: {}", part_number_sum);

    let gear_ratio_sum: u32 = symbols
        .iter()
        .filter(|(_, symbol)| **symbol == '*')
        .map(|(loc, _)| gear_ratio(*loc, &numbers))
        .map(|gn| gn.unwrap_or(0))
        .sum();

    println!("The sum of all gear ratios is: {}", gear_ratio_sum);
}
