use std::fs;
use std::fs::File;
use std::io::Write;

fn load_file(file_name: &str) -> String {
    let file_parse = fs::read_to_string(file_name);

    if file_parse.is_ok() {
       return file_parse.unwrap(); 
    } else {
        panic!("Error reading file: {}", file_name);
    }
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
enum Card {
    DNE, Joker, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace
}

impl Card {
    fn from(c: &char) -> Option<Card> {
        match c {
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => Some(Card::Jack),
            '@' => Some(Card::Joker),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            'X' => Some(Card::DNE),
            _   => None::<Card>
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
enum HandType {
    HighCard, OnePair, TwoPair, ThreeOfKind, FullHouse, FourOfKind, FiveOfKind
}

fn parse_cards(c_str: &str) -> Vec<Card> {
    let mut cards = Vec::<Card>::new();

    for c in c_str.chars().collect::<Vec<char>>() {
        cards.push(Card::from(&c).unwrap());
    }

    return cards;
}

#[derive(Clone, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: i32,
    typ: HandType
}

impl Hand {
    fn from(line: &str) -> Hand {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let cards = parse_cards(parts[0]);
        
        let mut hand = Hand {
            cards,
            bid: parts[1].parse().unwrap(),
            typ: HandType::HighCard
        }; 
        hand.typ = hand.get_type();
        
        return hand; 
    }

    fn unique_cards(&self) -> Vec<Card> {
        let mut uni = Vec::<Card>::new();

        for card in &self.cards {
            if !uni.contains(&card) {
                uni.push(card.clone());
            }
        }

        return uni;
    }

    fn n_jokers(&self) -> usize {
        return self.cards.iter().filter(|&c| *c == Card::Joker).count();
    }

    fn is_five_of_kind(&self) -> bool {
        let uni = self.unique_cards();
        return uni.len() == 1 || uni.len() == 2 && uni.contains(&Card::Joker);
    }

    fn is_n_of_kind(&self, n: usize) -> bool {
        let uni = self.unique_cards();
        let n_js = self.n_jokers();
        let mut count: usize;

        for u_card in &uni {
            if u_card != &Card::Joker {
                count = self.cards.iter().filter(|&c| *c == *u_card).count();
                count += n_js; 
                if count == n {
                    return true;
                }
            }
        }

        return false;
    }

    fn is_full_house(&self) -> bool {
        let uni = self.unique_cards();
        if self.is_n_of_kind(3) && (uni.len() == 2 || (uni.len() == 3 && self.n_jokers() > 0)) {
            return true;
        }

        return false
    }

    fn count_pairs(&self) -> i8 {
        let mut count = 0;
        let mut used = Vec::<usize>::new();

        for i in 0..self.cards.len() - 1{
            for j in i + 1..self.cards.len() {
                if self.cards[i] == self.cards[j] || 
                    (self.cards[j] == Card::Joker && !used.contains(&j)) ||
                    (self.cards[i] == Card::Joker && !used.contains(&i)) {
                    
                    if self.cards[i] == Card::Joker && !used.contains(&i) {
                        used.push(i);
                    }else if self.cards[j] == Card::Joker && !used.contains(&j) {
                        used.push(j);
                    } 
                    count += 1;
                }
            }
        }

        return count;
    }
    
    fn get_type(&self) -> HandType {
        if self.is_five_of_kind() {
            return HandType::FiveOfKind;
        } 

        if self.is_n_of_kind(4) {
            return HandType::FourOfKind;
        }

        if self.is_full_house() {
            return HandType::FullHouse;
        }

        if self.is_n_of_kind(3) {
            return HandType::ThreeOfKind;
        }
        
        let n_pairs = self.count_pairs();

        if n_pairs == 2 {
            return HandType::TwoPair;
        }

        if n_pairs == 1 {
            return HandType::OnePair;
        }

        return HandType::HighCard;
    }
    
    fn cards_gt(&self, other: &Hand) -> bool {
        let mut i: usize = 0;

        while i < self.cards.len() {
            if self.cards[i] > other.cards[i] {
                return true;
            }
            
            if self.cards[i] < other.cards[i] {
                return false;
            }

            if self.cards[i] == other.cards[i] {
                i += 1;
            }
        }

        return false;
    }
}

fn swap(hands: &mut Vec<Hand>, i1: usize, i2: usize) {
    let temp = hands[i2].clone();
    hands[i2] = hands[i1].clone();
    hands[i1] = temp;
}

fn sort(hands: &mut Vec<Hand>) {
    for i in 0..hands.len() {
       for j in hands.len() - i - 1..hands.len() - 1 {
            if hands[j].typ > hands[j + 1].typ {
                swap(hands, j, j + 1);
            } else if hands[j].typ == hands[j + 1].typ {
                if hands[j].cards_gt(&hands[j + 1]) {
                    swap(hands, j, j + 1);
                }
            }
       } 
    }
}

fn to_csv(hands: &Vec<Hand>, filename: &str) {
    let mut f = File::create("data.csv").unwrap();
    let _ = f.write_all("rank,type,card1,card2,card3,card4,card5,bid".as_bytes());
    
    for (i, hand) in hands.iter().enumerate() {
        let _ = f.write_all(&format!(
            "{},{:?},{:?},{:?},{:?},{:?},{:?},{}\n",
            i as i32 + 1, hand.typ,
            hand.cards[0], hand.cards[1], hand.cards[2],
            hand.cards[3], hand.cards[4], hand.bid
        ).into_bytes()); 
    }
}

fn main() {
    let file = load_file("input.txt");
    let mut lines: Vec<&str> = file.split("\n").collect();
    let mut hands = Vec::<Hand>::new();
    let mut sum: i32 = 0; 
    
    lines.remove(lines.len() - 1);
    for line in &lines {
        hands.push(Hand::from(line));
    }

    sort(&mut hands);
    
    for (i, hand) in hands.iter().enumerate() {
        sum += (i as i32 + 1) * hand.bid;
    }

    println!("Part 1 - The total winnings are: {}", sum);
   
    sum = 0;
    hands.clear();
    for line in lines {
        hands.push(Hand::from(&line.replace("J", "@")));
    }

    sort(&mut hands);
    for (i, hand) in hands.iter().enumerate() {
        println!("{:?} {:?}: {}",
            hand.typ,
            hand.cards,
            // i as i32 + 1 * hand.bid
            hand.bid
        );

        sum += (i as i32 + 1) * hand.bid;
    }

    println!("Part 2 - The total winnings are: {}", sum);
}
