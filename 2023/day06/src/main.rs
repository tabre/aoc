use std::fs;

fn load_file(file_name: &str) -> String {
    let file_parse = fs::read_to_string(file_name);

    if file_parse.is_ok() {
       return file_parse.unwrap(); 
    } else {
        panic!("Error reading file: {}", file_name);
    }
}

struct Race {
    time: i64,
    record: i64
}

impl Race {
    fn get_dist(&self, hold_time: i64) -> i64 {
        return (self.time - hold_time) * hold_time;
    }

    fn get_n_win_hts(&self) -> i64 {
        let mut n: i64 = 0;

        for hold_time in 1..self.time {
            if self.get_dist(hold_time) > self.record {
                n += 1
            }
        }

        return n;
    }
}

fn parse_races(file: &str) -> Vec::<Race> {
    let lines: Vec<&str> = file.split("\n").collect();
    let mut races = Vec::<Race>::new();
    let mut line: Vec<&str> = lines[0].split_ascii_whitespace().collect();

    line.remove(0);
    for time in line {
        races.push(Race {time: time.parse().unwrap(), record: 0});

    }

    line = lines[1].split_ascii_whitespace().collect();
    line.remove(0);
    for (i, record) in line.iter().enumerate() {
        races[i].record = record.parse().unwrap();
    }

    return races;
}

fn parse_grand_race(file: &str) -> Race {
    let lines: Vec<&str> = file.split("\n").collect();
    let mut line: Vec<&str> = lines[0].split(":").collect();

    line.remove(0);
    let time = line[0].replace(" ", "").parse().unwrap();

    line = lines[1].split(":").collect();
    line.remove(0);
    let record = line[0].replace(" ", "").parse().unwrap();

    return Race {time, record}; 
}

fn main() {
    let file = load_file("input.txt");
    let mut races = parse_races(&file);
    let mut product = races[0].get_n_win_hts();

    races.remove(0);
    for race in races {
        product *= race.get_n_win_hts();
    }

    println!("Part 1 - Total ways to beat record in all races: {}", product);

    // let grand_race = Race {time:50748685, record:242101716911252};
    let grand_race = parse_grand_race(&file);
    println!("Part 2 - Total ways to beat the grand race record: {}",
             grand_race.get_n_win_hts()
    );
}
