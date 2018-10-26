use std::fmt::{Display, Error, Formatter};
use std::io;
use Tile::*;

fn main() {
    let mut board: Board = Board {
        tiles: [
            [Blank, Blank, Blank],
            [Blank, Blank, Blank],
            [Blank, Blank, Blank],
        ],
        turn: Black,
    };

    let game_end = false;

    while !game_end {
        print_board(&board);
        println!("{}'s turn!", &board.turn);
        print!("What is your next move?: ");
        io::Write::flush(&mut io::stdout()).expect("error flushing stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error reading text");
        let n: usize = input.trim().parse().unwrap_or(0);

        if n >= 1 && n <= 9 {
            if let Some(_) = &mut board.set(n) {
                let msg = match board.winner() {
                    None => continue,
                    Some(w) => match (w) {
                        Black => "X win!",
                        White => "O win!",
                        Blank => "Draw!",
                    },
                };
                println!("{}", msg);
                break;
            }
        }
        println!("Retry!!");
    }
}

fn print_board(board: &Board) {
    println!("+---+---+---+");
    for i in 0..3 {
        println!(
            "| {} | {} | {} |",
            board.tiles[i][0], board.tiles[i][1], board.tiles[i][2]
        );
        println!("+---+---+---+");
    }
}

struct Board {
    tiles: [[Tile; 3]; 3],
    turn: Tile,
}

impl Board {
    fn set(&mut self, pos: usize) -> Option<()> {
        let pos = pos - 1;
        let x = pos % 3;
        let y = pos / 3;

        if let Blank = self.tiles[y][x] {
            self.tiles[y][x] = self.turn;
            self.turn = match self.turn {
                Black => White,
                White => Black,
                _ => panic!("wtf"),
            };
            return Some(());
        }

        None
    }

    fn winner(&self) -> Option<Tile> {
        let mut full = true;
        'outer: for i in 0..3 {
            for j in 0..3 {
                if let Blank = self.tiles[j][i] {
                    full = false;
                    break 'outer;
                }
            }
        }
        if full {
            return Some(Blank);
        }

        None
    }
}

#[derive(Copy, Clone)]
enum Tile {
    Blank,
    White,
    Black,
}

enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let c = match self {
            Blank => " ",
            White => "O",
            Black => "X",
        };
        write!(f, "{}", c)
    }
}
