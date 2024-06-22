
use std::io;

use sdl2::{rect::Rect, sys::SDL_QuitEvent};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::video::WindowBuilder;

use memory_game::{Game, Coord};

mod view;
use view::board_view;

fn parse_and_check_bounds(s : &str, lower : usize, upper : usize) -> (bool, usize) {
    let mut rslt: (bool, usize) = (false, 0);
    let mut parse_ok = false;
    let val :usize = match s.trim().parse() {
        Ok(y) => { parse_ok = true; y },
        Err(_) => { parse_ok = false; 0 },
    };
    if parse_ok && val >= lower && val <= upper {
        rslt = (true, val);
    } else {
        rslt = (false, val);
    }
    rslt
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

    let screen_width = 600;
    let screen_height = 800;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Rusty!", screen_width, screen_height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    let board_view = board_view::Renderer {
        screen_area : Rect::new(0, 0, screen_width, screen_height),
        clear_color : Color::RGB(64, 192, 255),
    };

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { timestamp } => { running = false; },
                Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => {
                    println!("Mouse x: {}, y: {} \t x-rel: {}, y-rel: {}", x, y, xrel, yrel);
                },
                _ => {}
            }
        }
        board_view.render(&mut canvas);
        canvas.present();
    }

    let height = 8;
    let width = 8;
    let mut game = Game::new(height, width);
    println!("Welcome to the good olde fashioned memory game!\n");
    println!("Game width {} and height {}", game.field.width, game.field.height);
    let player_hugo = "Hugo".to_string();
    game.add_player(player_hugo);
    println!("Added Player: {}", game.players[0].name);
    println!("Deck has {} cards.", game.deck.len());
    println!("Setting cards...");
    game.reset();
    while ! game.check_game_over() && false {
        game.print_field();
        let mut coord_pair = [Coord(0,0), Coord(0,0)];
        for i in 0..2 {
            coord_pair[i] = query_one_coord(i, 7);
        }
        println!("\nPlayer 0 trying his luck at {} and {}", coord_pair[0], coord_pair[1]);
        let player_id = game.current_player_id;
        let found_pair = game.check_guess(player_id, &coord_pair[0], &coord_pair[1]);
        if found_pair {
            println!("You are in luck!");
            game.print_cards_of_player(player_id);
        } else {
            println!("Sorry, that did not work out");
            for coord in coord_pair {        
                println!("\nlocation {}: ", coord);
                game.print_card_at(&coord);
            }    
        }
        game.next_player();
    }
    
    Ok(())
}
