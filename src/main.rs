extern crate sdl2;
mod constants;
mod snake;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::TimerSubsystem;

use std::time::Duration;
use sdl2::rect::Rect;

use std::path::Path;
use sdl2::render::TextureQuery;

use crate::constants::PlayField;
use crate::constants::Type;
use crate::constants::GameState;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const SNAKE_WIDTH: u32 = WIDTH / 40;


// handle the annoying Rect i32
macro_rules! rect (
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
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

    let cx = (WIDTH as i32 - w) / 2;
    let cy = (HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

pub fn main() {
    let font_path: &Path = Path::new("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();
    ;

    let mut font = ttf_context.load_font(font_path, 128).unwrap();
    ;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let window = video_subsystem.window("SDL2 Snake", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();


    let mut event_pump = sdl_context.event_pump().unwrap();

    let frame_delay = 1000 / 60;
    let mut timer = sdl_context.timer().unwrap();


    const M: usize = 40;
    const N: usize = 40;

    let mut game_state = GameState::Play;


    let mut grid = [[PlayField { field_type: Type::Empty, x: 0, y: 0 }; N]; M];

    for i in 0..M {
        for j in 0..N {
            let k: u32 = i as u32;
            let l: u32 = j as u32;

            grid[i][j].x = SNAKE_WIDTH * k;
            grid[i][j].y = SNAKE_WIDTH * l;
        }
    }

    let mut snake = snake::Snake::new();

    snake.setup_board();

    'running: loop {
        let ticks = timer.ticks() as i32;

        for i in 0..M {
            for j in 0..N {
                if i == 0 || i == 39 || j == 0 || j == 39 {
                    grid[i][j].field_type = Type::Wall;
                } else {
                    grid[i][j].field_type = Type::Empty;
                }
            }
        }



        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        grid=snake.draw_elements(grid);

        for i in 0..M {
            for j in 0..N {
                let rect: Rect = Rect::new(grid[i][j].x as i32, grid[i][j].y as i32, SNAKE_WIDTH, SNAKE_WIDTH);
                match grid[i][j].field_type {
                    Type::Empty => {
                        canvas.set_draw_color(Color::RGB(0, 0, 0));
                    }
                    Type::Wall => {
                        canvas.set_draw_color(Color::RGB(255, 255, 255));
                    }
                    Type::Snake => {
                        canvas.set_draw_color(Color::RGB(0, 255, 0));
                    }
                    Type::Apple => {
                        canvas.set_draw_color(Color::RGB(255, 0, 0));
                    }
                }

                canvas.fill_rect(rect).expect("failed?");
            }
        }

        for event in event_pump.poll_iter() {
            match game_state {
                GameState::Play => {
                    match event {
                        Event::Quit { .. } |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'running;
                        }


                        _ => {
                            snake.parse_event(event);
                        }
                    }
                }
                GameState::GameOver => {
                    match event {
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

//         render a surface, and convert it to a texture bound to the canvas
        let surface = font.render(snake.score.to_string().as_str())
            .blended(Color::RGBA(255, 0, 0, 255)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();


        let TextureQuery { width, height, .. } = texture.query();
// If the example text is too big for the screen, downscale it (and center irregardless)
        let padding = 64;
        let target = get_centered_rect(width, height, WIDTH - padding, HEIGHT - padding);

        canvas.copy(&texture, None, Some(target)).unwrap();

        canvas.present();

        loop_delay(frame_delay, &mut timer, ticks)
    }
}


fn loop_delay(frame_delay: i32, timer: &mut TimerSubsystem, ticks: i32) -> () {
    let frame_time = timer.ticks() as i32;
    let frame_time = frame_time - ticks;
    if frame_delay > frame_time {
        let sleeptime = (frame_delay + 200 - frame_time) as u64;

        std::thread::sleep(Duration::from_millis(sleeptime));
    }
}

