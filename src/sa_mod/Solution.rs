pub struct Solution {
	items : Vec<u32>,
	box_size : u32,
	boxes: u32
}

impl Solution {
	pub fn eval(&self) -> u32 {
		self.boxes = 0;
		let mut temp: u32 = 0;
		for n in 0..self.items.len() {
			temp += self.items[n as usize];
			if(temp > self.box_size) {
				temp = self.items[n as usize];
				self.boxes += 1;
			}
		}
		return self.boxes;
	}
}