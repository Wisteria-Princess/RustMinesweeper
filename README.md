# Rust Minesweeper

A complete, playable Minesweeper game built with the Rust programming language and the `egui` immediate mode GUI library. This project provides a modern take on the classic game, featuring a clean dark-mode UI, progressive difficulty, and a polished user experience.

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

## Prerequisites

Before you begin, ensure you have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed on your system. This will provide you with `rustc` (the compiler) and `cargo` (the package manager and build tool).

## How to Run the Project

1.  **Clone the Repository (or download the files):**
    ```sh
    git clone <your-repository-url>
    cd rust-minesweeper
    ```
    Alternatively, create a new Cargo project (`cargo new minesweeper_rust`) and place the provided source files (`main.rs`, `game.rs`, `ui.rs`, `Cargo.toml`) in the correct directories.

2.  **Build and Run:**
    Use `cargo` to compile and run the application. Cargo will automatically download and compile all the necessary dependencies.
    ```sh
    cargo run
    ```

    The game window should appear, and you can start playing immediately.

## Project Structure

The project is organized into three main Rust files to separate concerns:

-   `src/main.rs`: The entry point of the application. It is responsible for setting up the game configuration (board size, mine count), calculating the initial window size, and launching the `eframe` application.

-   `src/game.rs`: Contains all the core game logic, completely decoupled from the UI. It manages the game state (`Playing`, `Won`, `Lost`), the board data structure, mine placement, cell revealing logic, win/loss conditions, and the level progression system.

-   `src/ui.rs`: Handles all rendering and user input. It implements the `eframe::App` trait for our `Minesweeper` struct, drawing the game board, header, and responding to mouse clicks.

-   `Cargo.toml`: The project's manifest file. It defines project metadata and lists all the external crates (dependencies) required to build the game.

## Key Dependencies

-   [**`eframe`**](https://crates.io/crates/eframe): The official application framework for `egui`. It handles the backend windowing, event loop, and rendering pipeline, allowing us to focus on the UI.
-   [**`egui`**](https://crates.io/crates/egui): A powerful and easy-to-use immediate mode GUI library for Rust. Used to draw every part of the user interface.
-   [**`rand`**](https://crates.io/crates/rand): A Rust library for random number generation, used here to randomly place mines on the game board.
