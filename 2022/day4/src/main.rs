use std::fs;

fn load_file(file_name: &str) -> String {
    let file_parse = fs::read_to_string(file_name);

    if file_parse.is_ok() {
       return file_parse.unwrap(); 
    } else {
        panic!("Error reading file: {}", file_name);
    }
}

struct Elf {
    start: i16,
    end: i16
}

impl Elf {
    fn from(line: &str) -> Vec<Elf> {
        let str_pair: Vec<&str> = line.split(",").collect();
        let mut pair = Vec::<Elf>::new();
        let mut coords: Vec<&str>;

        for sp in str_pair {
            coords = sp.split("-").collect();
            pair.push(Elf {
                start: coords[0].parse().unwrap(),
                end: coords[1].parse().unwrap()
            }) 
        }

        return pair;
    }

    fn contains(&self, other: &Elf) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }

    fn overlaps(&self, other: &Elf) -> bool {
        return (
            other.start >= self.start && other.start <= self.end
        ) || (
            other.end >= self.start && other.end <= self.end
        ) || (
            self.start >= other.start && self.end <= other.end
        ) || (
            self.end >= other.start && self.end <= other.end
        );
    }
}

fn main() {
    let file = load_file("input.txt");
    let mut lines: Vec<&str> = file.split("\n").collect();
    lines.remove(lines.len() -1);
    let mut pair: Vec<Elf>;
    let mut sum: i32 = 0;
    
    for line in &lines {
        pair = Elf::from(line);
        if pair[0].contains(&pair[1]) || pair[1].contains(&pair[0]) {
            sum += 1;
        }
    }

    println!("Part 1 - The number of fully overlapping pairs is: {}", sum);
    

    sum = 0;
    for line in &lines {
        pair = Elf::from(line);
        if pair[0].overlaps(&pair[1]) {
            sum += 1;
        }
    }

    println!("Part 2 - The number of overlapping pairs is: {}", sum);
}
