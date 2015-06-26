extern crate rand;

pub const GRID_WIDTH: u32 = 100;
pub const GRID_HEIGHT: u32 = 100;


#[derive(Debug)]
pub struct Color {
	r: f32,
	g: f32,
	b: f32,
	a: f32 
}

impl Color {
	pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
		Color {
			r: r,
			g: g,
			b: b,
			a: a
		}
	}

	// pub fn as_rgba(&self) -> [f32; 4] {
	// 	match *self {
	// 		Cyan	=> [0.0, 1.0, 1.0, 1.0],
	// 		Blue	=> [0.0, 0.5, 1.0, 1.0],
	// 		Orange	=> [1.0, 0.6, 0.0, 1.0],
	// 		Yellow	=> [1.0, 1.0, 0.0, 1.0],
	// 		Lime	=> [0.5, 1.0, 0.0, 1.0],
	// 		Purple	=> [0.8, 0.0, 1.0, 1.0],
	// 		Red		=> [1.0, 0.0, 0.0, 1.0]
	// 	}
	// }
}

#[derive(Debug)]
pub struct Block {
	color: Color,
	width: u32,
	height: u32
}

impl Block {
	pub fn new(color: Color, width: u32, height: u32) -> Block {
		Block {
			color: color,
			width: width,
			height: height
		}
	}
}

#[derive(Debug)]
pub struct ActiveBlock {
	x_coordinate: u32,
	y_coordinate: u32,
	block: Block 
}


impl ActiveBlock {
	pub fn new() -> ActiveBlock {
		use rand::Rng;

		let mut rng = rand::thread_rng();

  		let size = (rng.gen::<u32>() % 8) + 1;

  		let x_center_offet = rng.gen::<u32>() % ((GRID_WIDTH - size) / 2);
  		let y_center_offet = rng.gen::<u32>() % ((GRID_HEIGHT - size) / 2);

  		
  		// How do I pick a color randomly from an enum

  		let color = Color::new(0.0, 1.0, 1.0, 1.0);

  		let block = Block::new(color, size, size);

		ActiveBlock {
			x_coordinate: x_center_offet,
			y_coordinate: y_center_offet,
			block: block 
		}
	}
}