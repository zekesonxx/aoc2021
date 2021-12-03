
#[aoc_generator(day3)]
pub fn day3_gen(input: &str) -> Vec<String> {
	input.split('\n').map(|x| x.to_string()).collect()
}

pub fn popularity(input: &[String]) -> Vec<usize> {
	let width = input[0].len();
	let mut popularity = vec![];
	for _ in 0..width {
		popularity.push(0usize);
	}
	for line in input {
		let mut chars = line.char_indices();
		while let Some((i, x)) = chars.next() {
			if x == '1' {
				popularity[i] += 1;
			}
		}
	}
	popularity
}

#[aoc(day3, part1)]
pub fn part1(input: &[String]) -> usize {
	let lines = input.len();
	let popularity = popularity(&input);
	let mut gamma = 0;
	let mut epsilon = 0;
	for digit in popularity {
		gamma <<= 1;
		epsilon <<= 1;
		if digit > lines/2 {
			// most common is 1
			gamma += 1;
		} else {
			epsilon += 1;
		}
	}
	gamma*epsilon
}

#[aoc(day3, part2)]
pub fn part2(input: &[String]) -> usize {
	let mut digit = 0;
	
	let mut pop;
	let mut oxygen = input.to_vec();
	while oxygen.len() > 1 {
		pop = popularity(&oxygen);
		if pop[digit] >= oxygen.len()-pop[digit] {
			oxygen.drain_filter(|x| x.chars().nth(digit).unwrap() == '0');
		} else {
			oxygen.drain_filter(|x| x.chars().nth(digit).unwrap() == '1');
		}
		digit += 1;
	}
	let oxygen = usize::from_str_radix(&oxygen[0], 2).unwrap();
	
	digit = 0;
	
	let mut co2 = input.to_vec();
	while co2.len() > 1 {
		pop = popularity(&co2);
		if pop[digit] < co2.len()-pop[digit] {
			co2.drain_filter(|x| x.chars().nth(digit).unwrap() == '0');
		} else {
			co2.drain_filter(|x| x.chars().nth(digit).unwrap() == '1');
		}
		digit += 1;
	}
	let co2 = usize::from_str_radix(&co2[0], 2).unwrap();

	oxygen*co2
}

#[cfg(test)]
mod tests {
	use super::{day3_gen, part1, part2};
	
	#[test]
	fn sample1() {
		assert_eq!(part1(&day3_gen("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010")), 198);
	}
	
	
	#[test]
	fn sample2() {
		assert_eq!(part2(&day3_gen("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010")), 230);
	}
	
}
