
use std::cmp;

#[derive(Clone, Copy, Debug)]
pub struct Line {
	pub x1: usize,
	pub y1: usize,
	pub x2: usize,
	pub y2: usize
}

impl Line {
	pub fn is_straight(&self) -> bool {
		self.x1 == self.x2 || self.y1 == self.y2
	}
	pub fn is_gay(&self) -> bool {
		!self.is_straight()
	}
	pub fn is_v(&self) -> bool {
		self.x1 == self.x2
	}
	pub fn is_h(&self) -> bool {
		self.y1 == self.y2
	}
}

#[aoc_generator(day5)]
pub fn gen(input: &str) -> Vec<Line> {
	input.split('\n').map(|line| {
        	let points: Vec<Vec<usize>> = line.split(" -> ").map(|x| {
			x.split(',').map(|y| y.parse().unwrap()).collect()
		}).collect();
		Line {
			x1: points[0][0],
			y1: points[0][1],
			x2: points[1][0],
			y2: points[1][1],

		}
	}).collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[Line]) -> usize {
	let straights: Vec<&Line> = input.iter().filter(|x| x.is_straight()).collect();
	let maxx = straights.iter().map(|x| cmp::max(x.x1, x.x2)).max().unwrap()+1;
	let maxy = straights.iter().map(|x| cmp::max(x.y1, x.y2)).max().unwrap()+1;
	println!("{}x{} grid", maxx, maxy);
	println!("{:?}, {}", straights, input[1].is_straight());
	//  |
	//  y
	//  | 
	//   -x-
	//             x   y    o
	let mut grid: Vec<Vec<usize>> = vec![vec![0; maxy]; maxx];
	for line in straights {
		if line.is_v() {
			let x = line.x1;
			for y in cmp::min(line.y1, line.y2)..cmp::max(line.y1, line.y2)+1 {
				grid[x][y] += 1;
			}
		} else if line.is_h() {
			let y = line.y1;
			for x in cmp::min(line.x1, line.x2)..cmp::max(line.x1, line.x2)+1 {
				grid[x][y] += 1;
			}
		}
	}

	//debugging
	for y in 0..maxy {
		for x in 0..maxx {
			let o = grid[x][y];
			if o == 0 {
				print!(".");
			} else {
				print!("{}", o);
			}
		}
		println!();
	}

	grid.iter().map(|x| x.iter().map(|y| if *y > 1 { 1 } else { 0 }).sum::<usize>()).sum()
	//0
}

#[aoc(day5, part2)]
pub fn part2(input: &[Line]) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::{gen, part1, part2};
	
	#[test]
	fn sample1() {
		assert_eq!(part1(&gen("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2")), 5);
	}
	
	#[test]
	fn sample2() {
		assert_eq!(part2(&gen("")), 5);
	}
}
