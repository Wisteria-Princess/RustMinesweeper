# Rust Minesweeper

A complete, playable Minesweeper game built with the Rust programming language and the `egui` immediate mode GUI library. This project provides a modern take on the classic game, featuring a clean dark-mode UI, progressive difficulty, and a polished user experience.


## Download & Play (for Windows)

The easiest way to play is to download the latest pre-compiled executable from the GitHub Releases page.

**[➡️ Download the latest release here ⬅️]([https://github.com/YOUR_USERNAME/YOUR_REPOSITORY/releases/](https://github.com/Wisteria-Princess/RustMinesweeper/releases/tag/v1.0.1)**

Simply download the `minesweeper_rust.exe` file from the latest release, save it anywhere on your computer, and double-click to play. No installation is needed!

---

## Features

-   **Classic Minesweeper Gameplay**: Left-click to reveal tiles and right-click to flag potential mines.
-   **Modern UI**: A visually appealing dark theme built with `egui`.
-   **Dynamic Game Status Header**: Includes a mine counter, an in-game timer, and a smiley-face reset button that changes based on game state (playing, won, lost).
-   **Progressive Difficulty**:
    -   Beat a level to advance to the next.
    -   The number of mines **doubles** after each victory, offering an increasing challenge.
    -   Losing a game resets the difficulty to Level 1.
-   **Golden Victory Screen**: Successfully clearing a minefield turns the board a satisfying gold color.
-   **Auto-Sizing Window**: The application window launches with the perfect size to fit the game board, and is non-resizable for a clean look.
-   **Safe Mine Generation**: The mine generation algorithm includes a cap to prevent an impossible-to-win board where every tile is a mine.

## Building from Source

If you are a developer and want to compile the project yourself, follow these steps.

### Prerequisites

Ensure you have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed on your system. This will provide you with `rustc` (the compiler) and `cargo` (the package manager and build tool).

### Instructions

1.  **Clone the Repository:**
    ```sh
    git clone https://github.com/YOUR_USERNAME/YOUR_REPOSITORY.git
    cd YOUR_REPOSITORY
    ```

2.  **Build and Run in Debug Mode:**
    Use `cargo run` to compile and run the application. Cargo will automatically download all the necessary dependencies.
    ```sh
    cargo run
    ```

## Creating Your Own Executable

To create your own optimized, standalone `.exe` file from the source code, you can build the project in *release mode*.

1.  **Hide the Console Window**: Ensure the line `#![windows_subsystem = "windows"]` is at the very top of your `src/main.rs` file. This attribute prevents a console window from appearing behind the game.

2.  **Run the Release Build Command**: Open your terminal in the project's root directory and run:
    ```sh
    cargo build --release
    ```

3.  **Find the Executable**: Your optimized `.exe` file will be located in the `target/release/` directory (e.g., `target/release/minesweeper_rust.exe`). You can copy this single file and run it on any modern Windows computer.

## Project Structure

-   `src/main.rs`: The application's entry point. Configures and launches the `eframe` window.
-   `src/game.rs`: Contains all the core game logic, completely decoupled from the UI.
-   `src/ui.rs`: Handles all rendering and user input via the `egui` library.
-   `Cargo.toml`: The project's manifest file, defining metadata and dependencies.

## Key Dependencies

-   [**`eframe`**](https://crates.io/crates/eframe): The official application framework for `egui`.
-   [**`egui`**](https://crates.io/crates/egui): A powerful and easy-to-use immediate mode GUI library for Rust.
-   [**`rand`**](https://crates.io/crates/rand): Used for randomly placing mines on the game board.
