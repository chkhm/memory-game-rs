
use crate::model::game_model::{Coord, Game, GameState};
use crate::view::board_view;

// use sdl2::libc::winsize;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect; //, sys::SDL_QuitEvent};
use sdl2::pixels::Color;
use sdl2::event::Event;
// use sdl2::video::WindowBuilder;

pub struct Control {
    pub game : Game,
}

fn calculate_card_coord_from_mouse_click(y : i32, x : i32, screen_left : i32, screen_top : i32, screen_height : u32, screen_width : u32) -> Option<Coord> {
    let padding_left: i32  = 5 + screen_left;
    let padding_top : i32 = 5 + screen_top;
    let screen_height_i32: i32 = screen_height.try_into().unwrap();
    let screen_width_i32: i32 = screen_width.try_into().unwrap();
    let y_max : i32 = screen_height_i32 - 5 + screen_top;
    let x_max : i32 = screen_width_i32 - 5 + screen_left;

    if y > padding_top || y < y_max || x > padding_left || x < x_max {
        let card_width_plus_padding : u32 = screen_width / 8;
        let card_height_plus_padding : u32 = screen_height / 8;
        let x_minus_padding : u32 = (x - padding_left).try_into().unwrap();
        let y_minus_padding : u32 = (y - padding_top).try_into().unwrap();
        let col : u32 = (x_minus_padding) / card_width_plus_padding;
        let row : u32 = (y_minus_padding) / card_height_plus_padding;
        let coord = Coord(row.try_into().unwrap(), col.try_into().unwrap());
        return Some(coord);
    }
    None
}



impl Control {
    pub fn new(height : usize, width : usize) -> Control {
        Control {
            game : Game::new(height, width),
        }
    }

    pub fn reset(&mut self) {
        self.game.reset();
    }
    
    fn handle_mouse_click(&mut self, y : i32, x : i32, screen_height : u32, screen_width : u32) {
        let state = self.game.game_state();

        if state == GameState::GameOver {
            print!("Game over. Resetting ... ");
            self.game.reset(); // state == StartSelectCards
            println!("done.");
            let p = self.game.current_player();
            println!("Player {}, select your first card", p.name);
            return;
        }

        if state == GameState::StartSelectCards || state == GameState::StartGame {
            let p = self.game.current_player();
            let c = calculate_card_coord_from_mouse_click(y, x, screen_height, screen_width);
            if c.is_none() {
                println!("Player {}, select your first card", p.name);
                return;
            }
            let c = c.unwrap();
            let card_opened = self.game.open_card(&c); // state is FirstCard if success
            if card_opened {
                println!("Card Opened at ({}, {})", c.0, c.1);
                println!("Player {}, select your second card", p.name);
                return;
            }
            println!("No card opened.");
            println!("Player {}, select your first card", p.name);
            return ;
        }

        if state == GameState::FirstCard {
            let p = self.game.current_player();
            let c = calculate_card_coord_from_mouse_click(y, x, screen_height, screen_width);
            if c.is_none() {
                println!("Player {}, select your second card", p.name);
                return;
            }
            let c = c.unwrap();
            let card_opened = self.game.open_card(&c); // state is SecondCard if success
            if card_opened {
                println!("Card Opened at ({}, {})", c.0, c.1);
                println!("Player {}, check the result", p.name);
                return;
            }
            println!("No card opened.");
            println!("Player {}, select your second card", p.name);
            return ;
        }

        if state == GameState::SecondCard {
            let p = self.game.current_player();
            let found_pair = self.game.check_guess_current_player(); // state is now ViewResult
            if found_pair {
                println!("Player {}, you found a pair, you now have cards", p.name);
                self.game.print_cards_of_current_player();
            } else {
                println!("Player {}, bad luck, no pair found", p.name);
            }
            println!("Player {}, click to pass on to next player.", p.name);
            return;
        }

        if state == GameState::ViewResult {
            let p = self.game.current_player();
            let game_over = self.game.check_game_over(); // result is now either GameOver or NextUser
            if game_over {
                println!("Player {}, Game is over. Press any key to start new game.", p.name);
            } else {
                self.game.close_selected_cards();
                self.game.next_player();
                let p = self.game.current_player();
                println!("Player {}, your turn!", p.name);
            }
            return ;
        }

        if state == GameState::NextUser {
            println!("Warning! We should never reach this state!");
            self.game.next_player();
            let p = self.game.current_player();
            println!("Player {}, click to start your selection", p.name);
            return ;
        }
    }

    pub fn run(&mut self) {
        let window_height : u32 = 1000;
        let window_width : u32 = 1600;
        let screen_height: u32 = 800;
        let screen_width : u32 = 600;
        let status_bar = Rect::new(0, (window_height-40).try_into().unwrap(), window_width, 40);
    
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Play Memory!", window_width, window_height)
            .build()
            .unwrap();
    
        let game_window_id = window.id();

        let mut canvas = window.into_canvas()
            .build()
            .unwrap();
    
        let board_view = board_view::Renderer {
            window_height,
            window_width,
            statusbar_area : status_bar,
            screen_area : Rect::new(200, 100, screen_width, screen_height),
            clear_color : Color::RGB(64, 192, 255),
        };
    
        let mut running = true;
        let mut event_queue = sdl_context.event_pump().unwrap();
    
        while running {
            for event in event_queue.poll_iter() {
                match event {
                    Event::Quit { timestamp: _ } => { running = false; },
                    // Event::MouseMotion { timestamp: _, window_id: _, which: _, mousestate: _, x, y, xrel, yrel } => {
                    //     println!("Mouse x: {}, y: {} \t x-rel: {}, y-rel: {}", x, y, xrel, yrel);
                    // },
                    Event::MouseButtonDown { timestamp: _, window_id, which: _, mouse_btn, clicks: _, x, y } => {
                        if window_id == game_window_id && mouse_btn == MouseButton::Left {
                            self.handle_mouse_click(y, x, screen_height, screen_width);
                        }
                    }
                    _ => {}
                }
            }
            board_view.render(&mut canvas, &self.game);
            canvas.present();
        }    
    }
}