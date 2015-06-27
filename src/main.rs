extern crate rand;
extern crate piston_window;
extern crate piston;

mod block;

use piston_window::{WindowSettings, PistonWindow, rectangle, clear};


fn main() {
    let blk = block::ActiveBlock::new();
    println!("{:?}", blk);

    ///////

    // What exactly does the into trait do? Defined here
    // https://github.com/PistonDevelopers/piston/blob/b139e853d3111e8156e92078266e1605996e81cf/src/window/src/lib.rs#L135
    let window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).into();


    

    for e in window {
        use piston::input::Button;
        use piston::event::Event;

        if let Some(event) = e.event {
            if let Event::Input(input) = event {
                println!("{:?}", input);
            }

        }
        //updateWindow(&e);
    }
}

fn updateWindow(window: & PistonWindow) {
    // Whats going on in draw_2d
    // https://github.com/PistonDevelopers/piston_window/blob/master/src/lib.rs#L128
    // |c, g| is a closure, and draw_2d specifies the required argument types, c must be of type Context, and g must be of type GfxGraphics
    // The return value of the expression contained in the following brackets is return value of the closure. 
    // (But that turns out to be nothing (unit) since the last line in that expression ends in a semicolon). So draw_2d returns unit ()
    window.draw_2d(|c, g| {
        // clears the screen, first argument is the background color, of type Color (which is [ColorComponent; 4]). ColorComponent is just an f32.
        // So the first aregument ends up as an array of 4 f32 numbers. The second argument type is graphics (duh).
        clear([1.0; 4], g);

        // Creates a rectangle. First argument is same type as first argument of clear (Color aka [f32; 4]). 
        // Second arg is a type where Into<types::Rectangle> is defined (which it is for [f32, 4]).
        // Third arg type is math::Matrix2d
        // Fourth is graphics
        rectangle([1.0, 0.0, 0.0, 1.0], // red
                  [0.0, 0.0, 100.0, 100.0],
                  c.transform, g);
    });
}