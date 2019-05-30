extern crate rand;

use rand::Rng;

use crate::constants::PlayField;
use crate::constants::Type;
use crate::constants::GameState;
use crate::constants::Keyboard;
use crate::events::SnakeEvent;

const WIDTH: u32 = 800;
const M: usize = 40;
const N: usize = 40;
const SNAKE_WIDTH: u32 = WIDTH / 40;

pub struct Snake {
    rng: rand::rngs::ThreadRng,
    game_state: GameState,
    wrap: bool,
    velocity_x: i32,
    velocity_y: i32,
    pub score: i32,
    snake_vec: Vec<PlayField>,
    apple_vec: Vec<PlayField>,
    grid: [[PlayField; 40]; 40],
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            game_state: crate::constants::GameState::Play,
            score: 0,
            velocity_y: 0,
            velocity_x: 0,
            snake_vec: Vec::new(),
            apple_vec: Vec::new(),
            wrap: false,
            rng: rand::thread_rng(),
            grid: [[PlayField { field_type: Type::Empty, x: 0, y: 0 }; N]; M],
        }
    }
    pub fn setup_board(&mut self) -> () {
        for i in 0..M {
            for j in 0..N {
                let k: u32 = i as u32;
                let l: u32 = j as u32;

                self.grid[i][j].x = SNAKE_WIDTH * k;
                self.grid[i][j].y = SNAKE_WIDTH * l;
            }
        }

        self.score = 0;
        self.velocity_x = 0;
        self.velocity_y = 0;
        self.snake_vec.clear();
        self.snake_vec.push(PlayField { x: 20, y: 20, field_type: Type::Snake });

        self.add_snake_segment();

        self.apple_vec.clear();
        self.apple_vec.push(PlayField { x: 0, y: 0, field_type: Type::Apple });
        self.set_apple();

        self.game_state = GameState::Play;
    }

    fn set_apple(&mut self) {
        self.apple_vec[0].x = self.rng.gen_range(1, SNAKE_WIDTH);
        self.apple_vec[0].y = self.rng.gen_range(1, SNAKE_WIDTH);
    }

    fn add_snake_segment(&mut self) -> () {
        match self.velocity_x {
            1 => {
                self.snake_vec.push(PlayField { x: self.snake_vec.last().unwrap().x - 1, y: self.snake_vec.last().unwrap().y, field_type: Type::Snake });
            }
            -1 => {
                self.snake_vec.push(PlayField { x: self.snake_vec.last().unwrap().x + 1, y: self.snake_vec.last().unwrap().y, field_type: Type::Snake });
            }
            _ => {}
        }
        match self.velocity_y {
            1 => {
                self.snake_vec.push(PlayField { y: self.snake_vec.last().unwrap().x - 1, x: self.snake_vec.last().unwrap().y, field_type: Type::Snake });
            }
            -1 => {
                self.snake_vec.push(PlayField { y: self.snake_vec.last().unwrap().x + 1, x: self.snake_vec.last().unwrap().y, field_type: Type::Snake });
            }
            _ => {}
        }
    }

    fn move_snake_parts(&mut self) -> () {
        if self.velocity_x != 0 || self.velocity_y != 0 {
            for pos in (1..self.snake_vec.len()).rev() {
                self.snake_vec[pos].x = self.snake_vec[pos - 1].x;
                self.snake_vec[pos].y = self.snake_vec[pos - 1].y;
            }
        }
    }

    fn move_snake(&mut self, velocity: i32, coordinate: usize) -> usize {
        let mut coordinate_new: usize = coordinate;
        if !self.wrap && coordinate == 1 && velocity == -1 as i32
        {
            coordinate_new = 38;
            self.wrap = true;
        } else if !self.wrap && coordinate == 38 && velocity == 1 as i32 {
            coordinate_new = 1;
            self.wrap = true;
        } else {
            if velocity != 0 {
                if velocity == 1 {
                    coordinate_new = coordinate + 1;
                } else if velocity == -1 {
                    coordinate_new = coordinate - 1;
                }
                self.wrap = false;
            }
        }
        return coordinate_new;
    }

    pub fn play_state(&mut self) -> GameState {
        self.move_snake_parts();

        let mut snake_elem = self.snake_vec[0];

        snake_elem.x = self.move_snake(self.velocity_x, snake_elem.x as usize) as u32;
        snake_elem.y = self.move_snake(self.velocity_y, snake_elem.y as usize) as u32;
        self.snake_vec[0] = snake_elem;

        if self.snake_vec[0].x == self.apple_vec[0].x && self.snake_vec[0].y == self.apple_vec[0].y {
            self.set_apple();
            self.score = self.score + 1;

            self.add_snake_segment();
        }

        for pos in (1..self.snake_vec.len()).rev() {
            if self.snake_vec[0].x == self.snake_vec[pos].x && self.snake_vec[0].y == self.snake_vec[pos].y {
                self.game_state = GameState::GameOver;
            }
        }

        return self.game_state;
    }

    pub fn draw_elements(&mut self,) -> [[PlayField; 40]; 40] {
        self.setup_grid();

        for snake_elem in self.snake_vec.clone() {
            println!("x1 {} y1 {} wrap {} vx {} vy {}", snake_elem.x, snake_elem.y, self.wrap, self.velocity_x, self.velocity_y);
            self.grid[snake_elem.x as usize][snake_elem.y as usize].field_type = snake_elem.field_type;
        }

        for apple_elem in self.apple_vec.clone() {
            self.grid[apple_elem.x as usize][apple_elem.y as usize].field_type = apple_elem.field_type;
        }

        self.grid
    }

    pub fn parse_event(&mut self, snake_event: SnakeEvent) -> () {
        let key = snake_event.get_key();
        match key {
            Keyboard::Up => {
                self.velocity_y = -1;
                self.velocity_x = 0;
            }
            Keyboard::Down => {
                self.velocity_y = 1;
                self.velocity_x = 0;
            }
            Keyboard::Left => {
                self.velocity_y = 0;
                self.velocity_x = -1;
            }
            Keyboard::Right => {
                self.velocity_y = 0;
                self.velocity_x = 1;
            }
            _ => {}
        }
    }
    fn setup_grid(&mut self) {
        for i in 0..M {
            for j in 0..N {
                if i == 0 || i == 39 || j == 0 || j == 39 {
                    self.grid[i][j].field_type = Type::Wall;
                } else {
                    self.grid[i][j].field_type = Type::Empty;
                }
            }
        }
    }
}