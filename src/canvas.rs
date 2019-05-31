use sdl2::pixels::Color;

use std::time::Duration;
use sdl2::rect::Rect;

use std::path::Path;
use sdl2::render::TextureQuery;

use crate::constants::Type;
use crate::constants::PlayField;
use sdl2::{ttf};
use crate::constants::Keyboard;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::constants::GameState;
use crate::events::SnakeEvent;
use crate::snake::Snake;

pub struct SdlCanvas<'a, 'b> {
    font: sdl2::ttf::Font<'a, 'b>,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    timer: sdl2::TimerSubsystem,
    frame_delay: i32,
    ticks: i32,
    sdl_context: sdl2::Sdl,
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const SNAKE_WIDTH: u32 = WIDTH / 40;
const M: usize = 40;
const N: usize = 40;
// handle the annoying Rect i32
macro_rules! rect (
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

trait Canvas {}

impl<'a, 'b> Canvas for SdlCanvas<'a, 'b> {}


impl<'a, 'b> SdlCanvas<'a, 'b> {
    pub fn new(ttf_context: &'a ttf::Sdl2TtfContext) -> SdlCanvas<'a, 'b> {
        let sdl_context = sdl2::init().unwrap();

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("SDL2 Snake", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let font_path: &Path = Path::new("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf");

        let mut font = ttf_context.load_font(font_path, 128).unwrap();

        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let canvas = window.into_canvas().build().unwrap();

        let timer = sdl_context.timer().unwrap();

        let texture_creator = canvas.texture_creator();

        SdlCanvas {
            font,
            canvas,
            texture_creator,
            timer,
            frame_delay: 1000 / 60,
            ticks: 0,
            sdl_context,
        }
    }

    // Scale fonts to a reasonable size when they're too big (though they might look less smooth)
    fn get_centered_rect(&self, rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
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

    pub fn create_score_text(&mut self, text: &str) -> () {
//             render a surface, and convert it to a texture bound to the canvas
        let surface = self.font.render(text)
            .blended(Color::RGBA(255, 0, 0, 255)).unwrap();
        let texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();


        let TextureQuery { width, height, .. } = texture.query();
// If the example text is too big for the screen, downscale it (and center irregardless)
        let padding = 64;
        let target = self.get_centered_rect(width, height, WIDTH - padding, HEIGHT - padding);
        self.canvas.copy(&texture, None, Some(target)).unwrap();
    }

    pub fn start_loop(&mut self) -> () {
        self.ticks = self.timer.ticks() as i32;
    }

    pub fn loop_delay(&mut self) -> () {
        let frame_time = self.timer.ticks() as i32;
        let frame_time = frame_time - self.ticks;
        if self.frame_delay > frame_time {
            let sleeptime = (self.frame_delay + 150 - frame_time) as u64;

            std::thread::sleep(Duration::from_millis(sleeptime));
        }
    }

    pub fn draw(&mut self, grid: [[PlayField; 40]; 40]) -> () {
        for i in 0..M {
            for j in 0..N {
                let rect: Rect = Rect::new(grid[i][j].x as i32, grid[i][j].y as i32, SNAKE_WIDTH, SNAKE_WIDTH);
                match grid[i][j].field_type {
                    Type::Empty => {
                        self.color_black();
                    }
                    Type::Wall => {
                        self.color_white();
                    }
                    Type::Snake => {
                        self.color_green();
                    }
                    Type::Apple => {
                        self.color_red();
                    }
                }

                self.canvas.fill_rect(rect).expect("failed?");
            }
        }
    }

    fn color_red(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
    }

    fn color_green(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
    }

    fn color_white(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
    }

    fn color_black(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    }

    pub fn clear(&mut self) -> () {
        self.color_black();
        self.canvas.clear();
    }

    pub fn present(&mut self) -> () {
        self.canvas.present();
    }


    fn parse_event(&self, snake: &mut Snake, snake_event: SnakeEvent) -> () {
        let key = snake_event.get_key();
        match key {
            Keyboard::Up => snake.move_up(),
            Keyboard::Down => snake.move_down(),
            Keyboard::Left => snake.move_left(),
            Keyboard::Right => snake.move_right(),
            _ => {}
        }
    }

    pub fn start(&mut self, mut snake: &mut Snake) -> () {
        let mut game_state = GameState::Play;

        snake.setup_board();

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop {
            self.start_loop();

            self.clear();

            let grid = snake.draw_elements();

            self.draw(grid);

            for sdl_event in event_pump.poll_iter() {
                match game_state {
                    GameState::Play => {
                        match sdl_event {
                            Event::Quit { .. } |
                            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                                break 'running;
                            }

                            _ => {
                                self.parse_event(&mut snake, SnakeEvent::new(sdl_event));
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

            self.create_score_text(snake.score.to_string().as_str());

            self.present();

            self.loop_delay()
        }
    }
}