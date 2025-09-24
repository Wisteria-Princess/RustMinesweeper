mod game;
mod ui;

use eframe::{egui, NativeOptions};
use game::Minesweeper;

// Define constants for game and UI configuration.
const BOARD_WIDTH: usize = 16;
const BOARD_HEIGHT: usize = 16;
const MINE_COUNT: usize = 40;

// UI constants used for window size calculation.
const HEADER_HEIGHT: f32 = 50.0;
const CELL_SIZE: f32 = 35.0;

fn main() {
    // Calculate the total required width and height for the inner content of the window.
    let window_width = BOARD_WIDTH as f32 * CELL_SIZE;
    let window_height = (BOARD_HEIGHT as f32 * CELL_SIZE) + HEADER_HEIGHT;
    
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // Set the initial inner size of the window.
            .with_inner_size([window_width, window_height])
            // Prevent the user from resizing the window.
            .with_resizable(false)
            .with_title("Minesweeper"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Minesweeper",
        options,
        Box::new(|_cc| Box::new(Minesweeper::new(BOARD_WIDTH, BOARD_HEIGHT, MINE_COUNT))),
    )
        .unwrap();
}