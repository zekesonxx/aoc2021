
#[aoc_generator(day1)]
pub fn day1_gen(input: &str) -> Vec<usize> {
        input.split('\n').map(|x| x.parse().unwrap_or(0)).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[usize]) -> u32 {
	let mut last = input[0];
	let mut increases = 0;
	for i in input {
		if *i > last {
			increases += 1;
		}
		last = *i;
	}
	increases
}

#[aoc(day1, part2, orig)]
pub fn part2_orig(input: &[usize]) -> u32 {
	let mut windows = vec![];
	for i in 0..input.len()-2 {
		windows.push(input[i] + input[i+1] + input[i+2]);
	}
	part1(&windows)
}

#[aoc(day1, part2, oneliner)]
pub fn part2_oneliner(input: &[usize]) -> u32 {
	part1(&input.windows(3).map(|x| x.iter().sum()).collect::<Vec<usize>>())
}

#[cfg(test)]
mod tests {
	use super::{day1_gen, part1, part2_orig as part2};
	
	#[test]
	fn sample1() {
		assert_eq!(part1(&day1_gen("199
200
208
210
200
207
240
269
260
263")), 7);
	}
	
	#[test]
	fn sample2() {
		assert_eq!(part2(&day1_gen("199
200
208
210
200
207
240
269
260
263")), 5);
	}
}
