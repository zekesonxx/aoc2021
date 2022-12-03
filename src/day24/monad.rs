#[derive(Default, Debug, Clone, Copy)]
pub struct MONAD {
	next_digit: u8,
	output: isize,
}

impl MONAD {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn reset(&mut self) {
		self.next_digit = 0;
		self.output = 0;
	}
	pub fn is_done(&self) -> bool {
		self.next_digit > 13
	}
	pub fn consts(digit: u8) -> (isize, isize, bool) {
		match digit {
			0 =>  (13,  6, false),
			1 =>  (15,  7, false),
			2 =>  (15, 10, false),
			3 =>  (11,  2, false),
			4 =>  (-7, 15, true),
			5 =>  (10,  8, false),
			6 =>  (10,  1, false),
			7 =>  (-5, 10, true),
			8 =>  (15,  5, false),
			9 =>  (-3,  3, true),
			10 => ( 0,  5, true),
			11 => (-5, 11, true),
			12 => (-9, 12, true),
			13 =>  (0, 10, true),
			_ => panic!("bad digit const reqested")
		}
	}
	pub fn step(&mut self, input: isize) {
		if self.is_done() { return; }
		assert!(input > 0 && input < 10);
		let (a, b, dodiv) = MONAD::consts(self.next_digit);
		let w=input;
		let x = self.output%26;
		if dodiv {
			self.output /= 26;
		}
		if w != x+a {
			self.output *= 26;
			self.output += w+b; 
		}
		self.next_digit += 1;
	}
	/// Returns a list of digits that if provided next
	/// wouldn't trigger the multiplication
	pub fn lookahead_digits(&self) -> Vec<isize> {
		if self.is_done() { return vec![]; }
		let (a, _, _) = MONAD::consts(self.next_digit);
		let x = self.output%26;
		(1..=9).filter(|w| *w==x+a).collect()
	}

	pub fn get_output(&self) -> isize {
		self.output
	}
}
