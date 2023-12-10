use std::fs;

fn load_file(file_name: &str) -> String {
    let file_parse = fs::read_to_string(file_name);

    if file_parse.is_ok() {
       return file_parse.unwrap(); 
    } else {
        panic!("Error reading file: {}", file_name);
    }
}

fn parse_line(line: &str) -> Vec<i64> {
    let mut values = Vec::<i64>::new();
    let str_vals: Vec<&str> = line.split_ascii_whitespace().collect();

    for str_val in str_vals {
        values.push(str_val.parse::<i64>().unwrap());
    }

    return values; 
}

fn all_zeros(values: &Vec<i64>) -> bool {
    for value in values {
        if *value != 0 {
            return false;
        }
    }

    return true;
}

fn get_diffs(values: &Vec<i64>) -> Vec<i64> {
    let mut diffs = Vec::<i64>::new();
    
    for i in 0..values.len() - 1 {
       diffs.push(values[i + 1] - values[i]);
    }

    return diffs;
}

fn get_last_value(values: &mut Vec<i64>) {
    if all_zeros(values) {
        values.push(0);
    } else {
        let mut diffs = get_diffs(values);
        get_last_value(&mut diffs);
        values.push(values[values.len() - 1] + diffs[diffs.len() - 1]);
    }
}

fn get_prev_value(values: &mut Vec<i64>) {
    if all_zeros(values) {
        values.insert(0, 0);
    } else {
        let mut diffs = get_diffs(values);
        get_prev_value(&mut diffs);
        values.insert(0, values[0] - diffs[0]);
    }
}

fn main() {
    let file = load_file("input.txt");
    let mut lines: Vec<&str> = file.split("\n").collect();
    let mut values: Vec<i64>;
    let mut sum = 0;

    lines.remove(lines.len() - 1);

    for line in &lines {
        values = parse_line(line);
        get_last_value(&mut values);
        sum += values[values.len() - 1];
    }

    println!("Part 1 - The sum of the next values in each series is {}", sum);
    
    sum = 0;

    for line in &lines {
        values = parse_line(line);
        get_prev_value(&mut values);
        sum += values[0];
    }

    println!("Part 2 - The sum of the previous values in each series is {}", sum);
}
