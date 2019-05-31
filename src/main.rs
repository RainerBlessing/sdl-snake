extern crate sdl2;

mod constants;
mod events;
mod snake;
mod canvas;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::constants::GameState;
use crate::events::SnakeEvent;

use crate::constants::Keyboard;

fn parse_event(snake: &mut snake::Snake, snake_event: SnakeEvent) -> () {
    let key = snake_event.get_key();
    match key {
        Keyboard::Up => snake.move_up(),
        Keyboard::Down => snake.move_down(),
        Keyboard::Left => snake.move_left(),
        Keyboard::Right => snake.move_right(),
        _ => {}
    }
}

pub fn main() {

    let sdl_context = sdl2::init().unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();

    let mut sdl_canvas = canvas::SdlCanvas::new(&ttf_context, &sdl_context);


    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game_state = GameState::Play;

    let mut snake = snake::Snake::new();


    snake.setup_board();

    'running: loop {
        sdl_canvas.start_loop();

        sdl_canvas.clear();

        let grid = snake.draw_elements();

        sdl_canvas.draw(grid);

        for sdl_event in event_pump.poll_iter() {
            match game_state {
                GameState::Play => {
                    match sdl_event {
                        Event::Quit { .. } |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'running;
                        }

                        _ => {
                            parse_event(&mut snake,SnakeEvent::new(sdl_event));
                        }
                    }
                }
                GameState::GameOver => {
                    match sdl_event {
                        Event::Quit { .. } |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'running;
                        }
                        Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                            snake.setup_board();
                            game_state = GameState::Play;
                        }
                        _ => {}
                    }
                }
            }
        }

        match game_state {
            GameState::Play => {
                game_state = snake.play_state();
            }
            _ => {}
        }

        sdl_canvas.create_score_text(snake.score.to_string().as_str());


        sdl_canvas.present();

        sdl_canvas.loop_delay()
    }
}

