use super::primitives::*;

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

pub trait Optimize {
	fn optimize(&mut self);
}

impl Optimize for Vec<OptimizedOp> {
    fn optimize(&mut self) {
		use OptimizedOp::*;
        let mut ops = self.iter_mut().peekable();
		while let Some(op) = ops.next() {
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
