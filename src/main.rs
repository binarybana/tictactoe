use crossterm::terminal::{terminal, ClearType};
use crossterm::{cursor, input};

use derive_more::Display;

#[derive(Debug)]
struct Game {
    cells: [Cell; 9],
}

const POSSIBLE_WINS: [[u8; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

impl Game {
    fn new() -> Game {
        Game {
            cells: [Cell::Empty; 9],
        }
    }

    fn has_won(&self, who: Cell) -> bool {
        POSSIBLE_WINS
            .iter()
            .map(|indices| indices.iter().all(|i| self.cells[*i as usize] == who))
            .any(|x| x)
    }

    //TODO: put into Display or Debug trait
    fn print_board(&self) {
        for row in 0..3 {
            print!("|");
            for col in 0..3 {
                print!("{}|", self.cells[row * 3 + col]);
            }
            println!("\n-------");
        }
    }

    fn play(&mut self, index: u8, who: Cell) -> Result<(), ()> {
        if self.cells[index as usize] != Cell::Empty {
            Err(())
        } else {
            self.cells[index as usize] = who;
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Copy, Display, Eq, PartialEq)]
enum Cell {
    #[display(fmt = " ")]
    Empty,
    #[display(fmt = "X")]
    X,
    #[display(fmt = "O")]
    O,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn winning() {
        let mut g = Game::new();
        g.cells[3] = Cell::X;
        g.cells[7] = Cell::O;
        assert!(!g.has_won(Cell::O));
        assert!(!g.has_won(Cell::X));
        g.cells[3] = Cell::O;
        g.cells[4] = Cell::O;
        g.cells[5] = Cell::O;
        assert!(!g.has_won(Cell::X));
        assert!(g.has_won(Cell::O));
        g.cells[6] = Cell::X;
        g.cells[7] = Cell::X;
        g.cells[8] = Cell::X;
        assert!(g.has_won(Cell::X));
        assert!(g.has_won(Cell::O));
    }
}

fn main() {
    let input = input();
    let terminal = terminal();

    terminal.clear(ClearType::All);
    let mut g = Game::new();
    let mut player = Cell::X;
    while !g.has_won(Cell::X) && !g.has_won(Cell::O) {
        terminal.clear(ClearType::All);
        g.print_board();
        println!("Index of where you wish to play:");
        let index = match input.read_line() {
            Ok(x) => x,
            Err(_) => continue,
        };
        let index = match index.parse() {
            Ok(x @ 0..=8) => x,
            _ => continue,
        };
        match g.play(index, player) {
            Ok(_) => {}
            Err(_) => continue,
        };
        player = match player {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            _ => panic!("How?"),
        };
    }
    terminal.clear(ClearType::All);
    g.print_board();
    println!("Someone won!");
}
