#![allow(dead_code)]
// #![allow(unused_mut)]
// #![allow(unused_assignments)]

mod primitives;
mod optimizedop;
mod alu;
mod monad;
mod alu_generated;
mod alu_generated_unoptimized;
mod rustgen;
mod modified_gen;

use itertools::Itertools;
use primitives::*;
use rayon::prelude::*;

use monad::*;

#[aoc_generator(day24)]
pub fn gen(_input: &str) -> Vec<Op> {
	vec![]
	//primitives::parse_alu_program(input)
}

pub fn step_any(digits: Vec<isize>) -> Vec<Vec<isize>> {
	// let mut monad = MONAD::new();
	// for digit in &digits {
	// 	monad.step(*digit);
	// }
	(1..=9).map(|l| {
		let mut digits = digits.clone();
		digits.push(l);
		digits
	}).collect_vec()
}
pub fn step_lookahead(digits: Vec<isize>) -> Vec<Vec<isize>> {
	let mut monad = MONAD::new();
	for digit in &digits {
		monad.step(*digit);
	}
	monad.lookahead_digits().iter().map(|l| {
		let mut digits = digits.clone();
		digits.push(*l);
		digits
	}).collect_vec()
}

#[aoc(day24, part1)]
pub fn part1(_input: &[Op]) -> isize {
	{1111111isize..9999999}
	.into_par_iter()
	.map(|serial| {
		num_digits(serial)
	})
	.filter(|digits| !digits.contains(&0))
	.map(step_lookahead).flatten()
	.map(step_any).flatten()
	.map(step_lookahead).flatten()
	.map(step_lookahead).flatten()
	.map(step_lookahead).flatten()
	.map(step_lookahead).flatten()
	.map(step_lookahead).flatten()
	// check it against the original program
	.filter(|x| alu_generated::alu_generated_program(x).3 == 0)
	// convert it back into a normal int
	.map(|digits| digits.iter().fold(0, |acc, x| x+(acc*10)))
	// find the biggest
	.max().unwrap()
}

#[aoc(day24, part2)]
pub fn part2(_input: &[Op]) -> isize {
	{1111111isize..9999999}
	.into_par_iter()
	.map(|serial| {
		num_digits(serial)
	})
	.filter(|digits| !digits.contains(&0))
	.map(step_lookahead).flatten()
	.map(step_any).flatten()
	.map(step_lookahead).flatten()
	.map(step_lookahead).flatten()
	.map(step_lookahead).flatten()
	.map(step_lookahead).flatten()
	.map(step_lookahead).flatten()
	// check it against the original program
	.filter(|x| alu_generated::alu_generated_program(x).3 == 0)
	// convert it back into a normal int
	.map(|digits| digits.iter().fold(0, |acc, x| x+(acc*10)))
	// find the smallest
	.min().unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::alu::*;
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
	fn sample1() {
		assert_eq!(part1(&gen("")), 0);
	}
	
	#[test]
	fn sample2() {
		assert_eq!(part2(&gen("")), 0);
	}
}
