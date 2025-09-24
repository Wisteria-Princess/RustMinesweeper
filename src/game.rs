use rand::Rng;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Covered,
    Revealed,
    Flagged,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub has_mine: bool,
    pub state: CellState,
    pub adjacent_mines: u8,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            has_mine: false,
            state: CellState::Covered,
            adjacent_mines: 0,
        }
    }
}

pub enum GameState {
    Playing,
    Won,
    Lost,
}

pub struct Minesweeper {
    pub board: Vec<Vec<Cell>>,
    pub game_state: GameState,
    pub start_time: Option<Instant>,
    pub elapsed_time: f32,
    pub mines_left: i32,
    pub width: usize,
    pub height: usize,
    mines: usize,
    base_mines: usize,
    pub level: usize,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        let mut game = Self {
            board: vec![vec![Cell::new(); height]; width],
            game_state: GameState::Playing,
            start_time: None,
            elapsed_time: 0.0,
            mines_left: mines as i32,
            width,
            height,
            mines,
            base_mines: mines, // Store the starting mine count
            level: 1,          // Start at level 1
        };
        game.generate_board();
        game
    }

    fn generate_board(&mut self) {
        let mut rng = rand::thread_rng();
        let mut mines_placed = 0;

        while mines_placed < self.mines {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);

            if !self.board[x][y].has_mine {
                self.board[x][y].has_mine = true;
                mines_placed += 1;
            }
        }

        for x in 0..self.width {
            for y in 0..self.height {
                if !self.board[x][y].has_mine {
                    self.board[x][y].adjacent_mines = self.count_adjacent_mines(x, y);
                }
            }
        }
    }
    
    pub fn new_game(&mut self) {
        // --- Progression Logic ---
        // Check the state of the game we just finished.
        match self.game_state {
            GameState::Won => {
                // If won, advance to the next level and double the mines.
                self.level += 1;
                self.mines *= 2;
            }
            GameState::Lost => {
                // If lost, reset to Level 1 difficulty.
                self.level = 1;
                self.mines = self.base_mines;
            }
            GameState::Playing => {
                // If the player restarts mid-game, just restart the current level.
            }
        }

        // Safety Cap: Ensure mines don't fill the entire board.
        // We leave at least 9 empty cells as a safe starting area.
        let max_mines = self.width * self.height - 9;
        if self.mines > max_mines {
            self.mines = max_mines;
        }

        // --- Board Reset ---
        self.board = vec![vec![Cell::new(); self.height]; self.width];
        self.game_state = GameState::Playing;
        self.start_time = None;
        self.elapsed_time = 0.0;
        self.mines_left = self.mines as i32;
        self.generate_board();
    }

    fn count_adjacent_mines(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let nx = x as i32 + i;
                let ny = y as i32 + j;
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    if self.board[nx as usize][ny as usize].has_mine {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn handle_left_click(&mut self, x: usize, y: usize) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
        if self.board[x][y].state == CellState::Covered {
            self.reveal_cell(x, y);
        }
    }

    fn reveal_cell(&mut self, x: usize, y: usize) {
        if self.board[x][y].state != CellState::Covered {
            return;
        }
        self.board[x][y].state = CellState::Revealed;
        if self.board[x][y].has_mine {
            self.game_state = GameState::Lost;
            return;
        }
        if self.board[x][y].adjacent_mines == 0 {
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let nx = x as i32 + i;
                    let ny = y as i32 + j;
                    if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                        self.reveal_cell(nx as usize, ny as usize);
                    }
                }
            }
        }
        self.check_win_condition();
    }

    pub fn handle_right_click(&mut self, x: usize, y: usize) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
        match self.board[x][y].state {
            CellState::Covered => {
                self.board[x][y].state = CellState::Flagged;
                self.mines_left -= 1;
            }
            CellState::Flagged => {
                self.board[x][y].state = CellState::Covered;
                self.mines_left += 1;
            }
            _ => (),
        }
    }

    fn check_win_condition(&mut self) {
        let covered_cells = self
            .board
            .iter()
            .flatten()
            .filter(|c| c.state == CellState::Covered || c.state == CellState::Flagged)
            .count();

        if covered_cells == self.mines {
            self.game_state = GameState::Won;
        }
    }

    pub fn update_timer(&mut self) {
        if let Some(start_time) = self.start_time {
            if matches!(self.game_state, GameState::Playing) {
                self.elapsed_time = start_time.elapsed().as_secs_f32();
            }
        }
    }
}