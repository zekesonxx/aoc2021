#![allow(dead_code)]

use super::primitives::*;
use super::optimizedop::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ALUError {
	ProgramEnded,
	InpWhenOutOfInput,
	DivByZero,
	ModOnNegative,
	ModByNegativeOrZero,
}

#[derive(Debug, Clone, Default)]
pub struct ALU {
	/// input characters to be provided to inp opcodes
	input: Vec<isize>,
	// input counter, index into input
	ic: usize,
	/// list of instructions
	program: Vec<Op>,
	/// program counter, index into program
	pc: usize,
	w: isize,
	x: isize,
	y: isize,
	z: isize,
	/// previously encountered error
	/// prevents further execution
	error: Option<ALUError>
}

impl ALU {

	#[inline(always)]
	fn hit_error(&mut self, error: ALUError) -> ALUError {
		self.error = Some(error);
		error
	}

	#[inline(always)]
	pub fn reg(&self, register: var) -> &isize {
		match register {
			var::w => &self.w,
			var::x => &self.x,
			var::y => &self.y,
			var::z => &self.z,
		}
	}
	#[inline(always)]
	pub fn reg_mut(&mut self, register: var) -> &mut isize {
		match register {
			var::w => &mut self.w,
			var::x => &mut self.x,
			var::y => &mut self.y,
			var::z => &mut self.z,
		}
	}
	#[inline(always)]
	pub fn state(&self) -> (isize, isize, isize, isize) {
		(self.w, self.x, self.y, self.z)
	}
	pub fn new(program: &[Op]) -> Self {
		let mut me = ALU::default();
		program.clone_into(&mut me.program);
		me
	}
	pub fn reset(&mut self) {
		self.ic = 0;
		self.pc = 0;
		self.w = 0;
		self.x = 0;
		self.y = 0;
		self.z = 0;
		self.error = None;
	}
	pub fn set_input(&mut self, input: &[isize]) {
		input.clone_into(&mut self.input);
	}
	/// Step the ALU forward one instruction
	/// Returns Ok(true) while there's still more instructions to run
	/// Returns Ok(false) if the program is complete
	pub fn step(&mut self) -> Result<bool, ALUError> {
		if self.pc >= self.program.len() {
			// we've reached the end of the program
			return Err(ALUError::ProgramEnded);
		}
		if let Some(err) = self.error {
			// previously errored, return it again
			return Err(err);
		}
		let instruction = self.program[self.pc];
		match instruction {
			Op::Inp(reg) => {
				if self.ic >= self.input.len() {
					return Err(self.hit_error(ALUError::InpWhenOutOfInput));
				}
				let val = self.input[self.ic];
				*self.reg_mut(reg) = val;
				self.ic += 1;
			},
			Op::Add(a, b) => {
				*self.reg_mut(a) = self.reg(a)+self.reg(b);
			},
			Op::AddLiteral(a, b) => {
				*self.reg_mut(a) = self.reg(a)+b;
			},
			Op::Mul(a, b) => {
				*self.reg_mut(a) = self.reg(a)*self.reg(b);
			},
			Op::MulLiteral(a, b) => {
				*self.reg_mut(a) = self.reg(a)*b;
			},
			Op::Div(a, b) => {
				let b = *self.reg(b);
				if b == 0 {
					return Err(self.hit_error(ALUError::DivByZero));
				}
				*self.reg_mut(a) = self.reg(a)/b;
			},
			Op::DivLiteral(a, b) => {
				if b == 0 {
					return Err(self.hit_error(ALUError::DivByZero));
				}
				*self.reg_mut(a) = self.reg(a)/b;
			},
			Op::Mod(a, b) => {
				let aval = *self.reg(a);
				let b = *self.reg(b);
				if aval < 0 {
					return Err(self.hit_error(ALUError::ModOnNegative));
				} else if b <= 0 {
					return Err(self.hit_error(ALUError::ModByNegativeOrZero));
				}
				*self.reg_mut(a) = aval%b;
			},
			Op::ModLiteral(a, b) => {
				let aval = *self.reg(a);
				if aval < 0 {
					return Err(self.hit_error(ALUError::ModOnNegative));
				} else if b <= 0 {
					return Err(self.hit_error(ALUError::ModByNegativeOrZero));
				}
				*self.reg_mut(a) = aval%b;
			},
			Op::Eql(a, b) => {
				if *self.reg(a) == *self.reg(b) {
					*self.reg_mut(a) = 1;
				} else {
					*self.reg_mut(a) = 0;
				}
			},
			Op::EqlLiteral(a, b) => {
				if *self.reg(a) == b {
					*self.reg_mut(a) = 1;
				} else {
					*self.reg_mut(a) = 0;
				}
			},
		}
		self.pc += 1;
		Ok(self.pc < self.program.len())
	}

	/// Run the ALU until the program finishes or hits an error
	pub fn run(&mut self) -> Result<(), ALUError> {
		while self.step()? {}
		Ok(())
	}

	/// Run with input, resetting the machine after
	/// Returns (w,x,y,z) if no errors occur
	/// Input provided before is discarded
	pub fn run_with_input(&mut self, input: &[isize]) -> Result<(isize, isize, isize, isize), ALUError> {
		self.reset();
		self.input.truncate(0);
		input.clone_into(&mut self.input);

		match self.run() {
			Ok(_) => {
				let state = self.state();
				self.reset();
				Ok(state)
			},
			Err(err) => {
				self.reset();
				Err(err)
			}
		}
	}

	pub fn generate_rust(&self) -> String {
		let mut output = String::new();
		output += stringify! {
			#[allow(dead_code)]
			#[allow(unused_mut)]
			#[allow(unused_assignments)]
			#[allow(unused_variables)]
			pub fn alu_generated_program(input: &[isize]) -> (isize, isize, isize, isize)
		};
		output += " {\n";

		// calculate how long the input should be
		let inputs = self.program.iter().filter(|x| (**x).is_inp()).count();
		output += &format!("assert_eq!(input.len(), {}, \"invalid number of input digits\");", inputs);
		output += stringify!{
			let mut w = 0isize;
			let mut x = 0isize;
			let mut y = 0isize;
			let mut z = 0isize;
		};
		output += "\n\n";

		let mut program: Vec<OptimizedOp> = self.program.iter().map(|op| (*op).into()).collect();
		let before = program.len();
		for _ in 1..15 { program.optimize(); }
		let after = program.len();
		println!("{}->{}", before, after);

		let mut i = 0usize;
		for (_pos, op) in program.iter().enumerate() {
			if let OptimizedOp::Inp(_) = op {
				output += "\n";
			}
			output += &op.to_rust_src(&mut i);
			output += "\n";
		}

		output += "\n";
		output += stringify! {
			(w,x,y,z)
		};
		output += "\n}\n";
		output
	}

}