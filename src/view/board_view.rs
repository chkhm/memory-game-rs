use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::render::TextureQuery;
use sdl2::render::Texture;
use sdl2::surface::Surface;
use sdl2::video::Window;

use crate::model::game_model::Coord;
use crate::model::game_model::{Game, Card, GameState};

// -----------------------------------------------------------------------------------------------
/// 
/// The Renderer class. Holds all data and functionality to render the view on the SDL2 Canvas
/// 
// -----------------------------------------------------------------------------------------------
pub struct Renderer {
    pub window_height : u32,
    pub window_width : u32,
    pub statusbar_area : Rect,
    pub screen_area : Rect,
    pub clear_color : Color,
}

/// handle the annoying Rect i32 casting need
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

/// This function creates a rectangle of given dimensions as a SDL2::Rect that is centered within 
/// a constraining rectangle.
/// 
/// Returns a Rect with dimension: 
///  - (rect_width, rect_height) 
/// that is centered inside 
/// - (cons_width, cons_height)
/// If the rectangle does not fit inside the constraining rectangle it is scaled down.
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let t1 : i32 = (cons_width / 2).try_into().unwrap();
    let t2 = w / 2;
    let cx :i32 =  t1 - t2;
    let t1 : i32 = (cons_height / 2).try_into().unwrap();
    let t2 = h / 2;
    let cy = t1 - t2;
    rect!(cx, cy, w, h)
}

struct TextRenderData<'a> {
    text : &'a str,
    font_path : &'a str,
    font_style : sdl2::ttf::FontStyle,
}

// -----------------------------------------------------------------------------------------------
/// Implementation of the Renderer.
/// The main public API function is render(). It renders the view on a SDL2 Canvas based on the 
/// state of the game.
// -----------------------------------------------------------------------------------------------
impl Renderer {
    /// Creates a SDL2 Surface from a given text which can be used to create a Texture.
    fn surface_from_text(&self, text_render_data : &TextRenderData) -> Surface {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

        // Load a font
        let mut font = ttf_context.load_font(text_render_data.font_path, 24).unwrap();
        font.set_style(text_render_data.font_style);
        //let txt = (row*8+col+1).to_string();
        let txt = text_render_data.text;
        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render(&txt)
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| e.to_string()).unwrap();
        surface
    }

    /// Return a string representation of the given GameState
    fn format_status(& self, state : & GameState) -> &str {
        match state {
            GameState::GameSetup => { "Enter player name" },
            GameState::StartGame => { "select first card" },
            GameState::StartSelectCards => { "select first card" },
            GameState::FirstCard => { "select second card" },
            GameState::SecondCard => { "SecondCard" },
            // GameState::ViewResult=> { "ViewResult" },
            GameState::NextUser => { "NextUser" },
            GameState::GameOver => { "Game Over!" },
        }
    }

    fn render_text(& self, canvas : &mut Canvas<Window>, rect : &Rect, text_render_data : &TextRenderData, clear_box : bool) {
        if clear_box {
            canvas.set_draw_color(self.clear_color);
            canvas.fill_rect(*rect).ok().unwrap_or_default();
        }

        let surface = self.surface_from_text(text_render_data);

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string()).unwrap();

        let TextureQuery { width, height, .. } = texture.query();

        let mut dst = get_centered_rect(
            width,
            height,
            rect.width(),
            rect.height(),
        );
        dst.x += rect.left();
        dst.y += rect.top();
        canvas.copy(&texture, None, dst).unwrap();
    }

    /// Function renders an interactive box on the screen where a user can enter the name
    fn render_signup_box(& self, game : & Game) {
        
    }

    /// Function renders the status bar. The status bar shows the points of the top five players. It also shows the current player,
    /// and the round number.
    /// 
    fn render_status_box(&self, canvas : &mut Canvas<Window>, game : &Game) {
        let text = format!("Round: {} - Current Player: {} has {} cards - {}", 
            game.round(), game.current_player().name, game.current_player().collected_cards.len(), 
            self.format_status(&game.game_state()));
        let font_path = "./python/fonts/OpenSans-Bold.ttf";
        let font_style = sdl2::ttf::FontStyle::BOLD;

        let text_render_data = TextRenderData {
            text : text.as_str(),
            font_path,
            font_style,
        };

        let rect = rect!(
            self.statusbar_area.left(), 
            self.statusbar_area.top(), 
            self.statusbar_area.width(), 
            self.statusbar_area.height());
        self.render_text(canvas, &rect, &text_render_data, true);
    }
    
    /// Function renders a single card at given position and dimension on the Canvas
    fn render_card(&self, canvas : &mut Canvas<Window>, card : &Card, y : i32, x : i32, card_height : u32 , card_width : u32) {
        let text = card.title.as_str();
        let font_path = "./python/fonts/OpenSans-Bold.ttf";
        let font_style = sdl2::ttf::FontStyle::BOLD;

        let text_render_data = TextRenderData {
            text,
            font_path,
            font_style,
        };

        let rect = rect!(x, y, card_width, card_height);
        self.render_text(canvas, &rect, &text_render_data, false);
    }


    /// Function renders a text on the canvas right across the cards. The text is a
    /// success message if the player opened to matching cards or otherwise a fail 
    /// message.
    fn render_check_result_box(&self, canvas : &mut Canvas<Window>, game : &Game) {
        let mut text = "Not a pair, bad luck.";

        if game.last_guess_success() {
            text = "You found a pair!";
        }

        let font_path = "./python/fonts/OpenSans-Bold.ttf";
        let font_style = sdl2::ttf::FontStyle::BOLD;

        let text_render_data = TextRenderData {
            text,
            font_path,
            font_style,
        };

        let rect = rect!(0, 400, (self.statusbar_area.width() * 3) / 5 , self.statusbar_area.height()-10);        
        self.render_text(canvas, &rect, &text_render_data, false);
    }


    /// renders the cardboard to the screen.
    /// it iterates over each row and col.
    /// If a coordinate is empty (card already taken) it shows an empty area (no rectangle drawn).
    /// If a coordinate is not empty and not opened it shows a rectangle
    /// If a coordinate is not empty and opened it shows the rectangle and the title of the card.
    pub fn render(&self, canvas : &mut Canvas<Window>, game : &Game) {
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.screen_area).ok().unwrap_or_default();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let padding: u32  = 5;
        let card_width: u32 = (((self.screen_area.width()) / 8) - (padding*8/6)).try_into().unwrap();
        let card_height: u32= (((self.screen_area.height()) / 8) - (padding*8/6)).try_into().unwrap();
        //let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        //let font_path = "./python/fonts/OpenSans-Bold.ttf";

        for row in 0..8 {
            for col in 0..8 {

                let card = game.card_at(row.try_into().unwrap(), col.try_into().unwrap());
                match card {
                    Some(c) => {
                        let x_offset:u32 = self.screen_area.left().try_into().unwrap();
                        let y_offset:u32 = self.screen_area.top().try_into().unwrap();
                        let x: i32 = (x_offset+padding+col*(card_width + padding)).try_into().unwrap();
                        let y: i32= (y_offset+padding+row*(card_height+padding)).try_into().unwrap();
                        let r = Rect::new(x, y, card_width, card_height);
                        canvas.draw_rect(r).expect("Error on Drawing Rectangle on canvas");
                        let coord = Coord(row.try_into().unwrap(), col.try_into().unwrap());
                        if game.is_clicked(&coord) {
                            self.render_card(canvas, c, y, x, card_height, card_width);
                        }
                    },
                    None => {},
                } // match
            } // for col
        } // for row
        self.render_status_box(canvas, game);
        if game.game_state() == GameState::NextUser {
            self.render_check_result_box(canvas, game);
        }
    }
}