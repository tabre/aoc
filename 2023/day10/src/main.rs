use std::fs;
use std::fs::File;
use std::io::Write;

use regex::Regex;

fn load_file(file_name: &str) -> String {
    let file_parse = fs::read_to_string(file_name);

    if file_parse.is_ok() {
       return file_parse.unwrap(); 
    } else {
        panic!("Error reading file: {}", file_name);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)] enum Dir {
    North,
    South,
    East,
    West
}

impl Dir {
    fn get_offset(&self) -> (i32, i32) {
        match self {
            Dir::North => (0, -1),
            Dir::South => (0, 1),
            Dir::East => (1, 0),
            Dir::West => (-1, 0),
        }
    }

    fn offset(&self, pos: (i32, i32)) -> (usize, usize) {
        let (x1, y1) = pos;
        let (x2, y2) = self.get_offset();

        return ((x1 + x2).try_into().unwrap(), (y1 + y2).try_into().unwrap());        
    }

    fn opposite(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East
        }
    }
}

#[derive(Debug, Clone)]
enum PipeType {
    Vert,
    Horiz,
    EastNorth,
    WestNorth,
    WestSouth,
    EastSouth
}

impl PipeType {
    fn from(c: char) -> Option<PipeType> {
        match c {
            '|' => Some(PipeType::Vert),
            '-' => Some(PipeType::Horiz),
            'L' => Some(PipeType::EastNorth),
            'J' => Some(PipeType::WestNorth),
            '7' => Some(PipeType::WestSouth),
            'F' => Some(PipeType::EastSouth),
            _ => None::<PipeType>           
        } 
    }

    fn get_dirs(&self) -> (Dir, Dir) {
        match self {
            PipeType::Vert => (Dir::North, Dir::South),
            PipeType::Horiz => (Dir::West, Dir::East),
            PipeType::EastNorth => (Dir::East, Dir::North),
            PipeType::WestNorth => (Dir::West, Dir::North),
            PipeType::WestSouth => (Dir::West, Dir::South),
            PipeType::EastSouth => (Dir::East, Dir::South)
        }
    }

    fn get_other(&self, dir: &Dir) -> Dir{
        let dirs = self.get_dirs();

        if dirs.0 == *dir {
            return dirs.1
        }

        return dirs.0;
    }
}

#[derive(Debug, Clone)]
struct Pipe {
    typ: PipeType,
    pos: (i32, i32),
    enter: Dir,
}

impl Pipe {
    fn from(c: char, pos: (usize, usize), traveling: &Dir) -> Option<Pipe> {
        let typ: PipeType;
        let result = PipeType::from(c);

        if result.is_some() {
            typ = PipeType::from(c).unwrap();
        } else {
            println!(
                "Invalid character for Pipe: [{}] Line: {}, Col: {}",
                c, pos.1 + 1, pos.0 + 1
            );
            return None::<Pipe>;
        }

        let x: i32 = pos.0 as i32; 
        let y: i32 = pos.1 as i32;

        let pipe = Pipe {
            typ,
            pos: (x, y),
            enter: traveling.opposite(),
        };

        return Some(pipe);
    }

    fn get_exit(&self) -> Dir {
        self.typ.get_other(&self.enter)
    }

    fn get_next_pos(&self) -> (usize, usize) {
        let exit = self.get_exit();
        return exit.offset(self.pos);
    }
}

fn get_start_pos(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                return (x, y);
            }
        }
    }
    return (0, 0);
}

fn get_start_pipe(grid: &Vec<Vec<char>>, start_pos: (usize, usize)) -> Pipe {
    let mut temp_pipe: Pipe;
    let mut dirs: (Dir, Dir);
    let mut x: usize;
    let mut y: usize;
    let mut result: Option<Pipe>;
    
    for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
        (x, y) = dir.offset((start_pos.0 as i32, start_pos.1 as i32));
        
        result = Pipe::from(grid[y][x], (x, y), &dir);
        if result.is_some() {
            temp_pipe = result.unwrap();
        } else {
            continue;
        }

        dirs = temp_pipe.typ.get_dirs();

        if dirs.0.opposite() == dir || dirs.1.opposite() == dir {
            return temp_pipe;
        }
    }

    panic!("No start pipe found.");
}

fn get_pipe_positions(pipes: Vec<Pipe>) -> Vec<(usize, usize)> {
    let mut pipe_pos = Vec::<(usize, usize)>::new();
    for pipe in pipes {
        pipe_pos.push((pipe.pos.0 as usize, pipe.pos.1 as usize));
    }
    return pipe_pos;
}

fn grid_replace(
    grid: &mut Vec<Vec<char>>,
    start_pos: (usize, usize),
    poss: &mut Vec<(usize, usize)>,
    c: char
) {
    poss.push(start_pos);
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if !poss.contains(&(x, y)) && grid[y][x] != '.' {
                grid[y][x] = c;
            }                 
        }
    }
}

fn grid_to_file(grid: &Vec<Vec<char>>, file_name: &str) {
    let mut line = String::from(""); 
    let mut buffer = File::create(file_name).unwrap();
    
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            line.push(grid[y][x]);             
        }
        line.push('\n');
        let _ = buffer.write(&line.as_bytes());
        line = String::from("");
    }    
}

fn main() {
    let file = load_file("input.txt");
    let lines: Vec<&str> = file.split("\n").collect();
    let mut grid = Vec::<Vec::<char>>::new();
    
    for line in &lines {
        grid.push(line.chars().collect());
    }
    
    let start_pos = get_start_pos(&grid);
    let mut curr_pipe = get_start_pipe(&grid, start_pos);
    let mut x: usize;
    let mut y: usize;
    let mut pipes = Vec::<Pipe>::new();

    loop {
        pipes.push(curr_pipe.clone());
        (x, y) = curr_pipe.get_next_pos();
        curr_pipe = Pipe::from(
            grid[y][x],
            (x, y),
            &curr_pipe.get_exit()
        ).unwrap();

        if curr_pipe.get_next_pos() == start_pos {
            pipes.push(curr_pipe.clone());
            break;
        }
    }

    println!(
        "Part 1 - The far the point from start is {} steps away.",
        (pipes.len() + 1) / 2
    );
    
    grid[start_pos.1][start_pos.0] = '|';
    grid_replace(&mut grid, start_pos, &mut get_pipe_positions(pipes), '.');
    
    let regexes = [
       (Regex::new(r"L-*7").unwrap(), "|".to_string()),
       (Regex::new(r"L-*J").unwrap(), "||".to_string()),
       (Regex::new(r"F-*7").unwrap(), "||".to_string()),
       (Regex::new(r"F-*J").unwrap(), "|".to_string()),
    ];

    let mut fixed_line: String;
    let mut new_grid = Vec::<Vec<char>>::new();

    for line in &grid {
        fixed_line = String::from_iter(line);
        for (reg, rep) in &regexes {
            fixed_line = reg.replace_all(&fixed_line, rep).to_string();
        }
        new_grid.push(fixed_line.chars().collect());
    }
 

    grid_to_file(&new_grid, "pipemap.txt");

    let mut tile: char;
    let mut cross: i32 = 0;
    let mut inside: i32 = 0;

    for y in 0..new_grid.len() {
        for x in 0..new_grid[y].len() {
            tile = new_grid[y][x];

            if tile == '.' && !cross % 2 == 0 {
                inside += 1;                
            } else if ['F', '7', 'L', 'J', '|'].contains(&tile) {
                cross += 1;
            }

        }
        cross = 0;
    }

    println!("Part 2 - The total number of inside tiles is {}.", inside);
}
