
#[aoc_generator(day6)]
pub fn gen(input: &str) -> Vec<usize> {
        input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[usize]) -> usize {
	let mut fish = input.to_vec();
	let mut newfish = 0;
	for day in 0..80 {
		for f in &mut fish {
			match f {
				1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => *f -= 1,
				0 => { *f = 6; newfish += 1; },
				_ => {}
			}
		}
		while newfish > 0 {
			newfish -= 1;
			fish.push(8);
		}
	}
	fish.len()
}

#[aoc(day6, part2)]
pub fn part2(input: &[usize]) -> usize {
	let mut fish = input.to_vec();
	let mut newfish = 0;
	for day in 0..256 {
		for f in &mut fish {
			match f {
				1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => *f -= 1,
				0 => { *f = 6; newfish += 1; },
				_ => {}
			}
		}
		while newfish > 0 {
			newfish -= 1;
			fish.push(8);
		}
		println!("{}", day);
	}
	fish.len()
}

#[cfg(test)]
mod tests {
	use super::{gen, part1, part2};
	
	#[test]
	fn sample1() {
		assert_eq!(part1(&gen("3,4,3,1,2")), 5934);
	}
	
	#[test]
	fn sample2() {
		assert_eq!(part2(&gen("3,4,3,1,2")), 26984457539);
	}
}
