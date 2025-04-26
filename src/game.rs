use crate::board::Board;
use colored::*;
use std::io::{self, Write};
use std::collections::HashSet;

const MAX_HINTS: u8 = 3;

pub struct Game {
    board: Board,
    moves: u32,
    score: i32,
    completed_rows: HashSet<usize>,
    completed_cols: HashSet<usize>,
    completed_boxes: HashSet<usize>,
    hints_used: u8,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(2), // Medium difficulty by default
            moves: 0,
            score: 0,
            completed_rows: HashSet::new(),
            completed_cols: HashSet::new(),
            completed_boxes: HashSet::new(),
            hints_used: 0,
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
                        
                        // Award points for correct move
                        self.award_points(row, col, value);
                        
                        if self.board.is_solved() {
                            // Add bonus points for completing the board
                            self.score += 9;
                            self.display();
                            println!("{}", "Congratulations! You solved the puzzle!".green().bold());
                            println!("Total moves: {}", self.moves);
                            println!("Final score: {}", self.score);
                            running = false;
                        }
                    } else {
                        // Penalize for incorrect move
                        self.score -= 1;
                        println!("{}", "Invalid move! Try again. (-1 point)".red());
                    }
                },
                Command::Hint => {
                    if self.hints_used >= MAX_HINTS {
                        println!("{} No more hints available! You've used all {} hints.", 
                            "⚠️".yellow(), 
                            MAX_HINTS);
                    } else if let Some((row, col, value)) = self.board.hint() {
                        self.hints_used += 1;
                        println!("{} Hint: {} at {}{}  (Hints remaining: {})", 
                            "💡".yellow(), 
                            value, 
                            (b'A' + row as u8) as char, 
                            col + 1,
                            MAX_HINTS - self.hints_used);
                    } else {
                        println!("No more cells to reveal!");
                    }
                },
                Command::New(difficulty) => {
                    self.board = Board::new(difficulty);
                    self.moves = 0;
                    self.score = 0;
                    self.hints_used = 0;
                    self.completed_rows.clear();
                    self.completed_cols.clear();
                    self.completed_boxes.clear();
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
        println!("Moves: {}   Score: {}   Hints: {}/{}", 
            self.moves, 
            self.score, 
            self.hints_used,
            MAX_HINTS);
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
        
        println!("{}", "Scoring:".yellow().bold());
        println!("• +1 point for each correct entry");
        println!("• -1 point for each incorrect entry or invalid move");
        println!("• +3 points for completing a row");
        println!("• +3 points for completing a column");
        println!("• +3 points for completing a 3×3 box");
        println!("• +9 points bonus for completing the entire board\n");
        
        println!("{}", "Commands:".yellow().bold());
        println!("• To place a number: [ROW][COL] [VALUE]  (e.g., 'A1 5' puts 5 in the top-left cell)");
        println!("• hint or h: Get a hint (reveals a random cell) - limited to {} per game", MAX_HINTS);
        println!("• new [1-3]: Start a new game with difficulty level (1=Easy, 2=Medium, 3=Hard)");
        println!("• help or ?: Show this help message");
        println!("• quit or q: Exit the game\n");
        
        print!("Press Enter to continue...");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }

    fn award_points(&mut self, row: usize, col: usize, value: u8) {
        // Get the current state of the board from the solution
        let is_correct = self.board.is_correct_value(row, col, value);
        
        if is_correct {
            // +1 for correct placement
            self.score += 1;
            
            // Check if a row has been completed
            if self.is_row_complete(row) && !self.completed_rows.contains(&row) {
                self.score += 3;
                self.completed_rows.insert(row);
                println!("{} Row {} completed! (+3 points)", "🎉".green(), (b'A' + row as u8) as char);
            }
            
            // Check if a column has been completed
            if self.is_column_complete(col) && !self.completed_cols.contains(&col) {
                self.score += 3;
                self.completed_cols.insert(col);
                println!("{} Column {} completed! (+3 points)", "🎉".green(), col + 1);
            }
            
            // Check if a 3x3 box has been completed
            let box_idx = (row / 3) * 3 + (col / 3);
            if self.is_box_complete(row, col) && !self.completed_boxes.contains(&box_idx) {
                self.score += 3;
                self.completed_boxes.insert(box_idx);
                println!("{} 3x3 box completed! (+3 points)", "🎉".green());
            }
        } else {
            // -1 for incorrect value
            self.score -= 1;
        }
    }
    
    fn is_row_complete(&self, row: usize) -> bool {
        for col in 0..crate::board::SIZE {
            if self.board.get_cell_value(row, col) == 0 {
                return false;
            }
        }
        true
    }
    
    fn is_column_complete(&self, col: usize) -> bool {
        for row in 0..crate::board::SIZE {
            if self.board.get_cell_value(row, col) == 0 {
                return false;
            }
        }
        true
    }
    
    fn is_box_complete(&self, row: usize, col: usize) -> bool {
        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        
        for r in box_row..box_row + 3 {
            for c in box_col..box_col + 3 {
                if self.board.get_cell_value(r, c) == 0 {
                    return false;
                }
            }
        }
        true
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