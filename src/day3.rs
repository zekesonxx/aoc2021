
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
	println!("popularity: {:?}, lines: {}", popularity, lines);
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
	let width = input[0].len();
	
	let mut oxypop;
	let mut oxygen = input.to_vec();
	let mut digit = 0;
	while oxygen.len() > 1 {
		oxypop = popularity(&oxygen);
		println!("{} ? {}", oxypop[digit], oxygen.len()/2);
		if oxypop[digit] >= oxygen.len()-oxypop[digit] {
			oxygen.drain_filter(|x| x.chars().nth(digit).unwrap() == '0');
		} else {
			oxygen.drain_filter(|x| x.chars().nth(digit).unwrap() == '1');
		}
		println!("{:?}", oxygen);
		digit += 1;
	}
	println!("oxygen: {}", oxygen[0]);
	let oxygen = usize::from_str_radix(&oxygen[0], 2).unwrap();
	println!("oxygen: {}", oxygen);
	digit = 0;
	
	let mut co2pop;
	let mut co2 = input.to_vec();
	while co2.len() > 1 {
		co2pop = popularity(&co2);
		println!("{} ? {}", co2pop[digit], co2.len()/2);
		if co2pop[digit] < co2.len()-co2pop[digit] {
			co2.drain_filter(|x| x.chars().nth(digit).unwrap() == '0');
		} else {
			co2.drain_filter(|x| x.chars().nth(digit).unwrap() == '1');
		}
		println!("{:?}", co2);
		digit += 1;
	}
	println!("co2: {}", co2[0]);
	let co2 = usize::from_str_radix(&co2[0], 2).unwrap();
	println!("co2: {}", co2);

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
