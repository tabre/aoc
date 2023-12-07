use std::fs;
use rayon::prelude::*;

fn load_file(file_name: &str) -> String {
    let file_parse = fs::read_to_string(file_name);

    if file_parse.is_ok() {
       return file_parse.unwrap(); 
    } else {
        panic!("Error reading file: {}", file_name);
    }
}

struct ElfMap {
    ranges: Vec<MapXRef>
}

impl ElfMap {
    fn from(s: &str) -> ElfMap{
        let lines: Vec<&str> = s.split("\n").collect();
        let mut xref: MapXRef;
        let mut ranges = Vec::<MapXRef>::new();

        for i in 1..lines.len() {
            if lines[i].len() != 0 {
                xref = MapXRef::from(lines[i]);
                ranges.push(xref); 
            }
        }

        return ElfMap {ranges};
    }

    fn lookup(&self, i: i64) -> i64 {
        let mut dest_val;
        for xref in &self.ranges {
            dest_val = xref.lookup(i);
            if dest_val != i {
                return dest_val;
            } 
        } 
        return i; 
    }
}

#[derive(Debug)]
struct MapXRef {
    imin: i64,
    imax: i64,
    offset: i64
}

impl MapXRef {
    fn from(s: &str) -> MapXRef {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        let dest_min: i64 = parts[0].parse().unwrap();
        let src_min: i64 = parts[1].parse().unwrap();
        let range: i64 = parts[2].parse::<i64>().unwrap() - 1;

        return MapXRef {
            imin: src_min, 
            imax: src_min + range,
            offset: dest_min - src_min
        };
    }

    fn contains(&self, i: i64) -> bool {
        return i >= self.imin && i <= self.imax;
    }

    fn lookup(&self, i: i64) -> i64 {
        if self.contains(i) {
            return i + self.offset; 
        }
        return i;
    }
}

fn parse_seeds(block: &str) -> Vec<i64> {
    let str_split: Vec<&str> = block.split_ascii_whitespace().collect();
    let mut seeds = Vec::<i64>::new();

    for i in 1..str_split.len() {
        seeds.push(str_split[i].parse().unwrap());
    }

    return seeds;
}

#[derive(Debug)]
struct SeedRange {
    start: i64,
    end: i64,
    min_result: i64
}

fn parse_seeds_rng(block: &str) -> Vec<SeedRange> {
    let str_split: Vec<&str> = block.split_ascii_whitespace().collect();
    let mut seeds = Vec::<SeedRange>::new();
    let mut range: SeedRange;

    let mut i: usize = 1;
    let mut start: i64;

    while i < str_split.len() {
        start = str_split[i].parse().unwrap();
        range = SeedRange{
            start,
            end: start + str_split[i + 1].parse::<i64>().unwrap(),
            min_result: i64::MAX 
        };

        seeds.push(range);
        i += 2;
    }

    return seeds;
}

fn min(v: Vec<i64>) -> i64 {
    let mut min: i64 = i64::MAX;
    for i in v {
        if i < min {
            min = i;
        }
    }
    return min;
}

fn main() {
    let file = load_file("example.txt");
    let blocks: Vec<&str> = file.split("\n\n").collect();
    // Part 1
    let seeds = parse_seeds(blocks[0]);

    let mut maps = Vec::<ElfMap>::new();
    for i in 1..blocks.len() {
        maps.push(ElfMap::from(blocks[i]));
    }
    
    let mut locs = Vec::<i64>::new();
    let mut out: i64;
    for seed in &seeds {
        out = maps[0].lookup(*seed);
        for i in 1..maps.len() {
            out = maps[i].lookup(out);
        }
        locs.push(out);
    }

    println!("Part 1 - The closest location is: {}", min(locs.clone()));

    // Part 2
    let mut seed_ranges = parse_seeds_rng(blocks[0]); 

    // SLOW ... even with parallel processing. There are probably easy
    // optimizations that I'm missing.
    seed_ranges.par_iter_mut().for_each(|range| {
        let mut out: i64;
        for seed in range.start..range.end + 1 {
            out = seed;
            for map in &maps {
                out = map.lookup(out);
            }
            if out < range.min_result {
                range.min_result = out;
            }
        }
    });
    
    locs.clear();
    for range in &seed_ranges {
        locs.push(range.min_result);
    }

    println!("Part 2 - The closest location is: {}", min(locs));

}
