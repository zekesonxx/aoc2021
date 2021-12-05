
#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<usize> {
        input.split('\n').map(|x| x.parse().unwrap_or(0)).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[usize]) -> usize {
	0
}

#[aoc(day1, part2)]
pub fn part2(input: &[usize]) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::{gen, part1, part2};
	
	#[test]
	fn sample1() {
		assert_eq!(part1(&gen("")), 7);
	}
	
	#[test]
	fn sample2() {
		assert_eq!(part2(&gen("")), 5);
	}
}
