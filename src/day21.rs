use std::collections::BTreeMap;

#[aoc_generator(day21)]
pub fn gen(input: &str) -> (usize, usize) {
        let starts: Vec<usize> = input.split('\n').map(|x| {
			x.chars().last().and_then(|x| x.to_digit(10).map(|x| x as usize)).unwrap()
		}).collect();
		(starts[0], starts[1])

}

#[aoc(day21, part1)]
pub fn part1(input: &(usize, usize)) -> usize {
	let mut die = (1..=100).cycle();
	let mut rolls = 0;
	let mut roll = || { rolls += 1; die.next().unwrap() };
	let mut p1pos = input.0;
	let mut p2pos = input.1;
	let mut p1score = 0;
	let mut p2score = 0;
	loop {
		p1pos += roll() + roll() + roll();
		p1pos %= 10;
		if p1pos == 0 { p1pos = 10 };
		p1score += p1pos;
		if p1score >= 1000 { break; }

		p2pos += roll() + roll() + roll();
		p2pos %= 10;
		if p2pos == 0 { p2pos = 10 };
		p2score += p2pos;
		if p2score >= 1000 { break; }
	}
	rolls*std::cmp::min(p1score, p2score)
}


pub fn roll_the_die(cache: &mut BTreeMap<(usize, usize, usize, usize), (usize, usize)>, p1score: usize, p2score: usize, 
	p1pos: usize, p2pos: usize) -> (usize, usize) {
if let Some(value) = cache.get(&(p1score, p2score, p1pos, p2pos)) {
	return *value;
}

if p1score >= 21 {
	return (1, 0);
} else if p2score >= 21 {
	return (0, 1);
}
let mut p1wins = 0;
let mut p2wins = 0;
for i in 1..=3 {
	for j in 1..=3 {
		for k in 1..=3 {
			let mut p1pos = (p1pos+i+j+k) % 10;
			if p1pos == 0 { p1pos = 10 };
			let mut p2pos = (p2pos+i+j+k) % 10;
			if p2pos == 0 { p2pos = 10 };
			let results = roll_the_die(cache, p1score+p1pos, p2score+p2pos, p1pos, p2pos);
			p1wins += results.0;
			p2wins += results.1;
		}
	}
	println!("{} {}", p1wins, p2wins);
}
cache.insert((p1score, p2score, p1pos, p2pos), (p1wins, p2wins));
(p1wins, p2wins)
}

#[aoc(day21, part2)]
pub fn part2(input: &(usize, usize)) -> usize {

	let mut cache = BTreeMap::new();

	let results = roll_the_die(&mut cache, 0, 0, input.0, input.1);
	std::cmp::max(results.0, results.1)
}

#[cfg(test)]
mod tests {
	use super::{gen, part1, part2};
	
	#[test]
	fn sample1() {
		assert_eq!(part1(&gen("Player 1 starting position: 4
		Player 2 starting position: 8")), 739785);
	}
	
	#[test]
	fn sample2() {
		assert_eq!(part2(&gen("4\n8")), 444356092776315);
	}
}
