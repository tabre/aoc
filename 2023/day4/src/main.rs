use std::fs;

fn load_file(file_name: &str) -> String {
    let file_parse = fs::read_to_string(file_name);

    if file_parse.is_ok() {
       return file_parse.unwrap(); 
    } else {
        panic!("Error reading file.");
    }
}

struct Card {
    num: i16,
    numbers: Vec<i16>, 
    winners: Vec<i16>,
    count: i32
}

impl Card {
    fn get_value(&self) -> i32 {
        let mut value: i32 = 0;

        for num in &self.numbers {
            if self.winners.contains(&num) {
                if value == 0 {
                    value = 1;
                } else {
                    value *= 2;
                }
            } 
        }
        return value;
    }

    fn get_num_winners(&self) -> usize {
        let mut n: usize = 0;

        for num in &self.numbers {
            if self.winners.contains(&num) {
                n += 1;
            }
        }

        return n;
    }

    fn from(line: &str) -> Card {
        let mut numbers = Vec::<i16>::new();
        let mut winners = Vec::<i16>::new();
        
        let col_split: Vec<&str> = line.split(":").collect();
        let title: Vec<&str> = col_split[0].split_ascii_whitespace().collect();
        let num = title[1].parse().unwrap();

        let mut num_strs: Vec<&str> = col_split[1].split("|").collect();
        let win_strs: Vec<&str> = num_strs[0].split_ascii_whitespace().collect();
        num_strs = num_strs[1].split_ascii_whitespace().collect();

        for s in win_strs {
            winners.push(s.parse().unwrap());
        }

        for s in num_strs {
            numbers.push(s.parse().unwrap());
        }

        return Card {num, numbers, winners, count: 1};
    }

    fn inc_count(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let file = load_file("input.txt");
    let mut lines: Vec<&str> = file.split("\n").collect();
    _ = &lines.pop();
    let mut cards = Vec::<Card>::new();
    let mut sum = 0;
    
    // Part 1
    for line in lines {
        cards.push(Card::from(line));
    }

    for card in &cards {
        sum += card.get_value()
    }

    println!("Part 1 - The value of the cards is {}", sum);

    // Part 2
    sum = 0;
    let mut idx: usize;
    let max_idx: usize = cards.len() - 1;
    let mut n_win: usize;

    for i in 0..max_idx {
        n_win = cards[i].get_num_winners();
        for _ in 0..cards[i].count {
            for j in i + 1..i + n_win + 1 {
                idx = j as usize;
                if idx <= max_idx {
                    cards[idx].inc_count();
                }
            }
        }
    }

    for card in cards {
        sum += card.count as i32;
    }

    println!("Part 2 - The number of scratchcards is {}", sum);

}
