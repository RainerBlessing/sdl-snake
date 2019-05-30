extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::constants::Keyboard;

pub struct SnakeEvent {
    sdl_event: Event
}

impl SnakeEvent{
    pub fn new(sdl_event:Event) -> SnakeEvent {
        SnakeEvent {
            sdl_event:sdl_event,
        }
    }
    pub fn get_key(&self) -> Keyboard {
                match self.sdl_event{
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                Keyboard::Up
            }
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                Keyboard::Down
            }
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                Keyboard::Left
            }
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                Keyboard::Right
            }
            _ => {
                Keyboard::Unknown
            }
        }

    }
}