use std::{env::args, fs::read_to_string, path::Path};

struct ResourceTranslationRule {
    destination_from: u64,
    source_from: u64,
    len: u64,
}

impl ResourceTranslationRule {
    fn has_resource(&self, resource_id: u64) -> bool {
        resource_id >= self.source_from && resource_id < self.source_from + self.len
    }

    fn translate(&self, resource_id: u64) -> u64 {
        assert!(self.has_resource(resource_id));
        self.destination_from + resource_id - self.source_from
    }

    fn from_line(line: &str) -> ResourceTranslationRule {
        let mut split_line = line.split(' ');

        ResourceTranslationRule {
            destination_from: str::parse(split_line.next().unwrap()).unwrap(),
            source_from: str::parse(split_line.next().unwrap()).unwrap(),
            len: str::parse(split_line.next().unwrap()).unwrap(),
        }
    }
}

#[derive(Clone)]
struct ResourceRange {
    from: u64,
    len: u64,
}

struct ResourceMap {
    rules: Vec<ResourceTranslationRule>,
}

impl ResourceMap {
    fn translate(&self, resource_id: u64) -> u64 {
        for range in &self.rules {
            if range.has_resource(resource_id) {
                return range.translate(resource_id);
            }
        }

        return resource_id;
    }

    fn from_lines(lines: &str) -> ResourceMap {
        ResourceMap {
            rules: lines
                .split('\n')
                .skip(1)
                .filter(|line| !line.is_empty())
                .map(|line| ResourceTranslationRule::from_line(line))
                .collect(),
        }
    }

    fn translate_ranges(&self, ranges: &Vec<ResourceRange>) -> Vec<ResourceRange> {
        let mut res = Vec::new();

        for range in ranges {
            for rule in &self.rules {
                let start = std::cmp::max(range.from, rule.source_from);
                let end = std::cmp::min(range.from + range.len, rule.source_from + rule.len);

                if start > end {
                    continue;
                }

                res.push(ResourceRange {
                    from: start + rule.destination_from - rule.source_from,
                    len: end - start,
                })
            }
        }

        return res;
    }
}

struct Almanac<T> {
    seeds: Vec<T>,
    resource_maps: Vec<ResourceMap>,
}

impl<T> Almanac<T> {
    fn seed_to_location(&self, seed: u64) -> u64 {
        self.resource_maps
            .iter()
            .fold(seed, |resource, map| map.translate(resource))
    }
}

impl Almanac<ResourceRange> {
    fn get_location_ranges(&self) -> Vec<ResourceRange> {
        self.resource_maps
            .iter()
            .fold(self.seeds.clone(), |ranges, map| {
                map.translate_ranges(&ranges)
            })
    }
}

fn basic_seed_reader(line: &str) -> Vec<u64> {
    line[7..]
        .split(' ')
        .map(|s| str::parse(s).unwrap())
        .collect()
}

fn range_seed_reader(line: &str) -> Vec<ResourceRange> {
    let mut res = Vec::new();

    let mut prev = 0;
    for (i, s) in line[7..].split(' ').enumerate() {
        if i % 2 == 1 {
            res.push(ResourceRange {
                from: prev,
                len: str::parse(s).unwrap(),
            })
        } else {
            prev = str::parse(s).unwrap()
        }
    }

    return res;
}

fn load_data<T, F>(input_path: &Path, seed_reader: F) -> Almanac<T>
where
    F: Fn(&str) -> Vec<T>,
{
    let content = read_to_string(input_path).expect("File not found!");

    let mut split_content = content.split("\n\n");

    let seeds = seed_reader(&split_content.next().unwrap()[7..]);
    let mut resource_maps = Vec::new();

    for lines in split_content {
        resource_maps.push(ResourceMap::from_lines(lines));
    }

    return Almanac {
        seeds,
        resource_maps,
    };
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument!");
    }

    let input_path = Path::new(&args[1]);
    let almanac = load_data(input_path, basic_seed_reader);

    println!(
        "Lowest location number: {}",
        almanac
            .seeds
            .iter()
            .map(|seed| almanac.seed_to_location(*seed))
            .min()
            .unwrap()
    );

    let ranged_almanac = load_data(input_path, range_seed_reader);
    println!(
        "Lowest location number (with seed ranges): {}",
        ranged_almanac
            .get_location_ranges()
            .iter()
            .map(|range| range.from)
            .min()
            .unwrap()
    );
}
