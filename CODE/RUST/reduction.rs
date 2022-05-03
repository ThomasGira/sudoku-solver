use std::convert::TryInto;
pub struct Reduction{
    given:Vec<(u32,u32)>,
    size: u32,
    grid_size: u32,
    prediction: Vec<u32>,
    valid_numbers: Vec<Vec<u32>>,
    indices: Vec<u32>  
}

impl Reduction {
    pub fn new(given: Vec<(u32,u32)>,starting_game: Vec<u32>,game_size: u32) -> Reduction {
        Reduction {
            given: given,
            size: game_size,
            grid_size: f64::sqrt(game_size as f64) as u32,
            prediction: starting_game,
            valid_numbers: vec![],
            indices: vec![0;(game_size*game_size).try_into().unwrap()],
        }
    }

    pub fn initialize(&mut self){
        for i in 0..self.given.len() {
            let (index, num) = self.given[i as usize];
            self.prediction[index as usize] = num;
        }
        for i in 0..self.size*self.size{
        // for i in 0..3{
            self.valid_numbers.push(vec![]);
            if self.prediction[i as usize] == 0 {
                let row: u32 = self.get_row(i);
                let col: u32 = self.get_col(i);
                let block: u32 = self.get_block(i);
                for j in 1..self.size+1{
                    let number: u32 = j as u32;
                    if ! self.valid_row(row,number){
                        continue;
                    }
                    if ! self.valid_col(col,number){
                        continue;
                    }
                    if ! self.valid_block(block,number){
                        continue;
                    }
                    self.valid_numbers[i as usize].push(j);
                }    
                if self.valid_numbers[i as usize].len() == 1{
                    self.prediction[i as usize] = self.valid_numbers[i as usize][0];
                }            
                // println!("Index: {}, Row: {}, Col: {}, Block: {}", i,row,col,block);
            } else {
                self.valid_numbers[i as usize].push(self.prediction[i as usize]);
            }
            // println!("{:?}", self.prediction);
            // println!("{:?}", self.valid_numbers);
        }
    }

    fn get_row(&self, index: u32) -> u32{
        return index / (self.size);
    }

    fn get_col(&self, index: u32) -> u32{
        return index % (self.size);
    }

    fn get_block(&self, index: u32) -> u32{
        let row: u32 = self.get_row(index);
        let col: u32 = self.get_col(index);
        
        return (row/self.grid_size)*self.grid_size + col/self.grid_size;
    }

    fn valid_row(&self, row: u32, number: u32) -> bool{
        let index: usize = (row*self.size) as usize;
        for i in 0..self.size{
            // println!("(ROW)Index: {}, Element: {}, Number: {}",i,self.prediction[index + i as usize], number);
            if self.prediction[index + i as usize] == number{
                return false;
            }
        }
        return true;
    }

    fn valid_col(&self, col: u32, number: u32) -> bool{
        for i in 0..self.size{
            // println!("I(COL)ndex: {}, Element: {}, Number: {}",i,self.prediction[(i * self.size + col) as usize], number);
            if self.prediction[(i * self.size + col) as usize] == number{
                return false;
            }
        }
        return true;
        
    }

    fn valid_block(&self, block: u32, number: u32) -> bool{
        let start_row: u32 = (block / (self.grid_size))*self.grid_size;
        let start_col: u32 = (block % (self.grid_size))*self.grid_size;

        for i in 0..self.grid_size{
            for j in 0..self.grid_size{
                let index = ((start_row+i)*self.size + (start_col+j)) as usize;
                // println!("(BLOCK)Index: {}, Element: {}, Number: {}",index,self.prediction[index], number);
                if self.prediction[index] == number{
                    return false;
                }
            }
        }
        return true;        
    }

    pub fn get_num_possibilities(&self) -> u32{
        let mut sum: f64 = 1.0;
        for i in 0..self.size*self.size{
            let cell = &self.valid_numbers[i as usize];
            if !cell.is_empty(){
                sum = sum * (cell.len() as f64);
            }
        }
        return sum as u32;
    }

    pub fn iter_prediction(&mut self){
        for i in 0..self.indices.len(){
            let index = self.indices[i as usize] as usize;
            if index as u32 == self.valid_numbers[i as usize].len() as u32 -1{
                self.indices[i as usize] = 0;
            } else {
                self.indices[i as usize] = self.indices[i as usize] + 1;
                break;
            }
        }
    }

    pub fn get_prediction(&self) -> Vec<u32> {
        let mut prediction: Vec<u32> = vec![];
        for i in 0..self.valid_numbers.len(){
            let index = i as usize;
            let num_index = self.indices[index] as usize;
            prediction.push(self.valid_numbers[index][num_index]);
        }
        return prediction;
    }
}