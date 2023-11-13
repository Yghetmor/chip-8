extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;

pub struct Platform {
    //window: Window,
    canvas: Canvas<Window>,
    event: EventPump,
    pub quit: bool,
}

impl Platform {
    pub fn new() -> Platform {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Chip-8", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();

        Platform {
            //window : window,
            canvas : canvas,
            event : event_pump,
            quit : false,
        }
    }

    pub fn update(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn process_input(&mut self, keys: &mut [u8; 16]) {
        for event in self.event.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => self.quit = true,
                Event::KeyDown { keycode: Some(Keycode::X), ..} => keys[0] = 1,
                Event::KeyDown { keycode: Some(Keycode::Kp1), ..} => keys[1] = 1,
                Event::KeyDown { keycode: Some(Keycode::Kp2), ..} => keys[2] = 1,
                Event::KeyDown { keycode: Some(Keycode::Kp3), ..} => keys[3] = 1,
                Event::KeyDown { keycode: Some(Keycode::Q), ..} => keys[4] = 1,
                Event::KeyDown { keycode: Some(Keycode::W), ..} => keys[5] = 1,
                Event::KeyDown { keycode: Some(Keycode::E), ..} => keys[6] = 1,
                Event::KeyDown { keycode: Some(Keycode::A), ..} => keys[7] = 1,
                Event::KeyDown { keycode: Some(Keycode::S), ..} => keys[8] = 1,
                Event::KeyDown { keycode: Some(Keycode::D), ..} => keys[9] = 1,
                Event::KeyDown { keycode: Some(Keycode::Z), ..} => keys[10] = 1,
                Event::KeyDown { keycode: Some(Keycode::C), ..} => keys[11] = 1,
                Event::KeyDown { keycode: Some(Keycode::Kp4), ..} => keys[12] = 1,
                Event::KeyDown { keycode: Some(Keycode::R), ..} => keys[13] = 1,
                Event::KeyDown { keycode: Some(Keycode::F), ..} => keys[14] = 1,
                Event::KeyDown { keycode: Some(Keycode::V), ..} => keys[15] = 1,
                Event::KeyUp { keycode: Some(Keycode::X), ..} => keys[0] = 0,
                Event::KeyUp { keycode: Some(Keycode::Kp1), ..} => keys[1] = 0,
                Event::KeyUp { keycode: Some(Keycode::Kp2), ..} => keys[2] = 0,
                Event::KeyUp { keycode: Some(Keycode::Kp3), ..} => keys[3] = 0,
                Event::KeyUp { keycode: Some(Keycode::Q), ..} => keys[4] = 0,
                Event::KeyUp { keycode: Some(Keycode::W), ..} => keys[5] = 0,
                Event::KeyUp { keycode: Some(Keycode::E), ..} => keys[6] = 0,
                Event::KeyUp { keycode: Some(Keycode::A), ..} => keys[7] = 0,
                Event::KeyUp { keycode: Some(Keycode::S), ..} => keys[8] = 0,
                Event::KeyUp { keycode: Some(Keycode::D), ..} => keys[9] = 0,
                Event::KeyUp { keycode: Some(Keycode::Z), ..} => keys[10] = 0,
                Event::KeyUp { keycode: Some(Keycode::C), ..} => keys[11] = 0,
                Event::KeyUp { keycode: Some(Keycode::Kp4), ..} => keys[12] = 0,
                Event::KeyUp { keycode: Some(Keycode::R), ..} => keys[13] = 0,
                Event::KeyUp { keycode: Some(Keycode::F), ..} => keys[14] = 0,
                Event::KeyUp { keycode: Some(Keycode::V), ..} => keys[15] = 0,
                _ => {}
            }
        }
    }   
}
