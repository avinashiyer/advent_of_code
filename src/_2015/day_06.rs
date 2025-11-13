use std::fmt::Display;

const ARR_SIZE: usize = 1000;
struct Board {
    mat: [[u8; ARR_SIZE]; ARR_SIZE],
}

struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn n(x: usize, y: usize) -> Self {
        Coord { x, y }
    }
}

enum Operation {
    On,
    Off,
    Toggle,
}

impl Operation {
    fn op(&self, tgt: u8) -> u8 {
        match self {
            Operation::On => tgt.checked_add(1).unwrap(),
            Operation::Off => {
                if tgt > 0 {
                    tgt - 1
                } else {
                    0
                }
            }
            Operation::Toggle => tgt.checked_add(2).unwrap(),
        }
    }

    fn new(tgt: &str) -> Self {
        match tgt {
            "toggle" => Operation::Toggle,
            "turn off" => Operation::Off,
            "turn on" => Operation::On,
            _ => panic!(),
        }
    }
}

impl Board {
    fn new() -> Self {
        let mat = [[0; ARR_SIZE]; ARR_SIZE];
        Board { mat }
    }

    fn select_and_alter(&mut self, first_corner: Coord, second_corner: Coord, command: Operation) {
        let mut x_pos = first_corner.x;
        while x_pos <= second_corner.x {
            let mut y_pos = first_corner.y;
            while y_pos <= second_corner.y {
                self.mat[x_pos][y_pos] = command.op(self.mat[x_pos][y_pos]);
                y_pos += 1;
            }
            x_pos += 1;
        }
    }
}
use regex::Regex;

pub fn christmas_lights(src: &str) {
    let mut board = Board::new();
    let re = Regex::new(r"(toggle|turn off|turn on) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut commands: Vec<(Operation, Coord, Coord)> = vec![];
    for (_, [com, x1, y1, x2, y2]) in re.captures_iter(src).map(|c| c.extract()) {
        commands.push((
            Operation::new(com),
            Coord::n(x1.parse::<usize>().unwrap(), y1.parse::<usize>().unwrap()),
            Coord::n(x2.parse::<usize>().unwrap(), y2.parse::<usize>().unwrap()),
        ))
    }
    for command in commands {
        board.select_and_alter(command.1, command.2, command.0);
    }
    let mut counter = 0usize;
    for r in board.mat {
        for c in r {
            counter += usize::from(c);
        }
    }
    println!("counter:{counter}");
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "START").unwrap();
        for row in self.mat {
            for col in row {
                if col > 0 {
                    write!(f, "* ").unwrap();
                } else {
                    write!(f, "0 ").unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        writeln!(f, "END")
    }
}
