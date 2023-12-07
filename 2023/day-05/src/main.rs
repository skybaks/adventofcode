use std::fs;

#[derive(Debug)]
struct SeedMapping {
    seeds: Vec<i64>,
    instructions: Vec<InstructionsMap>,
}

#[derive(Debug)]
struct InstructionsMap {
    src_name: String,
    dst_name: String,
    ranges: Vec<MappingRange>,
}

#[derive(Debug)]
struct MappingRange {
    dst_range_start: i64,
    src_range_start: i64,
    range_len: i64,
}

impl SeedMapping {
    fn new() -> SeedMapping {
        SeedMapping {
            seeds: Vec::new(),
            instructions: Vec::new(),
        }
    }

    fn seed_to_location(&self, seed: i64) -> i64 {
        let mut start_name = "seed";
        let mut value = seed;

        loop {
            let instr = self
                .instructions
                .iter()
                .find(|n| n.src_name == start_name)
                .expect("Error could not find instruction set");

            let new_value = instr.map_value2(value);
            //println!("{}->{}: {}->{}", instr.src_name, instr.dst_name, value, new_value);

            value = new_value;
            start_name = instr.dst_name.as_str();

            if instr.dst_name == "location" {
                break;
            }
        }

        value
    }

    fn seed_to_location2(&self, seed: i64) -> i64 {
        let mut value = seed;
        for instr in &self.instructions {
            value = instr.map_value2(value);
        }
        value
    }
}

impl InstructionsMap {
    fn new() -> InstructionsMap {
        InstructionsMap {
            src_name: String::default(),
            dst_name: String::default(),
            ranges: Vec::new(),
        }
    }

    fn map_value(&self, src: i64) -> i64 {
        for range in &self.ranges {
            if src >= range.src_range_start && src < range.src_range_start + range.range_len {
                //return src + range.dst_range_start - range.src_range_start;
                let delta = range.dst_range_start - range.src_range_start;
                let new_val = src + delta;
                return new_val;
            }
        }
        return src;
    }

    fn map_value2(&self, src: i64) -> i64 {
        for range in &self.ranges {
            if src >= range.src_range_start && src < range.src_range_start + range.range_len {
                //return src + range.dst_range_start - range.src_range_start;
                let delta = range.dst_range_start - range.src_range_start;
                let new_val = src + delta;
                return new_val;
            } else if src < range.src_range_start {
                break;
            }
        }
        return src;
    }
}

impl MappingRange {
    fn new() -> MappingRange {
        MappingRange {
            dst_range_start: 0,
            src_range_start: 0,
            range_len: 0,
        }
    }
}

fn main() {
    let data = read_input();
    part1(&data);
    part2(&data);
}

fn read_input() -> SeedMapping {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-05\\input.txt")
        .expect("Error reading input file");

    let mut seed_mapping = SeedMapping::new();

    for line in contents.lines() {
        if line.starts_with("seeds: ") {
            for seed_str in line[7..].split(" ") {
                let seed = seed_str.parse::<i64>().expect("Error parsing seed number");
                seed_mapping.seeds.push(seed);
            }
        } else if line.ends_with(" map:") {
            let mut instr = InstructionsMap::new();
            let mut split_map_name = line[..line.len() - 5].split("-to-");
            instr.src_name = split_map_name
                .next()
                .expect("Error getting map source name")
                .to_owned();
            instr.dst_name = split_map_name
                .next()
                .expect("Error getting map dest name")
                .to_owned();
            seed_mapping.instructions.push(instr);
        } else if line.trim() != "" {
            let last_instr = seed_mapping
                .instructions
                .last_mut()
                .expect("Error getting last instructions");
            let mut split_map_range = line.split(" ");
            let mut new_range = MappingRange::new();
            new_range.dst_range_start = split_map_range
                .next()
                .expect("Error getting dest range start")
                .parse::<i64>()
                .expect("Error parsing dest range number");
            new_range.src_range_start = split_map_range
                .next()
                .expect("Error getting src reange start")
                .parse::<i64>()
                .expect("Error parsing src range number");
            new_range.range_len = split_map_range
                .next()
                .expect("Error getting range len")
                .parse::<i64>()
                .expect("Error parsing range len number");
            last_instr.ranges.push(new_range);
            last_instr
                .ranges
                .sort_by(|a, b| a.src_range_start.cmp(&b.src_range_start))
        }
    }

    seed_mapping
}

fn part1(mapping: &SeedMapping) {
    let mut min_location = i64::MAX;
    mapping
        .seeds
        .iter()
        .map(|s| mapping.seed_to_location(*s))
        .for_each(|l| min_location = l.min(min_location));
    println!("Part 1: {:?}", min_location);
}

fn part2(mapping: &SeedMapping) {
    let mut min_location = i64::MAX;
    let mut i = 0;
    while i < mapping.seeds.len() {
        let seed_start = mapping
            .seeds
            .get(i)
            .expect("Error unable to get seed start");
        let seed_range = mapping
            .seeds
            .get(i + 1)
            .expect("Error unable to get seed range");
        for seed_i in *seed_start..*seed_start + *seed_range {
            min_location = mapping.seed_to_location2(seed_i).min(min_location);
        }
        i += 2;
        println!("curr i: {}, {}", i, min_location);
    }
    println!("Part 2: {}", min_location);
}
