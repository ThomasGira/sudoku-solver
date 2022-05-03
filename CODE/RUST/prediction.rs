use std::convert::TryInto;

pub struct Prediction{
    given:Vec<(u32,u32)>,
    size: u32,
    num_elements: u32,
    grid_size: u32,
    prediction: Vec<u32>,
    index: u32,
    indices: Vec<u32>,
    max_index: u32
}

impl Prediction{
    pub fn new(given: Vec<(u32,u32)>,game_size: u32) -> Prediction {
        Prediction {
            given: given,
            size: game_size,
            num_elements: game_size*game_size,
            grid_size: f64::sqrt(game_size as f64) as u32,
            prediction: vec![0;(game_size*game_size).try_into().unwrap()],
            index: 1,
            indices: vec![],
            max_index: 0
        }
    }

    pub fn initialize(&mut self){
        for i in 0..self.given.len() {
            let (index, num) = self.given[i as usize];
            self.prediction[index as usize] = num;
        }
        for i in 0..self.num_elements{
            if self.prediction[i as usize] == 0 {
                self.indices.insert(0,i);
                self.prediction[i as usize] = 1;
            }
        }
        self.max_index = self.indices.len() as u32;
    }

    pub fn get_prediction(&self) -> &Vec<u32> {
        return &self.prediction;
    }

    pub fn iter_prediction(&mut self) -> bool{
        for i in 0..self.max_index{
            let index = self.indices[i as usize] as usize;
            // println!("Current Index: {}", index);
            if self.prediction[index] == self.size{
                self.prediction[index] = 1;
                if i == self.index -1 {
                    self.index = self.index + 1;
                }
            } else {
                self.prediction[index] = self.prediction[index] + 1;
                break;
            }
        }
        if self.index == self.max_index + 1{
            println!("Max Iterations Reached");
            return false;
        }
        return true;
    }

    fn iter_element(&mut self, index:u32) -> bool {
        if self.prediction[index as usize] == self.size +1{
            self.prediction[index as usize] = 0;
            return false;
        } else {
            self.prediction[index as usize] = self.prediction[index as usize] + 1;
            return true;
        }
    }
}