use std::fmt::Write;

use itertools::Itertools;

//https://codereview.stackexchange.com/questions/226233/number-to-vector-of-its-digits
fn num_digits(num: isize) -> Vec<isize> {
    /*
     * Zero is a special case because
     * it is the terminating value of the unfold below,
     * but given a 0 as input, [0] is expected as output.
     * w/out this check, the output is an empty vec.
     */
    if num == 0 {
        return vec![0];
    }

    let mut x = num;
    let mut result = std::iter::from_fn(move || {
        if x == 0 {
            None
        } else {
            let current = x % 10;
            x /= 10;
            Some(current)
        }
    })
    .collect::<Vec<isize>>();

    result.reverse();
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum var {
	w,
	x,
	y,
	z,
}

impl std::fmt::Display for var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_char())
    }
}

impl var {
    pub fn from_char(value: char) -> Option<Self> {
        match value {
			'w' => Some(var::w),
			'x' => Some(var::x),
			'y' => Some(var::y),
			'z' => Some(var::z),
			_ => None
		}
    }
	pub fn to_char(&self) -> char {
		match *self {
			var::w => 'w',
			var::x => 'x',
			var::y => 'y',
			var::z => 'z',
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
	Inp(var),
	Add(var, var),
	AddLiteral(var, isize),
	Mul(var, var),
	MulLiteral(var, isize),
	Div(var, var),
	DivLiteral(var, isize),
	Mod(var, var),
	ModLiteral(var, isize),
	Eql(var, var),
	EqlLiteral(var, isize),
}

impl Op {
	pub fn is_inp(&self) -> bool {
		match *self {
			Op::Inp(_) => true,
			_ => false
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizedOp {
	Inp(var),
	Add(var, var),
	AddLiteral(var, isize),
	Mul(var, var),
	MulLiteral(var, isize),
	Div(var, var),
	DivLiteral(var, isize),
	Mod(var, var),
	ModLiteral(var, isize),
	Eql(var, var),
	EqlLiteral(var, isize),


	// #### Optimized Instructions ####

	//eql x w
	//eql x 0
	// -> x=if x==w {0} else {1}
	/// left, right
	InvertedEql(var, var),

	// eql x w
	// eql x 0
	// mul y 0
	// add y x
	// mul y 25
	// -> if (x==w) {y=0} else {y=25}
	/// left, right, dest, value
	//ConditionalSet(var, var, var, isize),

	//mul x 0
	// -> x=0
	Zero(var),
	// mul x 0
	// add x y
	// -> x=y
	/// dest, src
	Copy(var, var),
	// mul x 0
	// add x 26
	// -> x=26
	SetLiteral(var, isize),
	// mul y 0
	// add y 25
	// mul y x
	// -> y=25*x
	/// dest, src, value
	MulStore(var, var, isize),
	// mul y 0
	// add y 25
	// add y x
	// -> y=25+x
	/// dest, src, value
	AddStore(var, var, isize),
	// mul y 0
	// add y 25
	// div y x
	// -> y=25/x
	/// dest, src, value
	/// dest=value/src
	DivStore(var, var, isize),
	// mul y 0
	// add y 25
	// mod y x
	// -> y=25%x
	/// dest, src, value
	/// dest=value%src
	ModStore(var, var, isize),

	/// dest, src, value
	/// dest=src/value
	DivByStore(var, var, isize),

	/// dest, src, value
	/// dest=src%value
	ModByStore(var, var, isize),
	Noop
}

impl From<Op> for OptimizedOp {
    fn from(value: Op) -> Self {
        match value {
			Op::Inp(a) => OptimizedOp::Inp(a),
			Op::Add(a, b) => OptimizedOp::Add(a, b),
			Op::AddLiteral(a, b) => OptimizedOp::AddLiteral(a, b),
			Op::Mul(a, b) => OptimizedOp::Mul(a, b),
			Op::MulLiteral(a, b) => OptimizedOp::MulLiteral(a, b),
			Op::Div(a, b) => OptimizedOp::Div(a, b),
			Op::DivLiteral(a, b) => OptimizedOp::DivLiteral(a, b),
			Op::Mod(a, b) => OptimizedOp::Mod(a, b),
			Op::ModLiteral(a, b) => OptimizedOp::ModLiteral(a, b),
			Op::Eql(a, b) => OptimizedOp::Eql(a, b),
			Op::EqlLiteral(a, b) => OptimizedOp::EqlLiteral(a, b),
		}
    }
}

impl OptimizedOp {
	fn to_rust_src_inner(&self, input_counter: &mut usize) -> String {
		use OptimizedOp::*;
		match self {
			Inp(a) => format!("{}=input[{}];", a, {*input_counter+=1;*input_counter-1}),
			Add(a, b) => format!("{}+={};", a, b),
			AddLiteral(a, b) => format!("{}+={};", a, b),
			Mul(a, b) => format!("{}*={};", a, b),
			MulLiteral(a, b) => format!("{}*={};", a, b),
			Div(a, b) => format!("{}/={};", a, b),
			DivLiteral(a, b) => format!("{}/={};", a, b),
			Mod(a, b) => format!("{}%={};", a, b),
			ModLiteral(a, b) => format!("{}%={};", a, b),
			Eql(a, b) => format!("{a}=if {a}=={b} {{1}}else{{0}};", a = a, b = b),
			EqlLiteral(a, b) => format!("{a}=if {a}=={b} {{1}}else{{0}};", a = a, b = b),
			Zero(a) => format!("{a}=0;", a=a),
			Copy(dest, src) => format!("{}={};", dest, src),
			InvertedEql(a, b) => format!("{a}=if {a}=={b} {{0}}else{{1}};", a = a, b = b),
			SetLiteral(dest, value) => format!("{}={};", dest, value),
			MulStore(dest, src, value) => format!("{}={}*{};", dest, src, value),
			AddStore(dest, src, value) => format!("{}={}+{};", dest, src, value),
			DivStore(dest, src, value) => format!("{}={}/{};", dest, value, src),
			ModStore(dest, src, value) => format!("{}={}%{};", dest, value, src),
			DivByStore(dest, src, value) => format!("{}={}/{};", dest, src, value),
			ModByStore(dest, src, value) => format!("{}={}%{};", dest, src, value),
			Noop => String::new(),
		}
	}

	pub fn to_rust_src(&self, input_counter: &mut usize) -> String {
		let mut inner = self.to_rust_src_inner(input_counter);
		if let Some(v) = self.writes().get(0) {
			if self.will_overwrite(*v) {
				inner.insert_str(0, "let mut ");
			}
		}
		inner
	}

	/// Returns, in order:
	/// 0: list of variable(s) that are read by this opcode
	/// 1: list of variable(s) that are written to by this opcode
	/// 
	/// if variable is in 1 and 2, then it is written to with reference to its own value (x=x+2)
	/// if variable is only in 1, then it is read but not written (x in y=x+2)
	/// if variable is only in 2, then it is written to with no reference to its own value (x in x=2, x in x=y+2)
	//                                read  , written
	fn combined_metadata(&self) -> (Vec<var>, Vec<var>) {
		use OptimizedOp::*;
		match self {
			Inp(a) => (vec![], vec![*a]),
			Add(a, b) | Mul(a, b) | Div(a, b) | Mod(a, b) => (vec![*a], vec![*a, *b]),
			AddLiteral(a, _) | MulLiteral(a, _) | DivLiteral(a, _) | ModLiteral(a, _) => (vec![*a], vec![*a]),
			Eql(a, b) => (vec![*a], vec![*a, *b]),
			EqlLiteral(a, _) => (vec![*a], vec![*a]),
			Zero(a) => (vec![*a], vec![*a]),
			Copy(dest, src) => (vec![*src], vec![*dest]),
			InvertedEql(a, b) => (vec![*a, *b], vec![*a]),
			SetLiteral(dest, _) => (vec![*dest], vec![]),
			MulStore(dest, src, _) |
			AddStore(dest, src, _) |
			DivStore(dest, src, _) |
			ModStore(dest, src, _) |
			DivByStore(dest, src, _) |
			ModByStore(dest, src, _)  => (vec![*src], vec![*dest]),
			Noop => (vec![], vec![]),
		}
	}

	/// Get a list of variables that this opcode reads from
	pub fn reads(&self) -> Vec<var> {
		self.combined_metadata().0
	}

	/// Get a list of variables that this opcode overwrites
	pub fn writes(&self) -> Vec<var> {
		self.combined_metadata().1
	}

	// Return true if the opcode will overwrite the provided variable
	pub fn will_write(&self, v: var) -> bool {
		self.writes().contains(&v)
	}

	// Return true if the opcode will read from the provided variable
	pub fn will_read(&self, v: var) -> bool {
		self.reads().contains(&v)
	}

	// Return true if the opcode will write to the variable without
	// consideration of the variable's previous value
	pub fn will_overwrite(&self, v: var) -> bool {
		!self.will_read(v) && self.will_write(v)
	}

}

trait Optimize {
	fn optimize(&mut self);
}

impl Optimize for Vec<OptimizedOp> {
    fn optimize(&mut self) {
		use OptimizedOp::*;
        let mut ops = self.iter_mut().peekable();
		while let Some(mut op) = ops.next() {
			// single instruction optimizations
			match op {
				MulLiteral(a, 0) => {
					*op = Zero(*a);
				},
				DivLiteral(_, 1) | AddLiteral(_, 0) => {
					*op = Noop;
				},
				_ => {}
			}
			// double instruction optimizations
			if let Some(next_op) = ops.peek_mut() {
				match op {
					Eql(a, b) => {
						if let OptimizedOp::EqlLiteral(next_a, 0) = next_op {
							if *next_a == *a {
								*op = InvertedEql(*a, *b);
								**next_op = Noop;
							}
						}
					},
					Zero(a) => {
						if let OptimizedOp::Add(next_a, next_b) = next_op {
							if *next_a == *a {
								*op = Copy(*a, *next_b);
								**next_op = Noop;
							}
						} else if let OptimizedOp::AddLiteral(next_a, next_b) = next_op {
							if *next_a == *a {
								*op = SetLiteral(*a, *next_b);
								**next_op = Noop;
							}
						}
					},
					// this causes side effects
					// we would need to check ahead if anything reads x before it next gets overwritten
					// InvertedEql(left, right) => {
					// 	if let OptimizedOp::MulStore(dest, src, value) = next_op {
					// 		if *left == *src {
					// 			*op = ConditionalSet(*left, *right, *dest, *value);
					// 			**next_op = Noop;
					// 		}
					// 	}
					// },
					SetLiteral(dest, value) => {
						if let OptimizedOp::Add(next_a, next_b) = next_op {
							if *next_a == *dest {
								*op = AddStore(*dest, *next_b, *value);
								**next_op = Noop;
							}
						} else if let OptimizedOp::Mul(next_a, next_b) = next_op {
							if *next_a == *dest {
								*op = MulStore(*dest, *next_b, *value);
								**next_op = Noop;
							}
						} else if let OptimizedOp::Mod(next_a, next_b) = next_op {
							if *next_a == *dest {
								*op = ModStore(*dest, *next_b, *value);
								**next_op = Noop;
							}
						} else if let OptimizedOp::Div(next_a, next_b) = next_op {
							if *next_a == *dest {
								*op = DivStore(*dest, *next_b, *value);
								**next_op = Noop;
							}
						}
					},
					Copy(dest, src) => {
						if let OptimizedOp::AddLiteral(next_a, value) = next_op {
							if *next_a == *dest {
								*op = AddStore(*dest, *src, *value);
								**next_op = Noop;
							}
						} else if let OptimizedOp::MulLiteral(next_a, value) = next_op {
							if *next_a == *dest {
								*op = MulStore(*dest, *src, *value);
								**next_op = Noop;
							}
						} else if let OptimizedOp::ModLiteral(next_a, value) = next_op {
							if *next_a == *dest {
								*op = ModByStore(*dest, *src, *value);
								**next_op = Noop;
							}
						} else if let OptimizedOp::DivLiteral(next_a, value) = next_op {
							if *next_a == *dest {
								*op = DivByStore(*dest, *src, *value);
								**next_op = Noop;
							}
						}
					}

					_ => {}
				}
			}
		}
		self.retain(|x| *x != Noop);
	}
}


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
		for (position, op) in program.iter().enumerate() {
			if let OptimizedOp::Inp(_) = op {
				output += "\n";
			}
			output += &op.to_rust_src(&mut i);
			output += "\n";
			let mut k = 5;
			k %= 2;
		}

		output += "\n";
		output += stringify! {
			(w,x,y,z)
		};
		output += "\n}\n";
		output
	}

}


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


#[aoc_generator(day24)]
pub fn gen(input: &str) -> Vec<Op> {
        input.split('\n')
		.map(|x| {
			let mut chars = x.chars();
			// the first character of the opcode is almost unique
			let o = chars.next().unwrap();
			let p = chars.next().unwrap();
			chars.next();
			chars.next();
			let a = chars.next().unwrap();
			let a = var::from_char(a).expect("invalid opcode");
			if o == 'i' {
				// inp a instruction, no second operand
				Op::Inp(a)
			} else {
				// every other instruction, two operands
				chars.next().unwrap();
				let b = chars.collect::<String>();
				if let Some(b) = var::from_char(b.chars().next().unwrap()) {
					// Second operand is a variable
					match (o, p) {
						('a', 'd') => Op::Add(a, b),
						('m', 'u') => Op::Mul(a, b),
						('d', 'i') => Op::Div(a, b),
						('m', 'o') => Op::Mod(a, b),
						('e', 'q') => Op::Eql(a, b),
						_ => panic!("invalid opcode")
					}
				} else {
					// Second operand is a literal
					let b: isize = b.parse().unwrap();
					match (o, p) {
						('a', 'd') => Op::AddLiteral(a, b),
						('m', 'u') => Op::MulLiteral(a, b),
						('d', 'i') => Op::DivLiteral(a, b),
						('m', 'o') => Op::ModLiteral(a, b),
						('e', 'q') => Op::EqlLiteral(a, b),
						_ => panic!("invalid opcode")
					}
				}
			}
		})
		.collect()
}

#[aoc(day24, part1)]
pub fn part1(input: &[Op]) -> usize {
	use rayon::prelude::*;
	let mut alu = ALU::new(input);
	println!("{}", alu.generate_rust());
	let mut monad = MONAD::new(alu);
	
	// {11111111111111isize..99999999999999}.into_par_iter()//.rev()
	// //.inspect(|x| println!("checking {}", x))
	// .map_with(monad, |monad, serial| {
	// 		if alu_generated_program(&num_digits(serial)).3 == 0 {
	// 			Some(serial)
	// 		} else {
	// 			None
	// 		}
	// }).flatten()
	// .inspect(|x| println!("found valid serial {}", x))
	// .max();
	println!("{:?}", alu_generated_program(&num_digits(13579246899998)));
	0
}

#[aoc(day24, part2)]
pub fn part2(input: &[Op]) -> usize {
	0
}

#[allow(unused_assignments)]
pub fn original_alu_generated_program(input : & [isize]) ->
(isize, isize, isize, isize) {
assert_eq!(input.len(), 14, "invalid number of input digits");let mut w = 0isize ; let mut x = 0isize ; let mut y = 0isize ; let mut z =
0isize ;

w=input[0];
x*=0;
x+=z;
x%=26;
z/=1;
x+=13;
x=if x==w {1}else{0};
x=if x==0 {1}else{0};
y*=0;
y+=25;
y*=x;
y+=1;
z*=y;
y*=0;
y+=w;
y+=6;
y*=x;
z+=y;
w=input[1];
x*=0;
x+=z;
x%=26;
z/=1;
x+=15;
x=if x==w {1}else{0};
x=if x==0 {1}else{0};
y*=0;
y+=25;
y*=x;
y+=1;
z*=y;
y*=0;
y+=w;
y+=7;
y*=x;
z+=y;
w=input[2];
x*=0;
x+=z;
x%=26;
z/=1;
x+=15;
x=if x==w {1}else{0};
x=if x==0 {1}else{0};
y*=0;
y+=25;
y*=x;
y+=1;
z*=y;
y*=0;
y+=w;
y+=10;
y*=x;
z+=y;
w=input[3];
x*=0;
x+=z;
x%=26;
z/=1;
x+=11;
x=if x==w {1}else{0};
x=if x==0 {1}else{0};
y*=0;
y+=25;
y*=x;
y+=1;
z*=y;
y*=0;
y+=w;
y+=2;
y*=x;
z+=y;
w=input[4];
x*=0;
x+=z;
x%=26;
z/=26;
x+=-7;
x=if x==w {1}else{0};
x=if x==0 {1}else{0};
y*=0;
y+=25;
y*=x;
y+=1;
z*=y;
y*=0;
y+=w;
y+=15;
y*=x;
z+=y;
w=input[5];
x*=0;
x+=z;
x%=26;
z/=1;
x+=10;
x=if x==w {1}else{0};
x=if x==0 {1}else{0};
y*=0;
y+=25;
y*=x;
y+=1;
z*=y;
y*=0;
y+=w;
y+=8;
y*=x;
z+=y;
w=input[6];
x*=0;
x+=z;
x%=26;
z/=1;
x+=10;
x=if x==w {1}else{0};
x=if x==0 {1}else{0};
y*=0;
y+=25;
y*=x;
y+=1;
z*=y;
y*=0;
y+=w;
y+=1;
y*=x;
z+=y;
w=input[7];
x*=0;
x+=z;
x%=26;
z/=26;
x+=-5;
x=if x==w {1}else{0};
x=if x==0 {1}else{0};
y*=0;
y+=25;
y*=x;
y+=1;
z*=y;
y*=0;
y+=w;
y+=10;
y*=x;
z+=y;
w=input[8];
x*=0;
x+=z;
x%=26;
z/=1;
x+=15;
x=if x==w {1}else{0};
x=if x==0 {1}else{0};
y*=0;
y+=25;
y*=x;
y+=1;
z*=y;
y*=0;
y+=w;
y+=5;
y*=x;
z+=y;
w=input[9];
x*=0;
x+=z;
x%=26;
z/=26;
x+=-3;
x=if x==w {1}else{0};
x=if x==0 {1}else{0};
y*=0;
y+=25;
y*=x;
y+=1;
z*=y;
y*=0;
y+=w;
y+=3;
y*=x;
z+=y;
w=input[10];
x*=0;
x+=z;
x%=26;
z/=26;
x+=0;
x=if x==w {1}else{0};
x=if x==0 {1}else{0};
y*=0;
y+=25;
y*=x;
y+=1;
z*=y;
y*=0;
y+=w;
y+=5;
y*=x;
z+=y;
w=input[11];
x*=0;
x+=z;
x%=26;
z/=26;
x+=-5;
x=if x==w {1}else{0};
x=if x==0 {1}else{0};
y*=0;
y+=25;
y*=x;
y+=1;
z*=y;
y*=0;
y+=w;
y+=11;
y*=x;
z+=y;
w=input[12];
x*=0;
x+=z;
x%=26;
z/=26;
x+=-9;
x=if x==w {1}else{0};
x=if x==0 {1}else{0};
y*=0;
y+=25;
y*=x;
y+=1;
z*=y;
y*=0;
y+=w;
y+=12;
y*=x;
z+=y;
w=input[13];
x*=0;
x+=z;
x%=26;
z/=26;
x+=0;
x=if x==w {1}else{0};
x=if x==0 {1}else{0};
y*=0;
y+=25;
y*=x;
y+=1;
z*=y;
y*=0;
y+=w;
y+=10;
y*=x;
z+=y;
(w, x, y, z)
}

#[allow(unused_assignments)]
pub fn alu_generated_program(input : & [isize]) ->
(isize, isize, isize, isize) {
assert_eq!(input.len(), 14, "invalid number of input digits");let mut w = 0isize ; let mut x = 0isize ; let mut y = 0isize ; let mut z =
0isize ;

let mut w=input[0];
let mut x=z%26;
x+=13;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+6;
y*=x;
z+=y;
let mut w=input[1];
let mut x=z%26;
x+=15;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+7;
y*=x;
z+=y;
let mut w=input[2];
let mut x=z%26;
x+=15;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+10;
y*=x;
z+=y;
let mut w=input[3];
let mut x=z%26;
x+=11;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+2;
y*=x;
z+=y;
let mut w=input[4];
let mut x=z%26;
z/=26;
x+=-7;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+15;
y*=x;
z+=y;
let mut w=input[5];
let mut x=z%26;
x+=10;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+8;
y*=x;
z+=y;
let mut w=input[6];
let mut x=z%26;
x+=10;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+1;
y*=x;
z+=y;
let mut w=input[7];
let mut x=z%26;
z/=26;
x+=-5;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+10;
y*=x;
z+=y;
let mut w=input[8];
let mut x=z%26;
x+=15;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+5;
y*=x;
z+=y;
let mut w=input[9];
let mut x=z%26;
z/=26;
x+=-3;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+3;
y*=x;
z+=y;
let mut w=input[10];
let mut x=z%26;
z/=26;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+5;
y*=x;
z+=y;
let mut w=input[11];
let mut x=z%26;
z/=26;
x+=-5;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+11;
y*=x;
z+=y;
let mut w=input[12];
let mut x=z%26;
z/=26;
x+=-9;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+12;
y*=x;
z+=y;
let mut w=input[13];
let mut x=z%26;
z/=26;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+10;
y*=x;
z+=y;
(w, x, y, z)
}


#[cfg(test)]
mod tests {
	use super::*;
	const EXAMPLE1: &str = "inp x
mul x -1";
	const EXAMPLE2: &str = "inp z
inp x
mul z 3
eql z x";
	const EXAMPLE3: &str = "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2";
	#[test]
	fn alu_example1() {
		let ops = gen(EXAMPLE1);
		let mut alu = ALU::new(&ops);
		assert_eq!(alu.run_with_input(&[10]), Ok((0,-10,0,0)));
		assert_eq!(alu.run_with_input(&[-70]), Ok((0,70,0,0)));
		assert_eq!(alu.run_with_input(&[-90, 90]), Ok((0,90,0,0)));
	}

	#[test]
	fn alu_example2() {
		let ops = gen(EXAMPLE2);
		let mut alu = ALU::new(&ops);
		assert_eq!(alu.run_with_input(&[3, 9]).map(|x|x.3), Ok(1));
		assert_eq!(alu.run_with_input(&[3, 10]).map(|x|x.3), Ok(0));
		assert_eq!(alu.run_with_input(&[-10, -30]).map(|x|x.3), Ok(1));
	}
	#[test]
	fn alu_example3() {
		let ops = gen(EXAMPLE3);
		let mut alu = ALU::new(&ops);
		assert_eq!(alu.run_with_input(&[1337]), Ok((1,0,0,1)));
		assert_eq!(alu.run_with_input(&[69]), Ok((0,1,0,1)));
		assert_eq!(alu.run_with_input(&[13]), Ok((1,1,0,1)));
	}

	#[test]
	#[ignore]
	fn optimized_verification() {
		use rayon::prelude::*;
		{11111111111111isize..99999999999999}.into_par_iter().rev()
		.panic_fuse()
		//.inspect(|x| println!("checking {}", x))
		.for_each(|serial| {
			let digits = num_digits(serial);
			assert_eq!(original_alu_generated_program(&digits), alu_generated_program(&digits));
		})
	}
	
	#[test]
	fn sample1() {
		assert_eq!(part1(&gen("")), 0);
	}
	
	#[test]
	fn sample2() {
		assert_eq!(part2(&gen("")), 0);
	}
}
