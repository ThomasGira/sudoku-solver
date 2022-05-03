mod game;
mod prediction;
mod reduction;
use std::time::Instant;

fn main()
{
    let list1 = vec![0, 1, 0, 0, 
                    3, 0, 0, 1, 
                    4, 0, 0, 2, 
                    0, 0, 4, 0];
    let _list2 = vec![5, 3, 0, 0, 7, 0, 0, 0, 0, 
                    6, 0, 0, 1, 9, 5, 0, 0, 0,
                    0, 9, 8, 0, 0, 0, 0, 6, 0,
                    8, 0, 0, 0, 6, 0, 0, 0, 3,
                    4, 0, 0, 8, 0, 3, 0, 0, 1,
                    7, 0, 0, 0, 2, 0, 0, 0, 6,
                    0, 6, 0, 0, 0, 0, 2, 8, 0,
                    0, 0, 0, 4, 1, 9, 0, 0, 5,
                    0, 0, 0, 0, 8, 0, 0, 7, 9,];
    
    
    let now = Instant::now();
    brutus(&list1,4);
    let elapsed_time = now.elapsed();
    println!("Running brutus() took {:.6} seconds.", elapsed_time.as_micros() as f64/1000000.0);
    
    let now = Instant::now();
    big_brother(&list1,4);
    let elapsed_time = now.elapsed();
    println!("Running big_brother() took {:.6} seconds.", elapsed_time.as_micros() as f64/1000000.0);
    
    let now = Instant::now();
    doku(&list1,4);
    let elapsed_time = now.elapsed();
    println!("Running doku() took {:.6} seconds.", elapsed_time.as_micros() as f64/1000000.0);
    
    let now = Instant::now();
    doku(&_list2,9);
    let elapsed_time = now.elapsed();
    println!("Running doku() took {:.6} seconds.", elapsed_time.as_micros() as f64/1000000.0);
} 

fn brutus(list: &Vec<u32>, game_size: u128){
    println!("");
    println!("");
    println!("Running Brutus on a {}x{} Grid", game_size, game_size);
    println!("");
    println!("");
    let mut i: u128 = 0;
    let num_to_solve = list.to_vec().iter().filter(|&n| *n == 0).count() as u128;
    println!("num_to_solve: {}", num_to_solve);
    let max_val:u128 = game_size.pow((num_to_solve) as u32) -1;
    println!("max_value: {}", max_val);

    let mut last_percent: f64 = 0.0;
    let mut curr_percent: f64;

    while i < max_val {
        curr_percent = (i as f64)/(max_val as f64)*100.0;
        if (curr_percent/5.0).floor() > last_percent{
            println!("{:.0}% Complete",curr_percent);
            last_percent = (curr_percent/5.0).floor();
        }
        let mut p = game::Sudoku::new(list.to_vec(),game_size as u32);
        let combinations = deref(i as u128, game_size as u128,1);
        //println!("Combinations: {:?}", combinations);
        let placements = merge(list.to_vec(),combinations);
        // println!("Placements: {:?}", placements);
        i = i+1;
        for k in 0..game_size.pow(2){
            let position = deref(k as u128,4,0);
            
            if !p.add_number(position[1] as u32,position[0] as u32,placements[k as usize]){
                break;
            }
        }
        if p.is_solved(){
            p.print_board();
            return;
        }
    }
}

fn big_brother(list: &Vec<u32>, game_size: u32){
    println!("");
    println!("");
    println!("Running Big Brother on a {}x{} Grid", game_size, game_size);
    println!("");
    let mut given: Vec<(u32,u32)> = vec![];
    let num_to_solve = list.to_vec().iter().filter(|&n| *n == 0).count() as u32;
    let max_val:f64 = (game_size as f64).powf((num_to_solve) as f64) -1.0;
    println!("num_to_solve: {}", num_to_solve);
    println!("max_value: {}", max_val);
    for i in 0..game_size*game_size{
        if list[i as usize] != 0{
            given.push((i,list[i as usize]));
        }
    }
    let mut q = prediction::Prediction::new(given, game_size);
    q.initialize();
    let mut last_percent: f64 = 0.0;
    let mut i: f64 = 0.0;
    let mut curr_percent: f64;
    loop {
        i = i+1.0;
        curr_percent = (i as f64)/(max_val as f64)*100.0;
        if (curr_percent/5.0).floor() > last_percent{
            println!("{:.0}% Complete Index: {}",curr_percent,i);
            last_percent = (curr_percent/5.0).floor();
        }
        let mut p = game::Sudoku::new(list.to_vec(), game_size);
        if ! q.iter_prediction() {
            println!("Reached maximum iterations without as solution");
            return;
        }
        let prediction = q.get_prediction().to_vec();
        for k in 0..game_size.pow(2){
            let position = deref(k as u128,4,0);
            
            if !p.add_number(position[1] as u32, position[0] as u32, prediction[k as usize]){
                break;
            }
        }
        if p.is_solved(){
            p.print_board();
            return;
        }
    }
}

fn doku(list: &Vec<u32>, game_size: u32){
    println!("");
    println!("");
    println!("Running Doku on a {}x{} Grid", game_size, game_size);
    println!("");
    println!("");
    let mut given: Vec<(u32,u32)> = vec![];
    for i in 0..game_size*game_size{
        if list[i as usize] != 0{
            given.push((i,list[i as usize]));
        }
    }
   let mut q = reduction::Reduction::new(given,list.to_vec(), game_size as u32);
   q.initialize();
   let num_elements = q.get_num_possibilities();
   println!("Number of Possibilites: {}", num_elements);

   for i in 0..num_elements{
       q.iter_prediction();
       let prediction = q.get_prediction();
       let mut p = game::Sudoku::new(list.to_vec(), game_size);
        for k in 0..game_size.pow(2){
            let position = deref(k as u128,4,0);
            
            if !p.add_number(position[1] as u32, position[0] as u32, prediction[k as usize]){
                break;
            }
        }
        if p.is_solved(){
            p.print_board();
            return;
        }
    }
}

fn merge(mut base: Vec<u32>, mut supp: Vec<u32>) -> Vec<u32>{
    let length: u32 = base.len() as u32;
    supp.reverse();
    for i in 0..length{
        if base[i as usize] == 0{
            base[i as usize] = match supp.pop() {
                Some(top) => top,
                None => 0
            };
        }
    }
    return base
}

fn deref(reference: u128, base:u128, offset:u128) -> Vec<u32> {
    let mut indices: Vec<u32> = Vec::new();
    let mut temp: u128 = reference;
    for i in 1..base*base+1 {
        let div: u128 = base.pow((base*base-i)as u32);
        let sub: u128 = temp/div;
        indices.insert(0,(sub + offset) as u32);
        temp = temp - sub*div;
    }
    return indices;
}