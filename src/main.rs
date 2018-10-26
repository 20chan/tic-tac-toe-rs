use std::fmt::{Display, Error, Formatter};
use Space::*;

fn main() {
    let board: Board = [
        [Blank, Blank, Blank],
        [Blank, White, Blank],
        [Black, Blank, Blank],
    ];

    print_board(&board);
}

fn print_board(board: &Board) {
    println!("-------");
    for i in 0..3 {
        println!("|{}|{}|{}|", board[i][0], board[i][1], board[i][2]);
    }
    println!("-------");
}

type Board = [[Space; 3]; 3];

enum Space {
    Blank,
    White,
    Black,
}

impl Display for Space {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let c = match self {
            Blank => " ",
            White => "○",
            Black => "●",
        };
        write!(f, "{}", c)
    }
}
