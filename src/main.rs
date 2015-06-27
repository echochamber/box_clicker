extern crate rand;
extern crate piston_window;
extern crate piston;

mod block;

use piston_window::{WindowSettings, PistonWindow, rectangle, clear};

use block::{ActiveBlock, Point, Color};


fn main() {
    let mut game: Game = Game::new(640, 480);
    game.run()
}

struct ViewportDimensions {
    pub w: u32,
    pub h: u32
}

struct Game {
    current_block: ActiveBlock,
    current_mouse_coordinates: Point,
    viewport_dimensions: ViewportDimensions
}

impl Game {
    pub fn new(viewport_w: u32, viewport_h: u32) -> Game {
        let viewport_dimensions = ViewportDimensions { w: viewport_w, h: viewport_h };

        Game {
            current_block: generate_valid_block_for_viewport(&viewport_dimensions),
            viewport_dimensions: viewport_dimensions,
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
            use piston::input::Input::{Move, Press, Resize};
            use piston::input::Motion::MouseCursor;
            use piston::input::mouse::MouseButton;

            self.draw_current_box(&e);

            match e.event {
                Some(Event::Input(Move(MouseCursor(x, y)))) => {
                    self.current_mouse_coordinates.x = x;
                    self.current_mouse_coordinates.y = y;
                },
                Some(Event::Input(Press(Mouse(MouseButton::Left)))) => {
                    println!("Mouse coordinates: {:?}, Box edges: {:?}", self.current_mouse_coordinates, self.current_block.coordinates_as_array());
                    if self.current_mouse_coordinates_are_in_block() {
                        self.new_random_block();
                        println!("Hit, new block is here {:?}", self.current_block);
                    }
                },
                Some(Event::Input(Resize(w, h))) => {
                    self.update_viewport_size(w, h)
                },
                // Print all other types of inputs just for fun
                Some(Event::Input(input)) => println!("{:?}", input),
                _ => {}
            }
        }
    }

    pub fn new_random_block(&mut self){
        self.current_block = generate_valid_block_for_viewport(&self.viewport_dimensions)
    }

    pub fn update_viewport_size(&mut self, w: u32, h: u32) {
        self.viewport_dimensions.w = w;
        self.viewport_dimensions.h = h;
    }

    pub fn current_mouse_coordinates_are_in_block(&self) -> bool {
        return self.current_block.contains_point(&self.current_mouse_coordinates)
    }

    pub fn draw_current_box(&self, window: &PistonWindow) {
        window.draw_2d(|c, g| {
            clear([0.8, 0.8, 0.8, 1.0], g);

            rectangle(self.current_block.color(),
                      self.current_block.to_rectangle(),
                      c.transform,
                      g
            );
        });
    }
}

fn generate_valid_block_for_viewport(viewport_dimensions: &ViewportDimensions) -> ActiveBlock {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    let size = ((rng.gen::<u32>() % 8) + 1) * ((viewport_dimensions.w + viewport_dimensions.h) / 56);

    let x_offset = (rng.gen::<u32>() % ((viewport_dimensions.w - size))) as f64;
    let y_offset = (rng.gen::<u32>() % ((viewport_dimensions.h - size))) as f64;

    ActiveBlock::new(x_offset, y_offset, size, generate_random_color())
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