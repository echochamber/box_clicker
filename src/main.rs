#![feature(hash_default)]

extern crate rand;
extern crate piston_window;
extern crate piston;

mod block;

use piston_window::{WindowSettings, PistonWindow, rectangle, clear};

use block::{ActiveBlock, Point, Color};
use std::hash::{hash, Hash, SipHasher};

use std::collections::HashMap;



fn main() {
    let mut game: Game = Game::new(640, 480);
    game.run()
}

#[derive(Debug, Copy, Clone)]
struct ViewportDimensions {
    pub w: u32,
    pub h: u32
}

#[derive(Debug)]
struct Game {
    current_blocks: HashMap<u64, ActiveBlock>,
    current_mouse_coordinates: Point,
    viewport_dimensions: ViewportDimensions
}

impl Game {
    pub fn new(viewport_w: u32, viewport_h: u32) -> Game {
        let viewport_dimensions = ViewportDimensions { w: viewport_w, h: viewport_h };

        let mut game = Game {
            current_blocks: HashMap::new(),
            viewport_dimensions: viewport_dimensions,
            current_mouse_coordinates: Point {
                x: 0f64,
                y: 0f64
            }
        };

        let first_block = generate_valid_block_for_viewport(&viewport_dimensions);
        game.current_blocks.insert(hash::<_, SipHasher>(&first_block), first_block);

        game
    }

    pub fn run(&mut self) {
        let window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
            .exit_on_esc(true).into();
        for e in window {
            use piston::input::Button::*;
            use piston::event::Event;
            use piston::input::Input::{Move, Press, Resize};
            use piston::input::Motion::MouseCursor;
            use piston::input::mouse::MouseButton;
            use piston::input::keyboard::Key;

            self.draw_current_blocks(&e);

            match e.event {
                Some(Event::Input(Move(MouseCursor(x, y)))) => {
                    self.current_mouse_coordinates.x = x;
                    self.current_mouse_coordinates.y = y;
                },
                Some(Event::Input(Press(Mouse(MouseButton::Left)))) => {
                    println!("Mouse coordinates: {:?}", self.current_mouse_coordinates);


                    match self.current_mouse_coordinates_are_in_block() {
                        Some(key) => {
                            {
                                self.current_blocks.remove(&key);
                            }
                            {
                                let new_block = self.new_random_block();
                                println!("Hit, new block is here {:?}", new_block);
                                self.current_blocks.insert(hash::<_, SipHasher>(&new_block), new_block);
                            }
                        },
                        None => {}
                    }
                },
                Some(Event::Input(Resize(w, h))) => {
                    self.update_viewport_size(w, h)
                },
                // N key spawns new boxes
                Some(Event::Input(Press(Keyboard(Key::N)))) => {
                    let new_block = self.new_random_block();
                    println!("Hit, new block is here {:?}", new_block);
                    self.current_blocks.insert(hash::<_, SipHasher>(&new_block), new_block);
                },
                // Print all other types of inputs just for fun
                Some(Event::Input(input)) => println!("{:?}", input),
                _ => {}
            }
        }
    }

    pub fn new_random_block(&self) -> ActiveBlock {
        let mut done = false;
        let mut count = 0;
        let mut block: &ActiveBlock = &generate_valid_block_for_viewport(&self.viewport_dimensions);

        while !done || count > 100 {
            done = true;
            for (key, current_block) in self.current_blocks.iter() {
                if (current_block.overlaps(block)) {
                    done = false;
                }
            }

            let block = generate_valid_block_for_viewport(&self.viewport_dimensions);
            count = count + 1;
        }
        generate_valid_block_for_viewport(&self.viewport_dimensions)
    }

    pub fn update_viewport_size(&mut self, w: u32, h: u32) {
        self.viewport_dimensions.w = w;
        self.viewport_dimensions.h = h;
    }

    pub fn current_mouse_coordinates_are_in_block(&self) -> Option<u64> {
        for (key, current_block) in self.current_blocks.iter() {
            if current_block.contains_point(&self.current_mouse_coordinates) {
                return Some(key.clone())
            }
        }

        return None;
    }

    pub fn draw_current_blocks(&self, window: &PistonWindow) {
        window.draw_2d(|c, g| {
            clear([0.8, 0.8, 0.8, 1.0], g);

            for (key, current_block) in self.current_blocks.iter() {
                rectangle(current_block.color(),
                      current_block.to_rectangle(),
                      c.transform,
                      g
                );
            }
        });
    }
}

fn generate_valid_block_for_viewport(viewport_dimensions: &ViewportDimensions) -> ActiveBlock {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    let size = ((rng.gen::<u32>() % 8) + 1) * ((viewport_dimensions.w + viewport_dimensions.h) / 56);

    let x_offset = (rng.gen::<u32>() % ((viewport_dimensions.w - size))) as f64;
    let y_offset = (rng.gen::<u32>() % ((viewport_dimensions.h - size))) as f64;

    ActiveBlock::new(x_offset, y_offset, size as f64, generate_random_color())
}

fn generate_random_color() -> Color {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let random_color_r = ((rng.gen::<u32>() % 100) as f32) / 100.0;
    let random_color_g = ((rng.gen::<u32>() % 100) as f32) / 100.0;
    let random_color_b = ((rng.gen::<u32>() % 100) as f32) / 100.0;
    let random_color_a = ((rng.gen::<u32>() % 50) as f32) / 100.0 + 0.5;

    Color::new(random_color_r, random_color_g, random_color_b, random_color_a)
}