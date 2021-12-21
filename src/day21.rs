use itertools::Itertools;
use std::collections::HashMap;

#[test]
fn run() {
    let txt = crate::common::get_input(21).unwrap();
    let mut lines = txt.lines();
    let pos1: usize = lines.next().unwrap()[28..].parse().unwrap();
    let pos2: usize = lines.next().unwrap()[28..].parse().unwrap();

    let mut multi_of_dices = vec![0; 10];
    for (i, j, k) in itertools::iproduct!(1..=3, 1..=3, 1..=3) {
        multi_of_dices[i + j + k] += 1;
    }

    let mut states = HashMap::new();
    let mut win1: u64 = 0;
    let mut win2: u64 = 0;
    states.insert((pos1, 0, pos2, 0, true), 1);
    while !states.is_empty() {
        let mut new_states = HashMap::new();
        for ((pos1, score1, pos2, score2, next_is_score1), cnt) in states {
            #[allow(clippy::needless_range_loop)]
            for dices in 3..=9 {
                if next_is_score1 {
                    let new_pos1 = (pos1 + dices - 1) % 10 + 1;
                    let new_score1 = score1 + new_pos1;
                    let new_cnt = cnt * multi_of_dices[dices];
                    let new_state = (new_pos1, new_score1, pos2, score2, false);
                    if new_score1 >= 21 {
                        win1 += new_cnt;
                    } else {
                        *new_states.entry(new_state).or_default() += new_cnt;
                    }
                } else {
                    let new_pos2 = (pos2 + dices - 1) % 10 + 1;
                    let new_score2 = score2 + new_pos2;
                    let new_cnt = cnt * multi_of_dices[dices];
                    let new_state = (pos1, score1, new_pos2, new_score2, true);
                    if new_score2 >= 21 {
                        win2 += new_cnt;
                    } else {
                        *new_states.entry(new_state).or_default() += new_cnt;
                    }
                }
            }
        }
        states = new_states;
    }
    dbg!(win1, win2);
}
