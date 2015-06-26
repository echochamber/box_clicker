extern crate rand;
extern crate piston_window;

mod block;

use piston_window::{WindowSettings, PistonWindow, rectangle, clear};


fn main() {
    let blk = block::ActiveBlock::new();
    println!("{:?}", blk);

    ///////

	let window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
		.exit_on_esc(true).into();

	for e in window {
	    e.draw_2d(|c, g| {
	        clear([1.0; 4], g);
	        rectangle([1.0, 0.0, 0.0, 1.0], // red
	                  [0.0, 0.0, 100.0, 100.0],
	                  c.transform, g);
	    });
    }
}
