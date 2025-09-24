use crate::game::{Cell, CellState, GameState, Minesweeper};
use eframe::egui::{self, Align, Color32, FontId, Pos2, Rect, RichText, Sense, Stroke, Vec2};

// Define a color palette for the UI
const BACKGROUND_COLOR: Color32 = Color32::from_rgb(30, 30, 30);
const CELL_COVERED_COLOR: Color32 = Color32::from_rgb(70, 70, 70);
const CELL_REVEALED_COLOR: Color32 = Color32::from_rgb(50, 50, 50);
const CELL_HOVER_COLOR: Color32 = Color32::from_rgba_premultiplied(255, 255, 255, 20);
const GOLD_COLOR: Color32 = Color32::from_rgb(212, 175, 55); // A nice gold color for victory

impl eframe::App for Minesweeper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        self.update_timer();

        // --- Header Panel (Top) ---
        egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            ui.set_height(50.0);
            ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                ui.add_space(10.0);
                // Mines Left Display
                ui.label(
                    RichText::new(format!("🚩 {:03}", self.mines_left))
                        .font(FontId::monospace(32.0))
                        .color(Color32::RED),
                );

                // Centered New Game Button and Level Display
                ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
                    let face = match self.game_state {
                        GameState::Playing => "😊",
                        GameState::Won => "😎",
                        GameState::Lost => "😵",
                    };
                    if ui.button(RichText::new(face).font(FontId::proportional(32.0))).clicked() {
                        self.new_game();
                    }
                    ui.label(RichText::new(format!("Level {}", self.level)).strong());
                });

                // Right-aligned Timer
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(10.0);
                    ui.label(
                        RichText::new(format!("⏱ {:03}", self.elapsed_time as u32))
                            .font(FontId::monospace(32.0))
                            .color(Color32::LIGHT_GREEN),
                    );
                });
            });
        });

        // --- Main Game Board Panel ---
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(BACKGROUND_COLOR))
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    let cell_size = 35.0;
                    let board_size =
                        Vec2::new(self.width as f32 * cell_size, self.height as f32 * cell_size);
                    let (response, painter) =
                        ui.allocate_painter(board_size, Sense::click()); // Changed to Sense::click

                    let board_rect = response.rect;
                    let hover_pos = response.hover_pos();

                    for y in 0..self.height {
                        for x in 0..self.width {
                            let cell_rect = Rect::from_min_size(
                                Pos2::new(
                                    board_rect.min.x + x as f32 * cell_size,
                                    board_rect.min.y + y as f32 * cell_size,
                                ),
                                Vec2::splat(cell_size),
                            );

                            let is_hovered = hover_pos.map_or(false, |p| cell_rect.contains(p));
                            self.draw_cell(&painter, cell_rect, &self.board[x][y], is_hovered);

                            // --- FIXED INPUT LOGIC ---
                            // Only process clicks if the game is in the 'Playing' state.
                            if is_hovered && matches!(self.game_state, GameState::Playing) {
                                // Use the high-level `clicked()` and `secondary_clicked()` methods.
                                if response.clicked() {
                                    self.handle_left_click(x, y);
                                } else if response.secondary_clicked() {
                                    self.handle_right_click(x, y);
                                }
                            }
                        }
                    }
                });
            });

        ctx.request_repaint();
    }
}

impl Minesweeper {
    fn draw_cell(&self, painter: &egui::Painter, rect: Rect, cell: &Cell, is_hovered: bool) {
        let mut final_state = cell.state;
        if matches!(self.game_state, GameState::Lost | GameState::Won) && cell.has_mine {
            final_state = CellState::Revealed;
        }

        let bg_color = match final_state {
            CellState::Covered | CellState::Flagged => CELL_COVERED_COLOR,
            CellState::Revealed => {
                if matches!(self.game_state, GameState::Won) && !cell.has_mine {
                    GOLD_COLOR
                } else {
                    CELL_REVEALED_COLOR
                }
            }
        };

        painter.rect_filled(rect, 5.0, bg_color);

        if is_hovered && final_state == CellState::Covered {
            painter.rect_filled(rect, 5.0, CELL_HOVER_COLOR);
        }

        painter.rect_stroke(rect, 5.0, Stroke::new(2.0, BACKGROUND_COLOR));

        let content: Option<(String, Color32)> = match final_state {
            CellState::Revealed => {
                if cell.has_mine {
                    Some(("💣".to_string(), Color32::WHITE))
                } else if cell.adjacent_mines > 0 {
                    let color = match cell.adjacent_mines {
                        1 => Color32::from_rgb(0, 150, 255),
                        2 => Color32::from_rgb(0, 180, 0),
                        3 => Color32::from_rgb(255, 50, 50),
                        4 => Color32::from_rgb(0, 0, 180),
                        5 => Color32::from_rgb(180, 0, 0),
                        _ => Color32::from_rgb(255, 165, 0),
                    };
                    Some((cell.adjacent_mines.to_string(), color))
                } else {
                    None
                }
            }
            CellState::Flagged => Some(("🚩".to_string(), Color32::RED)),
            _ => None,
        };

        if let Some((text, color)) = content {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                FontId::proportional(22.0),
                color,
            );
        }
    }
}