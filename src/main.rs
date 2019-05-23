extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::TimerSubsystem;

use std::time::Duration;
use sdl2::rect::Rect;

use rand::Rng;
use std::path::Path;
use sdl2::render::TextureQuery;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const SNAKE_WIDTH: u32 = WIDTH / 40;

#[derive(Copy, Clone)]
enum Type {
    Empty,
    Wall,
    Snake,
    Apple,
}

#[derive(Copy, Clone)]
struct PlayField {
    field_type: Type,
    x: u32,
    y: u32,
}

#[derive(Copy, Clone)]
enum GameState {
    Play,
    GameOver
}

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

    let mut rng = rand::thread_rng();

    let mut wrap: bool = false;

    let mut velocity_x: i32 = 0;
    let mut velocity_y: i32 = 0;


    const M: usize = 40;
    const N: usize = 40;

    let mut score: i32 = 0;
    let mut game_state=GameState::Play;

    let mut snake_vec = Vec::new();
    snake_vec.push(PlayField { x: 20, y: 20, field_type: Type::Snake });

    let mut apple_vec = Vec::new();
    apple_vec.push(PlayField { x: 5, y: 5, field_type: Type::Apple });

    let mut grid = [[PlayField { field_type: Type::Empty, x: 0, y: 0 }; N]; M];

    for i in 0..M {
        for j in 0..N {
            let k: u32 = i as u32;
            let l: u32 = j as u32;

            grid[i][j].x = SNAKE_WIDTH * k;
            grid[i][j].y = SNAKE_WIDTH * l;
        }
    }
    add_snake_segment(&mut velocity_x, &mut velocity_y, &mut snake_vec);
    add_snake_segment(&mut velocity_x, &mut velocity_y, &mut snake_vec);
    add_snake_segment(&mut velocity_x, &mut velocity_y, &mut snake_vec);

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

//        println!("-------------------");
        for snake_elem in &snake_vec {
//            println!("x1 {} y1 {} wrap {} vx {} vy {}", snake_elem.x,snake_elem.y,wrap,velocity_x,velocity_y);
            grid[snake_elem.x as usize][snake_elem.y as usize].field_type = snake_elem.field_type;
        }

        for apple_elem in &apple_vec {
            grid[apple_elem.x as usize][apple_elem.y as usize].field_type = apple_elem.field_type;
        }

//        println!("-------------------");
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

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
                        Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                            velocity_y = -1;
                            velocity_x = 0;
                        }
                        Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                            velocity_y = 1;
                            velocity_x = 0;
                        }
                        Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                            velocity_y = 0;
                            velocity_x = -1;
                        }
                        Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                            velocity_y = 0;
                            velocity_x = 1;
                        }
                        _ => {}
                    }
                }
                GameState::GameOver => {
                    match event {
                        Event::Quit { .. } |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'running;
                        }
                        Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                            score=0;
                            velocity_x=0;
                            velocity_y=0;
                            snake_vec = Vec::new();
                            snake_vec.push(PlayField { x: 20, y: 20, field_type: Type::Snake });
                            apple_vec = Vec::new();
                            apple_vec.push(PlayField { x: 5, y: 5, field_type: Type::Apple });
                            game_state = GameState::Play;
                        }
                        _ => {}
                    }
                }
            }
        }

        match game_state {
            GameState::Play => {
                move_snake_parts(velocity_x, velocity_y, &mut snake_vec);
            }
            _ => {}
        }


        match game_state {
            GameState::Play => {
                let mut snake_elem = snake_vec[0];

                snake_elem.x = move_snake(&mut velocity_x, snake_elem.x as usize, &mut wrap) as u32;
                snake_elem.y = move_snake(&mut velocity_y, snake_elem.y as usize, &mut wrap) as u32;
                snake_vec[0] = snake_elem;

                if snake_vec[0].x == apple_vec[0].x && snake_vec[0].y == apple_vec[0].y {
                    apple_vec[0].x = rng.gen_range(1, SNAKE_WIDTH);
                    apple_vec[0].y = rng.gen_range(1, SNAKE_WIDTH);
                    score = score + 1;

                    add_snake_segment(&mut velocity_x, &mut velocity_y, &mut snake_vec)
                }

                for pos in (1..snake_vec.len()).rev() {
                    if snake_vec[0].x == snake_vec[pos].x && snake_vec[0].y == snake_vec[pos].y {
                        game_state = GameState::GameOver;
                    }
                }
            }
            _ => {}
        }


//        println!("-------------------");

        // render a surface, and convert it to a texture bound to the canvas
        let surface = font.render(score.to_string().as_str())
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

fn add_snake_segment(velocity_x: &mut i32, velocity_y: &mut i32, snake_vec: &mut Vec<PlayField>) -> () {
    match velocity_x {
        1 => {
            &snake_vec.push(PlayField { x: snake_vec.last().unwrap().x - 1, y: snake_vec.last().unwrap().y, field_type: Type::Snake });
        }
        -1 => {
            &snake_vec.push(PlayField { x: snake_vec.last().unwrap().x + 1, y: snake_vec.last().unwrap().y, field_type: Type::Snake });
        }
        _ => {}
    }
    match velocity_y {
        1 => {
            &snake_vec.push(PlayField { y: snake_vec.last().unwrap().x - 1, x: snake_vec.last().unwrap().y, field_type: Type::Snake });
        }
        -1 => {
            &snake_vec.push(PlayField { y: snake_vec.last().unwrap().x + 1, x: snake_vec.last().unwrap().y, field_type: Type::Snake });
        }
        _ => {}
    }
}

fn move_snake_parts(velocity_x: i32, velocity_y: i32, snake_vec: &mut Vec<PlayField>) -> () {
    if velocity_x != 0 || velocity_y != 0 {
        for pos in (1..snake_vec.len()).rev() {
            snake_vec[pos].x = snake_vec[pos - 1].x;
            snake_vec[pos].y = snake_vec[pos - 1].y;
        }
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

fn move_snake(velocity: &mut i32, coordinate: usize, wrap: &mut bool) -> usize {
    let mut coordinate_new: usize = coordinate;
    if !*wrap && coordinate == 1 && *velocity == -1 as i32
    {
        coordinate_new = 38;
        *wrap = true;
    } else if !*wrap && coordinate == 38 && *velocity == 1 as i32 {
        coordinate_new = 1;
        *wrap = true;
    } else {
        if *velocity != 0 {
            if *velocity == 1 {
                coordinate_new = coordinate + 1;
            } else if *velocity == -1 {
                coordinate_new = coordinate - 1;
            }
            *wrap = false;
        }
    }
    return coordinate_new;
}
