use std::fs;

fn load_file(file_name: &String) -> String {
    let file_parse = fs::read_to_string(file_name);

    if file_parse.is_ok() {
       return file_parse.unwrap(); 
    } else {
        panic!("Error reading file.");
    }
}

fn replace_nums(file: &mut String) {
    *file = file.replace(
        "one", "o1e"
    ).replace(
        "two", "t2o"
    ).replace(
        "three", "t3e"
    ).replace(
        "four", "f4r"
    ).replace(
        "five", "f5e"
    ).replace(
        "six", "s6x"
    ).replace(
        "seven", "s7n"
    ).replace(
        "eight", "e8t"
    ).replace(
        "nine", "n9e"
    ).to_string(); 
}

fn digits_to_num(digits: &Vec<u32>) -> u32 {
    let mut sum: u32 = 0; 

    if digits.len() > 1 {
        sum += digits.get(0).unwrap() * 10;
        sum += digits.get(digits.len() - 1).unwrap();
    } 

    if digits.len() == 1 {
        sum += digits.get(0).unwrap() * 10;
        sum += digits.get(0).unwrap();
    }
    return sum;    
}

fn digits_from_chars(chars: &mut Vec<char>) -> Vec<u32> { 
    let mut dig_parse;
    let mut dig;
    let mut digits = Vec::<u32>::new();

    for ch in &mut *chars { 
        if char::is_numeric(*ch) {
            dig_parse = ch.to_digit(10);
            
            if dig_parse.is_some() {
                dig = dig_parse.unwrap();
                digits.push(dig);
            } else {
                panic!("error parsing");
            }
        }
    }

    return digits;
}

fn process_lines(lines: &Vec<&str>, sum: &mut u32) {
    let mut chars: Vec<char>; 
    let mut digits: Vec<u32>;
    
    for line in lines { 
        chars = line.chars().collect();
        digits = digits_from_chars(&mut chars);

        *sum += digits_to_num(&digits);
        digits.clear();
    }
}

fn main() {
    // Test 1 
    let mut file = load_file(&String::from("example1.txt")); 
    replace_nums(&mut file);
    
    let lines: Vec<&str> = file.split("\n").collect();
    let mut sum: u32 = 0;
    process_lines(&lines, &mut sum);
    println!("Part One EXAMPLE - The calibration value is: {}\n", sum);

    // Test 2
    file = load_file(&String::from("example2.txt"));
    replace_nums(&mut file);
    
    sum = 0;
    let lines: Vec<&str> = file.split("\n").collect();
    process_lines(&lines, &mut sum);
    println!("Part Two EXAMPLE - The calibration value is: {}\n", sum);
    
    // Solution
    file = load_file(&String::from("input.txt"));
    replace_nums(&mut file);
    
    sum = 0;
    let lines: Vec<&str> = file.split("\n").collect();
    process_lines(&lines, &mut sum);
    println!("Part Two SOLUTION - The calibration value is: {}\n", sum);
}
