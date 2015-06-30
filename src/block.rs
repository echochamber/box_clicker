use std::hash::{Hash, Hasher};


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


#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Hash for Point {
	fn hash<H: Hasher>(&self, state: &mut H) {
		(self.x as u64).hash(state);
		(self.y as u64).hash(state);
	}
}

#[derive(Debug)]
pub struct ActiveBlock {
	top_left_corner: Point,
	bottom_right_corner: Point,
	color: Color
}

impl Hash for ActiveBlock {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.top_left_corner.hash(state);
		self.bottom_right_corner.hash(state);
	}
}


impl ActiveBlock {
	pub fn new(x_offset: f64, y_offset: f64, size: f64, color: Color) -> ActiveBlock {

		ActiveBlock {
			top_left_corner: Point{ x: x_offset, y: y_offset },
			bottom_right_corner: Point{ x: x_offset + size , y: y_offset + size },
			color: color
		}
	}

	pub fn contains_point(&self, point: &Point) -> bool {
		return 
			self.top_left_corner.x <= point.x &&
			self.bottom_right_corner.x >= point.x &&
			self.top_left_corner.y <= point.y &&
			self.bottom_right_corner.y >= point.y
	}

	pub fn move_block(&mut self, top_left_corner: Point) {
		self.bottom_right_corner = Point {
			x: self.bottom_right_corner.x + (self.top_left_corner.x - top_left_corner.x),
			y: self.bottom_right_corner.y + (self.top_left_corner.y - top_left_corner.y),
		};

		self.top_left_corner = top_left_corner;
	}

	pub fn color(&self) -> [f32; 4] {
		return self.color.as_array()
	}

	pub fn to_rectangle(&self) -> [f64; 4] {
		return [
			self.top_left_corner.x,
			self.top_left_corner.y,
			self.width(),
			self.height()
		];
	}

	pub fn width(&self) -> f64 {
		return self.bottom_right_corner.x - self.top_left_corner.x;
	}

	pub fn height(&self) -> f64 {
		return self.bottom_right_corner.y - self.top_left_corner.y;
	}

	pub fn overlaps(&self, other_block: &ActiveBlock) -> bool {
		if self.top_left_corner.x > other_block.bottom_right_corner.x || other_block.top_left_corner.x > self.bottom_right_corner.x {
			return false;
		}

		if self.top_left_corner.y < other_block.bottom_right_corner.y || other_block.top_left_corner.y < self.bottom_right_corner.y {
			return false
		}

		return true;
	}
}