use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::render::TextureQuery;
use sdl2::video::Window;

use crate::model::game_model::Coord;
use crate::model::game_model::{Game, Card, GameState};


pub struct Renderer {
    pub screen_area: Rect,
    pub clear_color : Color,

}

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);


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

fn render_status_box(canvas : &mut Canvas<Window>, y : i32, x : i32, height : u32, width : u32, game : & Game, game_state : GameState) {

}



impl Renderer {
    fn render_card(&self, canvas : &mut Canvas<Window>, card : &Card, y : i32, x : i32, card_height : u32 , card_width : u32) {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let font_path = "./python/fonts/OpenSans-Bold.ttf";

        let texture_creator = canvas.texture_creator();
        // Load a font
        let mut font = ttf_context.load_font(font_path, 24).unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);
        //let txt = (row*8+col+1).to_string();
        let txt = card.title.as_str();
        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render(&txt)
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| e.to_string()).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string()).unwrap();

        // canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
        // canvas.clear();

        let TextureQuery { width, height, .. } = texture.query();

        // If the example text is too big for the screen, downscale it (and center irregardless)
        // let padding = 64;
        let mut target = get_centered_rect(
            width,
            height,
            card_width,
            card_height,
        );
        target.x += x;
        target.y += y;
        canvas.copy(&texture, None, Some(target)).unwrap();
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
                        let x: i32 = (padding+col*(card_width + padding)).try_into().unwrap();
                        let y: i32= (padding+row*(card_height+padding)).try_into().unwrap();
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
    }
}