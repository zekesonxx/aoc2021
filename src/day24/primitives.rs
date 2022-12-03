use std::fmt::Write;

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

pub fn parse_alu_program(input: &str) -> Vec<Op> {
	input.split('\n')
	.filter(|x| !x.is_empty())
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


//https://codereview.stackexchange.com/questions/226233/number-to-vector-of-its-digits
pub fn num_digits(num: isize) -> Vec<isize> {
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