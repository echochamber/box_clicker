extern crate rand;

pub const GRID_WIDTH: u64 = 640u64;
pub const GRID_HEIGHT: u64 = 480u64;


#[derive(Debug)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32 
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

	pub fn as_array(&self) -> [f32; 4] {
		[self.r, self.g, self.b, self.a]
	}
}


#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64
}


#[derive(Debug)]
pub struct Block {
	pub color: Color,
	pub width: f64,
	pub height: f64
}

impl Block {
	pub fn new(color: Color, width: f64, height: f64) -> Block {
		Block {
			color: color,
			width: width,
			height: height
		}
	}
}

#[derive(Debug)]
pub struct ActiveBlock {
	top_left_corner: Point,
	block: Block 
}


impl ActiveBlock {
	pub fn new() -> ActiveBlock {
		use rand::Rng;

		let mut rng = rand::thread_rng();

  		let size = ((rng.gen::<u64>() % 8) + 1) * 20;

  		let x_offet = (rng.gen::<u64>() % ((GRID_WIDTH - size))) as f64;
  		let y_offet = (rng.gen::<u64>() % ((GRID_HEIGHT - size))) as f64;

  		let random_color_r = ((rng.gen::<u32>() % 100) as f32) / 100.0;
  		let random_color_g = ((rng.gen::<u32>() % 100) as f32) / 100.0;
  		let random_color_b = ((rng.gen::<u32>() % 100) as f32) / 100.0;
  		let random_color_a = ((rng.gen::<u32>() % 50) as f32) / 100.0 + 0.5;

  		let color = Color::new(random_color_r, random_color_g, random_color_b, random_color_a);

  		let block = Block::new(color, size as f64, size as f64);

		ActiveBlock {
			top_left_corner: Point{ x: x_offet, y: y_offet },
			block: block 
		}
	}

	pub fn contains_point(&self, point: &Point) -> bool {
		return self.top_left_corner.x <= point.x &&
			self.top_left_corner.y <= point.y &&
			self.top_left_corner.x + self.block.width >= point.x &&
			self.top_left_corner.y + self.block.height >= point.y
	}

	pub fn moveBlock(&mut self, point: Point) {
		self.top_left_corner = point
	}

	pub fn color(&self) -> [f32; 4] {
		return self.block.color.as_array()
	}

	pub fn coordinates_as_array(&self) -> [f64; 4] {
		return [
			self.top_left_corner.x,
			self.top_left_corner.y,
			self.top_left_corner.x + self.block.width,
			self.top_left_corner.y + self.block.height
		];
	}

	pub fn to_rectangle(&self) -> [f64; 4] {
		return [
			self.top_left_corner.x,
			self.top_left_corner.y,
			self.block.width,
			self.block.height
		];
	}
}