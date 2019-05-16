extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::TimerSubsystem;

use std::time::Duration;
use sdl2::rect::Rect;
use std::convert::TryFrom;

use rand::Rng;

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

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("SDL2 Snake", WIDTH+200, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let frame_delay = 1000 / 60;
    let mut timer = sdl_context.timer().unwrap();

    let mut rng = rand::thread_rng();

    let mut wrap: bool = false;

    let mut velocity_x: i32 = 0;
    let mut velocity_y: i32 = 0;

    let x: usize = 20;
    let y: usize = 20;

    const M: usize = 40;
    const N: usize = 40;

    #[derive(Copy, Clone)]
    let mut snake_vec= Vec::new();
    snake_vec.push(PlayField{x:20,y:20,field_type:Type::Snake});
    snake_vec.push(PlayField{x:20,y:21,field_type:Type::Snake});
    snake_vec.push(PlayField{x:20,y:22,field_type:Type::Snake});
    snake_vec.push(PlayField{x:20,y:23,field_type:Type::Snake});

    #[derive(Copy, Clone)]
    let mut apple_vec= Vec::new();
    apple_vec.push(PlayField{x:5,y:5,field_type:Type::Apple});

    let mut grid = [[PlayField { field_type: Type::Empty, x: 0, y: 0 }; N]; M];

    for i in 0..M {
        for j in 0..N {
            let k: u32 = i as u32;
            let l: u32 = j as u32;

            grid[i][j].x = SNAKE_WIDTH * k;
            grid[i][j].y = SNAKE_WIDTH * l;
        }
    }

    'running: loop {
        let ticks = timer.ticks() as i32;

        for i in 0..M {
            for j in 0..N {
                if i == 0 || i == 39 || j == 0 || j == 39 {
                    grid[i][j].field_type = Type::Wall;
                }else{
                    grid[i][j].field_type = Type::Empty;
                }
            }
        }

        println!("-------------------");
        for snake_elem in &snake_vec{
            println!("y {} x {} x1 {} y1 {}", y, x, snake_elem.x,snake_elem.y);
            grid[snake_elem.x as usize][snake_elem.y as usize].field_type = snake_elem.field_type;
        }

        for apple_elem in &apple_vec {
            grid[apple_elem.x as usize][apple_elem.y as usize].field_type = apple_elem.field_type;
        }

        println!("-------------------");
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let rect: Rect = Rect::new(x as i32, y as i32, SNAKE_WIDTH, SNAKE_WIDTH);
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(rect).expect("failed?");

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

        if velocity_x!=0 || velocity_y !=0 {
            for pos in (1..snake_vec.len()).rev() { 
                snake_vec[pos].x=snake_vec[pos-1].x;
                snake_vec[pos].y=snake_vec[pos-1].y;
            }
        }
        let mut snake_elem = snake_vec[0];

        snake_elem.x = move_snake(&mut velocity_x, snake_elem.x as usize, &mut wrap) as u32;
        snake_elem.y = move_snake(&mut velocity_y, snake_elem.y as usize, &mut wrap) as u32;
        snake_vec[0]=snake_elem;

        if snake_vec[0].x == apple_vec[0].x && snake_vec[0].y == apple_vec[0].y {
            println!("HHHHHHHHHHHHHHHHIIIIIIIIIIIIIIIIIIIIIITTTTTTTTTTTTTTTTTT");
            apple_vec[0].x=rng.gen_range(1, SNAKE_WIDTH);
            apple_vec[0].y=rng.gen_range(1, SNAKE_WIDTH);
        }
        println!("-------------------");

        canvas.present();

        loop_delay(frame_delay, &mut timer, ticks)
    }
}

fn loop_delay(frame_delay: i32, timer: &mut TimerSubsystem, ticks: i32) -> () {
    let frame_time = timer.ticks() as i32;
    let frame_time = frame_time - ticks;
    if frame_delay > frame_time {
        let sleeptime = (frame_delay + 250 - frame_time) as u64;

        std::thread::sleep(Duration::from_millis(sleeptime));
    }
}

fn move_snake(velocity: &mut i32, coordinate: usize, wrap: &mut bool) -> usize {
    let mut coordinate_new:usize = coordinate;
    if !*wrap && coordinate == 1
    {
        coordinate_new = 39;
        *wrap = true;
    } else if !*wrap && coordinate == 39 {
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
