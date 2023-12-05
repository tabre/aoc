use std::fs;
use std::num::ParseIntError;

use regex::Regex;
use std::borrow::Cow;

fn load_file(file_name: &str) -> String {
    let file_parse = fs::read_to_string(file_name);

    if file_parse.is_ok() {
       return file_parse.unwrap(); 
    } else {
        panic!("Error reading file.");
    }
}

fn get_pos_offset(pos: [usize; 2], offset: [i8; 2]) -> [usize; 2] {
    let mut pos_offset: [usize; 2] = [0, 0];

    for (i, o) in offset.iter().enumerate() {
        if o >= &0 {
            pos_offset[i] = pos[i] + o.abs() as usize;
        } else if pos[i] > 0 {
            pos_offset[i] = pos[i] - o.abs() as usize;
        } else {
            pos_offset[i] = 0;
        }
    }
   return pos_offset;
}

#[derive(Debug)]
struct Num {
   val: i16,
   l_num: usize,
   start: usize,
   end: usize
}

impl Num {
    fn adjacent_to(&self, c_num: usize, l_num: usize) -> bool {
        let mut coords = Vec::<[usize; 2]>::new();
        let offsets: Vec<i8> = [-1, 0, 1].to_vec();

        for pos in self.start..self.end + 1 {
            for l_offset in &offsets{
                for c_offset in &offsets {
                    coords.push(get_pos_offset(
                        [pos, self.l_num],
                        [*c_offset, *l_offset]
                    ));       
                }
            }
        }

        return coords.contains(&[c_num, l_num]);
    }
}

#[derive(Debug)]
struct Sym {
    val: char,
    l_num: usize,
    c_num: usize
}

fn parse(file: &str) -> (Vec<Num>, Vec<Sym>){
    let mut nums = Vec::<Num>::new();
    let mut syms = Vec::<Sym>::new();
    let mut num_strs: Vec<&str>;
    let mut sym_strs: Vec<&str>;
    let mut pos: usize;
    let mut num: Num;
    let mut sym: Sym;
    let mut result: Result<i16, ParseIntError>;
    let mut str_rep: Cow<str>;
    let mut eline: String;
    
    let re_dig = Regex::new(r"\D").unwrap();
    let re_sym = Regex::new(r"[\d\.]").unwrap();

    for (l_num, line) in file.split("\n").enumerate() {
        str_rep = re_dig.replace_all(line, " ");
        num_strs = str_rep.split_ascii_whitespace().collect::<Vec<&str>>();
        eline = line.to_string();
        
        for n in num_strs {
            pos = eline.find(n).unwrap();
            eline = eline.replacen(n, &" ".repeat(n.len()), 1);
            result = n.parse::<i16>();
            if result.is_ok() {
                num = Num{
                    val: result.unwrap(),
                    l_num,
                    start: pos,
                    end: pos + n.len() - 1
                };
                nums.push(num); 
            }
        }

        str_rep = re_sym.replace_all(line, " ");
        sym_strs = str_rep.split_ascii_whitespace().collect::<Vec<&str>>(); 
        

        for s in sym_strs {
            pos = eline.find(s).unwrap();
            eline = eline.replacen(s, " ", 1);
            sym = Sym{
                val: s.chars().next().unwrap(),
                l_num,
                c_num: pos
            };
            syms.push(sym);
        }
    }

    return (nums, syms)
}

fn main() {
    let file = load_file("input.txt");
    let nums: Vec<Num>;
    let syms: Vec<Sym>;
    let mut sum: i32 = 0;

    (nums, syms) = parse(&file);
     
    // Part 1
    for num in &nums {
        for sym in &syms {
            if num.adjacent_to(sym.c_num, sym.l_num)  {
                sum += i32::from(num.val);
            }
        }
    }
    
    println!("Part 1 - The sum of part numbers is: {}", sum);    

    // Part 2
    let mut gear_parts = Vec::<i16>::new();
    sum = 0;

    for sym in &syms {
        for num in &nums {
            if sym.val == '*' && num.adjacent_to(sym.c_num, sym.l_num) {
                gear_parts.push(num.val);
                if gear_parts.len() == 2 {
                    sum += i32::from(gear_parts[0]) * i32::from(gear_parts[1]);
                }
            }
        }
        gear_parts.clear();
    }

    println!("Part 2 - The sum of gear ratios is: {}", sum);
}
