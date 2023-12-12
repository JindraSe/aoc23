use std::{env::args, fs::read_to_string, path::Path};

struct Revelation {
    reds: u32,
    greens: u32,
    blues: u32,
}

impl Revelation {
    fn from_str(as_str: &str) -> Revelation {
        let mut split_str = as_str.split(' ');

        let count = str::parse::<u32>(split_str.nth(0).unwrap()).unwrap();
        let color = split_str.nth(0).unwrap();

        return match color {
            "red" => Revelation {
                reds: count,
                greens: 0,
                blues: 0,
            },
            "green" => Revelation {
                reds: 0,
                greens: count,
                blues: 0,
            },
            "blue" => Revelation {
                reds: 0,
                greens: 0,
                blues: count,
            },
            _ => Revelation {
                reds: 0,
                greens: 0,
                blues: 0,
            },
        };
    }

    fn combine(&self, other: &Self) -> Self {
        return Revelation {
            reds: self.reds + other.reds,
            greens: self.greens + other.greens,
            blues: self.blues + other.blues,
        };
    }

    fn is_leq_than(&self, other: &Self) -> bool {
        return self.reds <= other.reds && self.greens <= other.greens && self.blues <= other.blues;
    }
}

struct Game {
    id: u32,
    revelations: Vec<Revelation>,
}

impl Game {
    fn is_possible(&self, total: &Revelation) -> bool {
        for revelation in &self.revelations {
            if !revelation.is_leq_than(&total) {
                return false;
            }
        }

        return true;
    }

    fn smallest_possible(&self) -> Revelation {
        let mut res = Revelation {
            reds: 0,
            greens: 0,
            blues: 0,
        };

        for revelation in &self.revelations {
            res.reds = std::cmp::max(res.reds, revelation.reds);
            res.greens = std::cmp::max(res.greens, revelation.greens);
            res.blues = std::cmp::max(res.blues, revelation.blues);
        }

        return res;
    }

    fn power(&self) -> u32 {
        let revel = self.smallest_possible();
        return revel.reds * revel.greens * revel.blues;
    }
}

fn game_from_line(line: &str) -> Game {
    let mut split_line = line.split(": ");

    let prefix = split_line.nth(0);
    let suffix = split_line.nth(0);

    let mut sets: Vec<Revelation> = Vec::new();

    for revelation_str in suffix.unwrap().split("; ") {
        let mut revelation = Revelation {
            reds: 0,
            blues: 0,
            greens: 0,
        };

        for record in revelation_str.split(", ") {
            revelation = revelation.combine(&Revelation::from_str(record));
        }
        sets.push(revelation);
    }

    return Game {
        id: str::parse::<u32>(&prefix.unwrap()[5..]).unwrap(),
        revelations: sets,
    };
}

fn parse_input(input_path: &Path) -> Vec<Game> {
    return read_to_string(input_path)
        .expect("Input file not found.")
        .lines()
        .map(game_from_line)
        .collect();
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument!");
    }

    let path_to_input = Path::new(&args[1]);
    let games = parse_input(path_to_input);

    const TOTAL: Revelation = Revelation {
        reds: 12,
        greens: 13,
        blues: 14,
    };

    println!(
        "Possible IDs sum: {}",
        games
            .iter()
            .filter(|game| game.is_possible(&TOTAL))
            .map(|game| game.id)
            .sum::<u32>()
    );

    println!(
        "Sum of power numbers: {}",
        games.iter().map(|game| game.power()).sum::<u32>()
    );
}
