use nom::IResult;
use nom::bytes::complete::{take_while1, take_until};
use nom::character::complete::char;
use nom::character::is_digit;
use nom::sequence::tuple;

use std::str;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
	Forward(isize),
	Down(isize),
	Up(isize)
}

use Action::*;

fn action_line(input: &[u8]) -> IResult<&[u8], Action> {
	let (input, (action, _, amount)) = tuple((take_until(" "), char(' '), take_while1(is_digit)))(input)?;
	let amount: isize = str::from_utf8(&amount).unwrap_or("0").parse().unwrap();
	match str::from_utf8(action).unwrap_or("") {
		"forward" => Ok((input, Action::Forward(amount))),
		"down" => Ok((input, Action::Down(amount))),
		"up" => Ok((input, Action::Up(amount))),
		_ => panic!()
	}
}

#[aoc_generator(day2)]
pub fn day2_gen(input: &str) -> Vec<Action> {
        input.split('\n').map(|x| action_line(x.as_bytes()).unwrap().1).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Action]) -> isize {
	let mut h = 0;
	let mut d = 0;
	for action in input {
		match action {
			Forward(x) => h += x,
			Down(x) => d += x,
			Up(x) => d -= x
		}
	}
	h*d
}

#[aoc(day2, part2)]
pub fn part2(input: &[Action]) -> isize {
	let mut h = 0;
	let mut d = 0;
	let mut aim = 0;
	for action in input {
		match action {
			Forward(x) => { h += x; d += aim*x; },
			Down(x) => aim += x,
			Up(x) => aim -= x
		}
	}
	h*d
}

#[cfg(test)]
mod tests {
	use super::{day2_gen, part1, part2};
	
	#[test]
	fn sample1() {
		assert_eq!(part1(&day2_gen("forward 5
down 5
forward 8
up 3
down 8
forward 2")), 150);
	}

	#[test]
	fn sample2() {
		assert_eq!(part2(&day2_gen("forward 5
down 5
forward 8
up 3
down 8
forward 2")), 900);
	}

}
