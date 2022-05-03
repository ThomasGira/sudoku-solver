pub struct Sudoku{
    board:Vec<u32> ,
    size: u32,
    grid_size: u32
}

impl Sudoku {

    pub fn new(initial_board: Vec<u32>, game_size: u32) -> Sudoku {
        Sudoku {
            board: initial_board,
            size: game_size,
            grid_size: f64::sqrt(game_size as f64) as u32
        }
    }

    pub fn print_board(&self){
        println!("");
        for i in 0..self.size {
            for j in 0..self.size {
                let index: u32 = i* self.size + j;
                print!("{}",self.board[index as usize]);
                if j % self.grid_size == 1 && j != self.size -1{
                    print!("|");
                }
            }
            if i % self.grid_size == 1 && i != self.size -1{
                println!("");
                for _ in 0..self.size+ self.grid_size -1{
                    print!("-");
                }
            }
            println!("");
        }
        println!("");
    }

    pub fn add_number(&mut self, y: u32, x: u32, number: u32) -> bool{
        let index: usize = (y*self.size + x) as usize;
        if number < 1 || number > self.size{
            //println!("Error: Invalid Number");
            return false;
        }
        if self.board[index] != 0 {
            // println!("Error: Index {} already filled",index);
            if self.board[index] == number{
                // println!("ASDSD");
                return true;
            }
            return false;
        }
        let mut temp_board: Vec<u32> = Vec::new();
        for i in 0..self.size*self.size{
            temp_board.push(self.board[i as usize]);
        }
        temp_board[index] = number;
        if self.check_valid_placement(temp_board.to_vec()){
            self.board = temp_board.to_vec();
            return true;
        }
        return false;
    }

    fn check_valid_placement(&self,board: Vec<u32>) -> bool{
        if ! self.check_valid_box(board.to_vec()){
            return false;
        }
        if ! self.check_valid_row(board.to_vec()){
            return false;
        }
        if ! self.check_valid_col(board.to_vec()){
            return false;
        }
        return true;
    }

    fn check_valid_row(&self,board: Vec<u32>) -> bool{
        let mut valid_numbers: Vec<u32> = Vec::new();
        let mut index: u32;
        let mut value: u32;
        for i in 0..self.size{
            for k in 0..self.size {
                valid_numbers.push(k+1 as u32);
            }
            for j in 0.. self.size{
                index = i*self.size + j;
                value = board[index as usize];
                if value > 0 {
                    if valid_numbers[(value -1) as usize] == 0{
                        //println!("Error: Invalid Row");
                        return false;
                    } else {
                        valid_numbers[(value -1) as usize] = 0;
                    }
                }
            }
            for _ in 0..self.size {
                valid_numbers.pop();
            }
        }
        return true;
    }

    fn check_valid_col(&self,board: Vec<u32>) -> bool{
        let mut valid_numbers: Vec<u32> = Vec::new();
        let mut index: u32;
        let mut value: u32;
        for i in 0..self.size{
            for k in 0..self.size {
                valid_numbers.push(k+1 as u32);
            }
            for j in 0.. self.size{
                index = j*self.size + i;
                value = board[index as usize];
                if value > 0 {
                    if valid_numbers[(value -1) as usize] == 0{
                        //println!("Error: Invalid Column");
                        return false;
                    } else {
                        valid_numbers[(value -1) as usize] = 0;
                    }
                }
            }
            for _ in 0..self.size {
                valid_numbers.pop();
            }
        }
        return true;
    }

    fn check_valid_box(&self,board: Vec<u32>) -> bool{
        let mut valid_numbers: Vec<u32> = Vec::new();
        let mut index: u32;
        let mut value: u32;
        for i in 0..self.size{
            for k in 0..self.size {
                valid_numbers.push(k+1 as u32);
            }
            for j in 0.. self.size{
                index = i*self.size + j;
                value = board[index as usize];
                if value > 0 {
                    if valid_numbers[(value-1) as usize] == 0{
                        //println!("Error: Invalid Box");
                        return false;
                    } else {
                        valid_numbers[(value-1) as usize] = 0;
                    }
                }
            }
            for _ in 0..self.size {
                valid_numbers.pop();
            }
        }
        return true;
    }

    fn check_zeros(&self, board: Vec<u32>) -> bool{
        for i in 0..self.size*self.size{
            if board[i as usize] == 0{
                return false;
            }
        }
        return true;
    }

    pub fn is_solved(&self) -> bool{
        let mut board: Vec<u32> = Vec::new();
        for i in 0..self.size*self.size{
            board.push(self.board[i as usize]);
        }
        if ! self.check_valid_box(board.to_vec()){
            return false;
        }
        if ! self.check_valid_row(board.to_vec()){
            return false;
        }
        if ! self.check_valid_col(board.to_vec()){
            return false;
        }
        if ! self.check_zeros(board.to_vec()){
            return false;
        }
        return true;
        
    }
}