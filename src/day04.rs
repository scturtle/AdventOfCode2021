#[allow(dead_code)]
#[derive(Debug)]
struct Board {
    nums: Vec<u32>,
    mark: Vec<bool>,
    win: bool,
    score: u32,
}

#[allow(dead_code)]
impl Board {
    fn new(nums: Vec<u32>) -> Self {
        Self {
            nums,
            mark: vec![false; 25],
            win: false,
            score: 0,
        }
    }
    fn is_win(&self) -> bool {
        for i in 0..5 {
            if self.mark[i * 5..(i + 1) * 5].iter().all(|m| *m) {
                return true;
            }
            if (0..5).map(|j| self.mark[i + 5 * j]).all(|m| m) {
                return true;
            }
        }
        false
    }
    fn get_score(&self, num: u32) -> u32 {
        let mut unmarked = 0;
        for (n, m) in self.nums.iter().zip(self.mark.iter()) {
            if !*m {
                unmarked += n;
            }
        }
        unmarked * num
    }
    fn mark(&mut self, num: u32) {
        for (n, m) in self.nums.iter().zip(self.mark.iter_mut()) {
            if *n == num {
                *m = true;
            }
        }
        if self.is_win() {
            self.win = true;
            self.score = self.get_score(num);
        }
    }
}

#[test]
fn run() {
    let txt = crate::common::get_input(4).unwrap();
    let mut lines = txt.lines();
    let all_nums: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let mut boards = vec![];
    while let Some(_) = lines.next() {
        let mut nums = vec![];
        for _ in 0..5 {
            nums.extend(
                lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap()),
            );
        }
        assert_eq!(nums.len(), 25);
        boards.push(Board::new(nums));
    }
    let mut first_score = None;
    let mut last_score = None;
    for &num in &all_nums {
        for board in &mut boards {
            if board.win {
                continue;
            }
            board.mark(num);
            if board.win {
                if first_score.is_none() {
                    first_score = Some(board.score);
                }
                last_score = Some(board.score);
            }
        }
    }
    dbg!(first_score);
    dbg!(last_score);
}
