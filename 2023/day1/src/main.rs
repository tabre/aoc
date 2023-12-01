use std::fs;

fn word_to_digit2(word: &String) -> u32 {
    if word.contains("one") {
        return 1;
    }
    
    if word.contains("two") {
        return 2;
    }

    if word.contains("three") {
        return 3;
    }

    if word.contains("four") {
        return 4;
    }
    
    if word.contains("five") {
        return 5;
    }
    
    if word.contains("six") {
        return 6;
    }
    
    if word.contains("seven") {
        return 7;
    }
    
    if word.contains("eight") {
        return 8;
    }
    
    if word.contains("nine") {
        return 9;
    }

    return 0;
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
    print!("{}\n", sum);
    return sum;    
}

fn main() {
    let file_parse = fs::read_to_string("input.txt");
    let file: String; 
    
    if file_parse.is_ok() {
       file = file_parse.unwrap(); 
    } else {
        panic!("Error reading file.");
    }

    let lines: Vec<&str> = file.split("\n").collect();
    let mut chars: Vec<char>; 
    let mut digits = Vec::<u32>::new();
    let mut dig_parse;
    let mut dig;
    let mut sum: u32 = 0;

    for line in lines {
       chars = line.chars().collect();
       for ch in chars {
           if char::is_numeric(ch) {
                dig_parse = ch.to_digit(10);
                
                if dig_parse.is_some() {
                    dig = dig_parse.unwrap();
                    digits.push(dig);
                } else {
                    panic!("Error parsing");
                }
            }
        }
        sum += digits_to_num(&digits);
        digits.clear();
    }

    println!("Part One - The calibration value is: {}\n", sum);

    sum = 0;

    let lines: Vec<&str> = file.split("\n").collect();
    let mut prev_str = String::from("");
    let mut str_dig: u32;

    for line in lines {
        chars = line.chars().collect();
        
        for ch in chars { 
            if char::is_numeric(ch) {
                prev_str = String::from("");         
                dig_parse = ch.to_digit(10);
                
                if dig_parse.is_some() {
                    dig = dig_parse.unwrap();
                    digits.push(dig);
                } else {
                    panic!("error parsing");
                }

            } else {
                prev_str.push(ch);
                
                str_dig = word_to_digit2(&prev_str);
                if str_dig > 0 {
                    digits.push(str_dig);
                    prev_str = String::from("");
                }
            }

        }
 
        sum += digits_to_num(&digits);
        digits.clear();
        prev_str = String::from("");
    }

    println!("Part Two - The calibration value is: {}\n", sum);
    
}
