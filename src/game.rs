use crate::board::Board;
use colored::*;
use std::io::{self, Write};

pub struct Game {
    board: Board,
    moves: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(2), // Medium difficulty by default
            moves: 0,
        }
    }
    
    pub fn run(&mut self) {
        let mut running = true;
        
        while running {
            self.display();
            
            match self.get_command() {
                Command::Place(row, col, value) => {
                    if self.board.set(row, col, value) {
                        self.moves += 1;
                        
                        if self.board.is_solved() {
                            self.display();
                            println!("{}", "Congratulations! You solved the puzzle!".green().bold());
                            println!("Total moves: {}", self.moves);
                            running = false;
                        }
                    } else {
                        println!("{}", "Invalid move! Try again.".red());
                    }
                },
                Command::Hint => {
                    if let Some((row, col, value)) = self.board.hint() {
                        println!("{} Hint: {} at {}{}", 
                            "💡".yellow(), 
                            value, 
                            (b'A' + row as u8) as char, 
                            col + 1);
                    } else {
                        println!("No more hints available!");
                    }
                },
                Command::New(difficulty) => {
                    self.board = Board::new(difficulty);
                    self.moves = 0;
                    println!("Started a new game with difficulty level {}", difficulty);
                },
                Command::Quit => {
                    println!("Thanks for playing!");
                    running = false;
                },
                Command::Help => {
                    self.show_help();
                },
                Command::Unknown => {
                    println!("{}", "Unknown command. Type 'help' for instructions.".yellow());
                },
            }
        }
    }
    
    fn display(&self) {
        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");
        
        println!("{}", "SUDOKU".green().bold());
        println!("{}", "======".green());
        println!();
        println!("{}", self.board);
        println!();
        println!("Moves: {}", self.moves);
        println!();
        println!("Commands: [A-I][1-9] [1-9] | hint | new [1-3] | help | quit");
        println!();
    }
    
    fn get_command(&self) -> Command {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        
        if input == "quit" || input == "q" {
            return Command::Quit;
        } else if input == "hint" || input == "h" {
            return Command::Hint;
        } else if input == "help" || input == "?" {
            return Command::Help;
        } else if input.starts_with("new") {
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.len() > 1 {
                if let Ok(difficulty) = parts[1].parse::<u8>() {
                    if difficulty >= 1 && difficulty <= 3 {
                        return Command::New(difficulty);
                    }
                }
            }
            // Default to medium difficulty
            return Command::New(2);
        } else {
            // Try to parse as a move: [A-I][1-9] [1-9]
            let parts: Vec<&str> = input.split_whitespace().collect();
            
            if parts.len() == 2 && parts[0].len() == 2 {
                let cell = parts[0].to_uppercase();
                let mut chars = cell.chars();
                
                if let (Some(row_char), Some(col_char)) = (chars.next(), chars.next()) {
                    if row_char >= 'A' && row_char <= 'I' && col_char >= '1' && col_char <= '9' {
                        let row = (row_char as u8 - b'A') as usize;
                        let col = (col_char as u8 - b'1') as usize;
                        
                        if let Ok(value) = parts[1].parse::<u8>() {
                            if value >= 1 && value <= 9 {
                                return Command::Place(row, col, value);
                            }
                        }
                    }
                }
            }
        }
        
        Command::Unknown
    }
    
    fn show_help(&self) {
        println!("\n{}", "How to Play Sudoku".green().bold());
        println!("=================");
        println!("• Fill the 9×9 grid so each column, row, and 3×3 box contains the digits 1-9");
        println!("• Blue numbers are fixed and cannot be changed");
        println!("• Green numbers are your entries\n");
        
        println!("{}", "Commands:".yellow().bold());
        println!("• To place a number: [ROW][COL] [VALUE]  (e.g., 'A1 5' puts 5 in the top-left cell)");
        println!("• hint or h: Get a hint (reveals a random cell)");
        println!("• new [1-3]: Start a new game with difficulty level (1=Easy, 2=Medium, 3=Hard)");
        println!("• help or ?: Show this help message");
        println!("• quit or q: Exit the game\n");
        
        print!("Press Enter to continue...");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}

enum Command {
    Place(usize, usize, u8), // (row, col, value)
    Hint,
    New(u8),                 // difficulty level
    Quit,
    Help,
    Unknown,
} 