extern crate rand;
extern crate piston_window;
extern crate piston;

mod block;

use piston_window::{WindowSettings, PistonWindow, rectangle, clear};

use block::{ActiveBlock, Point};


fn main() {
    let mut game: Game = Game::new();
    game.run()
}

struct Game {
    current_block: ActiveBlock,
    current_mouse_coordinates: Point
}

impl Game {
    pub fn new() -> Game {
        let active_block = ActiveBlock::new();

        Game {
            current_block: active_block,
            current_mouse_coordinates: Point {
                x: 0f64,
                y: 0f64
            }
        }
    }

    pub fn run(&mut self) {
        let window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
            .exit_on_esc(true).into();
        for e in window {
            use piston::input::Button::*;
            use piston::event::Event;
            use piston::input::Input::{Move, Press};
            use piston::input::Motion::MouseCursor;
            use piston::input::mouse::MouseButton;

            self.draw_current_box(&e);

            // Destructuring
            // e.event returns an Option<Event> enum variant (Option::Some or Option::None)
            // Event is also an enum ()
            match e.event {
                Some(Event::Input(Move(MouseCursor(x, y)))) => {
                    self.current_mouse_coordinates.x = x;
                    self.current_mouse_coordinates.y = y;
                },
                Some(Event::Input(Press(Mouse(MouseButton::Left)))) => {
                    println!("Mouse coordinates: {:?}, Box edges: {:?}", self.current_mouse_coordinates, self.current_block.coordinates_as_array());
                    if self.current_mouse_coordinates_are_in_block() {
                        
                        self.current_block = ActiveBlock::new();
                        println!("Hit, new block is here {:?}", self.current_block);
                    }
                },
                // Print all other types of inputs just for fun
                Some(Event::Input(input)) => println!("{:?}", input),
                _ => {}
            }
        }
    }

    pub fn current_mouse_coordinates_are_in_block(&self) -> bool {
        return self.current_block.contains_point(&self.current_mouse_coordinates)
    }

    pub fn draw_current_box(&self, window: &PistonWindow) {
        window.draw_2d(|c, g| {
            // clears the screen, first argument is the background color, of type Color (which is [ColorComponent; 4]). ColorComponent is just an f32.
            // So the first aregument ends up as an array of 4 f32 numbers. The second argument type is graphics (duh).
            clear([0.8, 0.8, 0.8, 1.0], g);

            // Creates a rectangle. First argument is same type as first argument of clear (Color aka [f32; 4]). 
            // Second arg is a type where Into<types::Rectangle> is defined (which it is for [f32, 4]).
            // Third arg type is math::Matrix2d
            // Fourth is graphics
            rectangle(self.current_block.color(), // red
                      self.current_block.to_rectangle(),
                      c.transform,
                      g
            );
        });
    }
}