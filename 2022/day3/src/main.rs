use std::fs;

fn load_file(file_name: &String) -> String {
    let file_parse = fs::read_to_string(file_name);

    if file_parse.is_ok() {
       return file_parse.unwrap(); 
    } else {
        panic!("Error reading file.");
    }
}

fn get_priority(c: &char) -> u32 {
    let value = *c as u32;
    
    if value >= 97 && value <= 122 {
        return value - 96;
    }

    if value >= 65 && value <= 90 {
        return value - 38;
    }

    return 0; 
}

fn get_halves(s: &String) -> (&str, &str) {
    s.split_at(s.len() / 2)
}

fn get_common2(halves: (&str, &str)) -> char {
    for c0 in halves.0.chars() {
        for c1 in halves.1.chars() {
            if c0 == c1 {
                return c0.clone();
           }
       } 
    }

    return 0 as char;
}

fn get_common3(sacks: (&str, &str, &str)) -> char {
    for c0 in sacks.0.chars() {
        for c1 in sacks.1.chars() {
            if c0 == c1 {
                for c2 in sacks.2.chars() {
                    if c0 == c2 {
                        return c0.clone();
                    }
                }
           }
       } 
    }

    return 0 as char;
}

fn get_sum(p: &Vec<u32>) -> u32 {
    let mut sum: u32 = 0;

    for n in p {
        sum += n;
    }

    return sum;
}

fn main() {
    // Part 1
    let file = load_file(&String::from("input.txt")); 
    let mut lines: Vec<&str> = file.split("\n").collect();
    let mut priorities = Vec::<u32>::new();
    
    for line in lines {
        priorities.push(
            get_priority(&get_common2(get_halves(&String::from(line))))
        )
    }
    print!("Part 1 - The sum of priorities is {}\n", get_sum(&priorities));
    
    priorities.clear();

    // Part 2
    let mut group = Vec::<&str>::new();
    lines = file.split("\n").collect(); 
    
    for line in lines {
        if group.len() == 3 {
            priorities.push(
                get_priority(&get_common3(
                    (
                        &group.get(0).unwrap(), 
                        &group.get(1).unwrap(),
                        &group.get(2).unwrap()
                    )
                ))
            );
            group.clear();
        }
        group.push(line);
    }

    print!("Part 2 - The sum of priorities is {}\n", get_sum(&priorities));
}
