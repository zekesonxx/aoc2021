#![allow(dead_code)]
use super::alu::ALU;
use super::primitives::num_digits;

#[derive(Debug, Clone)]
pub struct MONAD {
	alu: ALU
}

impl MONAD {
	pub fn new(alu: ALU) -> Self {
		MONAD {
			alu
		}
	}

	fn alu_check_serial(&mut self, serial: &[isize]) -> bool {
		match self.alu.run_with_input(serial) {
			Ok((_, _, _, 0)) => true,
			_ => false
		}
	}
	
	fn preflight_check_serial(serial: &[isize]) -> bool {
		if serial.contains(&0) {
			false
		} else if serial.len() != 14 {
			false
		} else {
			true
		}
	}

	fn check_serial(&mut self, serial: isize) -> bool {
		let serial = num_digits(serial);
		if MONAD::preflight_check_serial(&serial) {
			self.alu_check_serial(&serial)
		} else {
			false
		}
	}
}
