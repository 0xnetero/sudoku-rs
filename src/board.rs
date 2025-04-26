use colored::*;
use rand::prelude::*;
use std::fmt;

pub const SIZE: usize = 9;
pub const BOX_SIZE: usize = 3;

pub struct Board {
    cells: [[Cell; SIZE]; SIZE],
    solution: [[u8; SIZE]; SIZE],
}

#[derive(Clone, Copy)]
pub struct Cell {
    value: u8,
    given: bool,
}

impl Board {
    pub fn new(difficulty: u8) -> Self {
        let mut board = Board {
            cells: [[Cell { value: 0, given: false }; SIZE]; SIZE],
            solution: [[0; SIZE]; SIZE],
        };
        
        board.generate();
        board.create_puzzle(difficulty);
        
        board
    }
    
    fn generate(&mut self) {
        // Generate a filled valid Sudoku board
        let mut rng = rand::thread_rng();
        
        // Start with a solved state using backtracking
        self.solve_empty_board(&mut rng);
        
        // Store the full solution
        for i in 0..SIZE {
            for j in 0..SIZE {
                self.solution[i][j] = self.cells[i][j].value;
            }
        }
    }
    
    fn solve_empty_board(&mut self, rng: &mut ThreadRng) -> bool {
        // Find an empty cell
        let mut row = 0;
        let mut col = 0;
        let mut found = false;
        
        'outer: for i in 0..SIZE {
            for j in 0..SIZE {
                if self.cells[i][j].value == 0 {
                    row = i;
                    col = j;
                    found = true;
                    break 'outer;
                }
            }
        }
        
        if !found {
            return true; // No empty cells left, puzzle is solved
        }
        
        // Try different values
        let mut values: Vec<u8> = (1..=9).collect();
        values.shuffle(rng);
        
        for &value in &values {
            if self.is_valid(row, col, value) {
                self.cells[row][col].value = value;
                
                if self.solve_empty_board(rng) {
                    return true;
                }
                
                self.cells[row][col].value = 0; // Undo if it doesn't lead to a solution
            }
        }
        
        false
    }
    
    fn create_puzzle(&mut self, difficulty: u8) {
        // Determine how many cells to remove based on difficulty (1-3)
        let cells_to_remove = match difficulty {
            1 => 30, // Easy
            2 => 45, // Medium
            _ => 55, // Hard
        };
        
        let mut rng = rand::thread_rng();
        let mut positions: Vec<(usize, usize)> = (0..SIZE)
            .flat_map(|i| (0..SIZE).map(move |j| (i, j)))
            .collect();
        positions.shuffle(&mut rng);
        
        // Copy the solution to cells
        for i in 0..SIZE {
            for j in 0..SIZE {
                self.cells[i][j] = Cell {
                    value: self.solution[i][j],
                    given: true,
                };
            }
        }
        
        // Remove cells_to_remove values
        for (i, j) in positions.iter().take(cells_to_remove) {
            self.cells[*i][*j].value = 0;
            self.cells[*i][*j].given = false;
        }
    }
    
    pub fn is_valid(&self, row: usize, col: usize, value: u8) -> bool {
        // Check row
        for j in 0..SIZE {
            if self.cells[row][j].value == value {
                return false;
            }
        }
        
        // Check column
        for i in 0..SIZE {
            if self.cells[i][col].value == value {
                return false;
            }
        }
        
        // Check 3x3 box
        let box_row = (row / BOX_SIZE) * BOX_SIZE;
        let box_col = (col / BOX_SIZE) * BOX_SIZE;
        
        for i in box_row..box_row + BOX_SIZE {
            for j in box_col..box_col + BOX_SIZE {
                if self.cells[i][j].value == value {
                    return false;
                }
            }
        }
        
        true
    }
    
    pub fn set(&mut self, row: usize, col: usize, value: u8) -> bool {
        if row >= SIZE || col >= SIZE {
            return false;
        }
        
        if self.cells[row][col].given {
            return false; // Can't modify given cells
        }
        
        // Allow erasing
        if value == 0 {
            self.cells[row][col].value = 0;
            return true;
        }
        
        if value > 9 {
            return false;
        }
        
        // Check if the move is valid
        if !self.is_valid(row, col, value) {
            return false;
        }
        
        self.cells[row][col].value = value;
        true
    }
    
    pub fn is_solved(&self) -> bool {
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.cells[i][j].value != self.solution[i][j] {
                    return false;
                }
            }
        }
        true
    }
    
    pub fn hint(&mut self) -> Option<(usize, usize, u8)> {
        let mut rng = rand::thread_rng();
        let mut empty_cells = Vec::new();
        
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.cells[i][j].value == 0 {
                    empty_cells.push((i, j));
                }
            }
        }
        
        if empty_cells.is_empty() {
            return None;
        }
        
        // Randomly select an empty cell
        let idx = rng.gen_range(0..empty_cells.len());
        let (row, col) = empty_cells[idx];
        let value = self.solution[row][col];
        
        // Set the value
        self.cells[row][col] = Cell { value, given: true };
        
        Some((row, col, value))
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "    1 2 3   4 5 6   7 8 9")?;
        writeln!(f, "  ┌───────┬───────┬───────┐")?;
        
        for i in 0..SIZE {
            write!(f, "{} │ ", (b'A' + i as u8) as char)?;
            
            for j in 0..SIZE {
                let cell = self.cells[i][j];
                
                if cell.value == 0 {
                    write!(f, ". ")?;
                } else if cell.given {
                    write!(f, "{} ", cell.value.to_string().blue().bold())?;
                } else {
                    write!(f, "{} ", cell.value.to_string().green())?;
                }
                
                if j % BOX_SIZE == BOX_SIZE - 1 && j < SIZE - 1 {
                    write!(f, "│ ")?;
                }
            }
            
            writeln!(f, "│")?;
            
            if i % BOX_SIZE == BOX_SIZE - 1 && i < SIZE - 1 {
                writeln!(f, "  ├───────┼───────┼───────┤")?;
            }
        }
        
        writeln!(f, "  └───────┴───────┴───────┘")
    }
} 