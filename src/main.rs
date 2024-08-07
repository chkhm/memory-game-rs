
use std::io;

mod model;
use model::game_model::{Game, Coord};

mod controller;
use controller::controller::Control;

mod view;
// use view::board_view;

fn parse_and_check_bounds(s : &str, lower : usize, upper : usize) -> (bool, usize) {
    // let mut rslt: (bool, usize) = (false, 0);
    let parse_ok : bool ;
    let val :usize = match s.trim().parse() {
        Ok(y) => { parse_ok = true; y },
        Err(_) => { parse_ok = false; 0 },
    };
    if parse_ok && val >= lower && val <= upper {
        let rslt = (true, val);
        return rslt
    } else {
        let rslt = (false, val);
        return rslt
  
    }
}

fn query_one_coord(coord_counter : usize, upper : usize) -> Coord {
    let mut the_coord = Coord(0,0);
    let mut input_ok = false;
    while ! input_ok {
        let mut coord_str = String::new();
        println!("\nMake a guess, for coord {} like so \"3, 5\":", coord_counter);
        io::stdin()
            .read_line(&mut coord_str)
            .expect("Failed to read");
        let splitted: Vec<&str>= coord_str.split(",").collect();
        if splitted.len() != 2 {
            println!("Input not acceptable. try again.");
        } else {
            let mut rslt: [(bool, usize); 2] = [(false, 0), (false, 0)];
            for j in 0..2 {
                rslt[j] = parse_and_check_bounds(splitted[j], 0, upper);
            }
            if rslt[0].0 && rslt[1].0 {
                the_coord = Coord(rslt[0].1, rslt[1].1);
                input_ok = true;
            } else {
                println!("Input coordinates must be integer number between 0 and 7 (inclusive). Try again.");
            }
        }
    }
    the_coord
}

fn main() -> Result<(), String> {
    let height = 8;
    let width = 8;
    let mut control = Control::new(height, width);
    control.reset();
    let player_hugo = "Hugo".to_string();
    control.game.add_player(player_hugo);
    control.run();
    
    Ok(())
}
