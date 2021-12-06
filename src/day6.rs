
#[aoc_generator(day6)]
pub fn gen(input: &str) -> Vec<usize> {
        input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[usize]) -> usize {
	let mut fish = [0usize; 9];
	for f in input {
		fish[*f] += 1;
	}
	for day in 0..80 {
		// 0 1 2 3 4 5 6 7 8
		// 1 2 3 4 5 6 7 8 0
		fish.rotate_left(1);
		fish[6] += fish[8];
	}
	fish.iter().sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &[usize]) -> usize {
	let mut fish = [0usize; 9];
	for f in input {
		fish[*f] += 1;
	}
	for day in 0..256 {
		// 0 1 2 3 4 5 6 7 8
		// 1 2 3 4 5 6 7 8 0
		fish.rotate_left(1);
		fish[6] += fish[8];
	}
	fish.iter().sum()
	
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
