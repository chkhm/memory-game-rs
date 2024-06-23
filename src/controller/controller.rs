
use crate::model::game_model::{Coord, Game};
use crate::view::board_view;

// use sdl2::libc::winsize;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect; //, sys::SDL_QuitEvent};
use sdl2::pixels::Color;
use sdl2::event::Event;
// use sdl2::video::WindowBuilder;


pub enum ControlState {
    StartGame,
    FirstCard,
    SecondCard,
    ViewResult,
    NextUser,
    GameOver,
}

pub struct Control {
    pub state : ControlState,
    pub game : Game,
}

impl Control {
    pub fn new(height : usize, width : usize) -> Control {
        Control {
            state : ControlState::StartGame,
            game : Game::new(height, width),
        }
    }

    pub fn reset(&mut self) {
        self.state = ControlState::StartGame;
        self.game.reset();
    }
    
    pub fn run(&mut self) {
        let screen_width : u32 = 600;
        let screen_height: u32 = 800;
    
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Rusty!", screen_width, screen_height)
            .build()
            .unwrap();
    
        let game_window_id = window.id();

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
                    Event::Quit { timestamp: _ } => { running = false; },
                    Event::MouseMotion { timestamp: _, window_id: _, which: _, mousestate: _, x, y, xrel, yrel } => {
                        println!("Mouse x: {}, y: {} \t x-rel: {}, y-rel: {}", x, y, xrel, yrel);
                    },
                    Event::MouseButtonDown { timestamp: _, window_id, which: _, mouse_btn, clicks: _, x, y } => {
                        if window_id == game_window_id && mouse_btn == MouseButton::Left {
                            let padding: i32  = 5;
                            let screen_height_i32: i32 = screen_height.try_into().unwrap();
                            let screen_width_i32: i32 = screen_width.try_into().unwrap();
                            let y_max : i32 = screen_height_i32 - padding;
                            let x_max : i32 = screen_width_i32 - padding;

                            if y > padding || y < y_max || x > padding || x < x_max {
                                //let t1 : i32 = screen_width_i32 / 8;
                                //let t2 : i32 = padding*8/6;
                                //let card_width: i32 = t1 - t2;
                                //let t1 : i32 = screen_height_i32 / 8;
                                //let t2 : i32 = padding*8/6;
                                //let card_height: i32= t1 - t2;
                                //let row : i32 = (y - padding) / card_height;
                                //let col : i32 = (x - padding) /card_width;

                                let card_width_plus_padding : u32 = screen_width / 8;
                                let card_height_plus_padding : u32 = screen_height / 8;
                                let x_minus_padding : u32 = (x - padding).try_into().unwrap();
                                let y_minus_padding : u32 = (y - padding).try_into().unwrap();
                                let col : u32 = (x_minus_padding) / card_width_plus_padding;
                                let row : u32 = (y_minus_padding) / card_height_plus_padding;
                                let coord = Coord(row.try_into().unwrap(), col.try_into().unwrap());
                                self.game.open_card(&coord);
                                println!("Card Opened at ({}, {})", coord.0, coord.1);
                            }
                        }
                    }
                    _ => {}
                }
            }
            // let mouse_state = event_queue.mouse_state();

            board_view.render(&mut canvas);
            canvas.present();
        }    
    }
}