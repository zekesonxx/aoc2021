
#[derive(Clone, Debug)]
pub struct BingoBoard {
	pub board: [[(usize, bool); 5]; 5]
}

impl BingoBoard {
	fn new(input: &str) -> Self {
		let mut board = [[(0, false); 5]; 5];
		let mut i = 0;
		for line in input.split('\n') {
			let nums: Vec<(usize, bool)> = line.split_whitespace().map(|x| (x.parse().unwrap(), false)).collect();
			board[i].copy_from_slice(&nums);
			i += 1;
		}
		BingoBoard {
			board: board
		}

	}
	fn mark(&mut self, number: usize) {
		for mut line in &mut self.board {
			for mut column in line {
				if column.0 == number {
					column.1 = true;
				}
			}
		}
	}
	fn has_won(&self) -> bool {
		// horizontals
		'lineloop: for line in self.board {
			// will continue to the next line if any number hasn't won
			for col in line {
				if col.1 == false {
					continue 'lineloop;
				}
			}
			return true;
		}
		// verticals
		'colloop: for i in 0..5 {
			for j in 0..5 {
				if self.board[j][i].1 == false {
					continue 'colloop;
				}
			}
			return true;
		}
		false
	}
}


#[aoc_generator(day4)]
pub fn day4_gen(input: &str) -> (Vec<usize>, Vec<BingoBoard>) {
        let mut sections = input.split("\n\n");
	let nums: Vec<usize> = sections.next().unwrap().split(',').map(|x| x.parse().unwrap()).collect();
	let mut boards: Vec<BingoBoard> = sections.map(|x| BingoBoard::new(x)).collect();
	(nums, boards)
}

#[aoc(day4, part1)]
pub fn part1(input: &(Vec<usize>, Vec<BingoBoard>)) -> usize {
	let nums = &input.0;
	let mut boards = input.1.clone();
	for num in nums {
		println!("{:?}", num);
	}
	0
}

#[aoc(day4, part2)]
pub fn part2(input: &(Vec<usize>, Vec<BingoBoard>)) -> usize {
	0
}

#[cfg(test)]
mod tests {
	use super::{day4_gen, part1, part2, BingoBoard};
	
	#[test]
	fn sample1() {
		assert_eq!(part1(&day4_gen("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7")), 4512);
	}
	
	#[test]
	fn sample2() {
		assert_eq!(part2(&day4_gen("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7")), 5);
	}

	#[test]
	fn board_can_vertical_win() {
		let mut board = BingoBoard::new("22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19");
		assert_eq!(board.has_won(), false);
 		board.mark(22);
		assert_eq!(board.has_won(), false);
 		board.mark(8);
		assert_eq!(board.has_won(), false);
 		board.mark(21);
		assert_eq!(board.has_won(), false);
 		board.mark(6);
		assert_eq!(board.has_won(), false);
 		board.mark(1);
		assert_eq!(board.has_won(), true);
	}
	
	#[test]
	fn board_can_horizontal_win() {
		let mut board = BingoBoard::new("22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19");
		assert_eq!(board.has_won(), false);
 		board.mark(8);
		assert_eq!(board.has_won(), false);
 		board.mark(2);
		assert_eq!(board.has_won(), false);
 		board.mark(23);
		assert_eq!(board.has_won(), false);
 		board.mark(4);
		assert_eq!(board.has_won(), false);
 		board.mark(24);
		assert_eq!(board.has_won(), true);
	}
}
