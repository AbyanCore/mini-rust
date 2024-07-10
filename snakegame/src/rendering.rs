// rendering.rs
use crate::entities::{Board, CellState};
use std::process::Command;

pub fn render(board: &Board) {
    clear_screen();

    for row in board.cells() {
        for cell in row {
            print!("{}", cell_to_char(cell.clone()));
        }
        println!();
    }
}

fn cell_to_char(cell: CellState) -> &'static str {
    match cell {
        CellState::Empty => "⬜",
        CellState::Food => "🍎",
        CellState::SnakeHead => "🟢",
        CellState::SnakeBody => "🟩",
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}
