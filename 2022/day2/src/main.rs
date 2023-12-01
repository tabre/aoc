use std::fs;

fn get_value(c: &char) -> i8 {
    match c {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => -1
    }
}

fn get_score(c1: &char, c2: &char) -> i8 {
    let i1 = get_value(c1);
    let i2 = get_value(c2);

    if i2 == i1 {
        return i2 + 3;
    }
    
    if i2 == 1 {
        if i1 == 2 {
            return i2 + 0;
        }
        return i2 + 6;
    }

    if i2 == 2 {
        if i1 == 3 {
           return i2 + 0; 
        }
        return i2 + 6;
    }

    if i2 == 3 {
        if i1 == 1 {
            return i2 + 0;
        }
        return i2 + 6;
    }
    
    return 0;
}

fn get_choice(c1: &Option<&char>, c2: &Option<&char>) -> char {
    let ch1 = c1.unwrap();
    let ch2 = c2.unwrap();

    if *ch2 == 'X' {
        // lose
        if *ch1 == 'A' {
            return 'Z';
        }
        
        if *ch1 == 'B' {
            return 'X';
        }
        
        return 'Y';
    }

    if *ch2 == 'Y' {
        //  draw
        return *ch1;
    }

    if *ch2 == 'Z' {
        // win
        if *ch1 == 'A' {
            return 'Y';
        }

        if *ch1 == 'B' {
            return 'Z';
        }

        return 'X';
    }

    return '0';
}

fn main() {
    let file_parse = fs::read_to_string("input.txt");
    let file: String;
    let lines: Vec<&str>;
    let mut chars: Vec<char>;
    let mut  score: i32 = 0;
    let mut c1;
    let mut c2;
    let mut choice;

    if file_parse.is_ok() {
        file = file_parse.unwrap();        
    } else {
        file = String::from("");
        print!("File read error.");
    }

    lines = file.split("\n").collect();

    for line in lines.iter() {
        chars = line.chars().collect();

        c1 = chars.get(0);
        c2 = chars.get(2);
        
        if c1.is_some() && c2.is_some() {
            score += i32::from(
                get_score(
                    c1.unwrap(),
                    c2.unwrap()
                )
            );
        }
    }

    println!("Part 1: Your total score is {}\n", score);
    
    score = 0;

    for line in lines.iter() {
        chars = line.chars().collect();

        c1 = chars.get(0);
        c2 = chars.get(2);

        if c1.is_some() && c2.is_some() {
            choice = get_choice(&c1, &c2);

            score += i32::from(
                get_score(
                    &c1.unwrap(),
                    &choice
                )
            )
        }
    }

    println!("Part 2: Your total score is {}\n", score);
}
