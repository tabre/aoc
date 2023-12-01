use std::fs;
use core::str::FromStr;

fn newline_fix(s: &mut String) {
   *s = s.replace("\n\n", "\n"); 
}

fn get_sum(elf: &str) -> i32 {
    let mut sum: i32 = 0;
    let mut parse: Result<i32, <i32 as FromStr>::Err>;
    let snacks: Vec<&str> = elf.split("\n").collect();

    for snack in snacks {
        parse = snack.parse::<i32>();
        if parse.is_ok() {
            sum += parse.unwrap();
        } else {
            println!("Error parsing.");
        }
    }

    return sum;
}

fn get_sums(s: String) -> Vec<i32> {
    let mut sums = Vec::<i32>::new();
    let elves:Vec<&str> = s.split("\n\n").collect();

    for elf in elves {
        sums.push(get_sum(elf));        
    }

    return sums;
}

fn show_sums(sums: &Vec<i32>) {
    for (i, sum) in sums.iter().enumerate() {
        println!("Elf {}:\t{}", i, sum);
    }
}

fn show_max(sums: &Vec<i32>) {
    let mut max_i: usize = 0;
    let mut max: i32 = -1;

    for (i, sum) in sums.iter().enumerate() {
        if sum > &max {
            max_i = i;
            max = *sum;
        } 
    }

    println!("Elf {} has the most calories with {}", max_i, max);
}

fn show_top_3(sums: &mut Vec<i32>) {
    let mut sum: i32 = 0;
    let mut current: i32;
    sums.sort();

    for i in 1..4 {
        current = *sums.get(sums.len() - i).unwrap();
        sum += current;

        println!("The #{} elf has {} calories", i, current);
    }

    println!(); 
    println!("The top 3 elves have {} calories combined.", sum);
}

fn main() {
    let mut s = fs::read_to_string("input.txt").unwrap();
    newline_fix(&mut s);

    let mut sums = get_sums(s);
    // show_sums(&sums);
    println!();
    show_max(&sums);
    println!();
    show_top_3(&mut sums);
}
