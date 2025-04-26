# Sudoku-rs

A command-line Sudoku game implemented in Rust.

## Features

- Play Sudoku in your terminal with a beautiful ASCII interface
- Three difficulty levels: Easy, Medium, and Hard
- Helpful hints when you get stuck
- Color-coded board for better readability
- Simple command-based interaction

## Installation

Make sure you have Rust and Cargo installed. If not, you can install them from [rustup.rs](https://rustup.rs/).

```bash
# Clone the repository
git clone https://github.com/yourusername/sudoku-rs.git
cd sudoku-rs

# Build and run the game
cargo run --release
```

## How to Play

- The game displays a 9x9 Sudoku grid
- Fill the grid so that every row, column, and 3x3 box contains the digits 1-9
- Blue numbers are fixed (given at the start)
- Green numbers are your entries

### Commands

- `[ROW][COL] [VALUE]` - Place a value in a cell (e.g., `A1 5` places 5 in the top-left cell)
- `hint` or `h` - Get a hint (reveals a random cell)
- `new [1-3]` - Start a new game with difficulty level (1=Easy, 2=Medium, 3=Hard)
- `help` or `?` - Show help message
- `quit` or `q` - Exit the game

## License

MIT 