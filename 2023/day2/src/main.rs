use std::fs;

fn load_file(file_name: &String) -> String {
    let file_parse = fs::read_to_string(file_name);

    if file_parse.is_ok() {
       return file_parse.unwrap(); 
    } else {
        panic!("Error reading file.");
    }
}

fn get_split_substr<'a>(string: &'a String, s: &'a str, i: usize) -> String {
    let parts: Vec<&str> = string.split(s).collect::<Vec<&str>>();
    let selection = parts.get(i); 
    
    if selection.is_some() {
        return selection.unwrap().to_string();
    }

    return String::from("");
}

fn parse_id(line: String) -> i32 {
    let game_num = get_split_substr(&line, ":", 0);
    let n = get_split_substr(&game_num.to_string(), " ", 1);
    let id = n.parse::<i32>();
    if id.is_ok() {
        return id.unwrap();
    }
    return -1;
}

fn get_draw(draw_s: String) -> Draw {
    let mut draw = Draw{red: 0, blue: 0, green: 0}; 
    let parts: Vec<&str> = draw_s.split(",").collect();

    for part in parts {
        let cube_s = part.trim().to_string();
        let cubes: Vec<&str> = cube_s.split(" ").collect();
        let quant = cubes.get(0).unwrap().trim().parse::<i32>();
 
        if quant.is_ok() {
            if cubes[1] == "red" {
                draw.red = quant.clone().unwrap();
            }
            
            if cubes[1] == "blue" {
                draw.blue = quant.clone().unwrap();
            }
            
            if cubes[1] == "green" {
                draw.green = quant.clone().unwrap();
            } 

        }
    }

    return draw;
}

fn parse_draws(line: String) -> Vec<Draw> {
    let draws_str = get_split_substr(&line, ":", 1);
    let draws_parts: Vec<&str>;
    let mut draws = Vec::<Draw>::new();

    draws_parts = draws_str.split(";").collect();
    
    for draws_part in draws_parts {
        draws.push(get_draw(draws_part.to_string()));
    }

    return draws;
}

fn game_possible(game: &Game, rlim: i32, glim: i32, blim: i32) -> bool {
    for draw in &game.draws {
        if draw.red > rlim {
            println!(
                "Game {} - red draw {} greater than limit {}",
                game.id, draw.red, rlim
            );

            return false;
        }

        if draw.blue > blim {
            println!(
                "Game {} - blue draw {} greater than limit {}",
                game.id, draw.blue, blim
            );

            return false;
        }

        if draw.green > glim {
            println!(
                "Game {} - green draw {} greater than limit {}",
                game.id, draw.green, glim
            );

            return false;
        }
    }

    return true;
}

fn get_minimum_draw(game: &Game) -> Draw {
    let mut red = 0; 
    let mut blue = 0; 
    let mut green = 0; 
    
    for draw in &game.draws {
        if draw.red > red {
            red = draw.red;
        }

        if draw.blue > blue {
            blue = draw.blue;
        }
        
        if draw.green > green {
            green = draw.green;
        }
    }

    Draw{
        red,
        blue,
        green
    }
}

#[derive(Debug)]
struct Draw {
    red: i32,
    green: i32,
    blue: i32
}

impl Draw {
    fn get_power(&self) -> i32 {
        return self.red * self.green * self.blue;
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    draws: Vec<Draw>
}

impl Game {
    fn from_line(line: &String) -> Game {
        Game{
            id: parse_id(line.to_string()),
            draws: parse_draws(line.to_string())
        }
    }
}

fn main() {
    let file = load_file(&String::from("input.txt"));
    let lines: Vec<&str> = file.split("\n").collect();
    let mut games = Vec::<Game>::new();
    let mut game: Game;

    for line in lines {
        game = Game::from_line(&line.to_string());
        
        if game.id != -1 {
            games.push(game); 
        }
    }

    let mut id_sum: i32 = 0;

    for game in &games {
        if game_possible(&game, 12, 13, 14) {
            id_sum += game.id;
        } 
    }

    println!("Part One - ID sum: {}", id_sum);
    
    let mut power_sum: i32 = 0;
    let mut min_draw: Draw;

    for game in games {
        min_draw = get_minimum_draw(&game);
        power_sum += min_draw.get_power();
    }

    println!("Part Two - Power sum: {}", power_sum);
}
