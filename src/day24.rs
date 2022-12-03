// #![allow(dead_code)]
// #![allow(unused_mut)]
// #![allow(unused_assignments)]

mod primitives;
mod optimizedop;
mod alu;
mod monad;
mod alu_generated;
mod rustgen;
mod modified_gen;

use primitives::*;

//use crate::day24::rustgen::generate_rust;
//use monad::*;

#[aoc_generator(day24)]
pub fn gen(input: &str) -> Vec<Op> {
	primitives::parse_alu_program(input)
}

#[aoc(day24, part1)]
pub fn part1(input: &[Op]) -> usize {
	//use rayon::prelude::*;
	//let alu = ALU::new(input);
	//println!("{}", generate_rust(input));
	//let mut monad = MONAD::new(alu);
	
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
	for i in [
		13579246899998,
		58284717283482,
		23488239482631,
		75351354645944,
		48753121843556,
		18753121843556,
		28753121843556,
		38753121843556,
		58753121843556,
		68753121843556,
		78753121843556,
		88753121843556,
		98753121843556,
		11111111111111,
		11119119199999
		] {
		let gen = alu_generated::alu_generated_program(&num_digits(i)).3;
		let hand = modified_gen::alu_generated_program(&num_digits(i)).3;
		println!("{:?} {:?}", gen, hand);
		assert_eq!(gen, hand);
	}
	0
}

#[aoc(day24, part2)]
pub fn part2(_input: &[Op]) -> usize {
	0
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

	// #[test]
	// #[ignore]
	// fn optimized_verification() {
	// 	use rayon::prelude::*;
	// 	{11111111111111isize..99999999999999}.into_par_iter().rev()
	// 	.panic_fuse()
	// 	//.inspect(|x| println!("checking {}", x))
	// 	.for_each(|serial| {
	// 		let digits = num_digits(serial);
	// 		assert_eq!(original_alu_generated_program(&digits), alu_generated_program(&digits));
	// 	})
	// }
	
	#[test]
	fn sample1() {
		assert_eq!(part1(&gen("")), 0);
	}
	
	#[test]
	fn sample2() {
		assert_eq!(part2(&gen("")), 0);
	}
}
