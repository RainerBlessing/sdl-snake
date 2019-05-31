extern crate sdl2;

mod constants;
mod events;
mod snake;
mod canvas;

pub fn main() {
    let ttf_context = sdl2::ttf::init().unwrap();

    let mut sdl_canvas = canvas::SdlCanvas::new(&ttf_context);

    let mut snake = snake::Snake::new();

    sdl_canvas.start(&mut snake);
}

