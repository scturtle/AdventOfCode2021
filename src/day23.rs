use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

type Amphipod = (char, u8, u8);
static HALLS: [(u8, u8); 7] = [(1, 1), (1, 2), (1, 4), (1, 6), (1, 8), (1, 10), (1, 11)];

fn move_cost(c: char, from: (u8, u8), to: (u8, u8)) -> usize {
    let cost = match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => unreachable!(),
    };
    cost * (from.0.max(to.0) + from.1.max(to.1) - from.0.min(to.0) - from.1.min(to.1)) as usize
}

fn move_to(poses: &[Amphipod], from: (u8, u8), to: (u8, u8)) -> Option<Vec<Amphipod>> {
    // from hallway to home, or home to hallway
    let mut new_poses = vec![];
    for &(c, i, j) in poses {
        if (i, j) == from {
            // from => to
            new_poses.push((c, to.0, to.1));
            continue;
        }
        // block on the hallway
        if i == from.0.min(to.0) && j >= from.1.min(to.1) && j <= from.1.max(to.1) {
            return None;
        }
        // block in the room
        #[allow(clippy::collapsible_else_if)]
        if from.0 < to.0 {
            if j == to.1 && i <= to.0 {
                return None;
            }
        } else {
            if j == from.1 && i <= from.0 {
                return None;
            }
        }
        new_poses.push((c, i, j));
    }
    new_poses.sort_unstable();
    Some(new_poses)
}

fn home_j_of(c: char) -> u8 {
    match c {
        'A' => 3,
        'B' => 5,
        'C' => 7,
        'D' => 9,
        _ => unreachable!(),
    }
}

fn move_in(poses: &[Amphipod], cost: usize, depth: u8) -> Vec<(Vec<Amphipod>, usize)> {
    let mut result = vec![];
    for &(c, i, j) in poses {
        if i != 1 {
            continue;
        }
        let to_j = home_j_of(c);
        for to_i in 2..2 + depth {
            let to = (to_i, to_j);
            // below are all the same kind of amphipod
            if !(to_i + 1..2 + depth).all(|i3| {
                poses
                    .iter()
                    .any(|&(c2, i2, j2)| c2 == c && i2 == i3 && j2 == to_j)
            }) {
                continue;
            }
            let from = (i, j);
            if let Some(new_poses) = move_to(poses, from, to) {
                let new_cost = cost + move_cost(c, from, to);
                result.push((new_poses, new_cost));
            }
        }
    }
    result
}

fn move_out(poses: &[Amphipod], cost: usize, depth: u8) -> Vec<(Vec<Amphipod>, usize)> {
    let mut result = vec![];
    for &(c, i, j) in poses {
        if i == 1 {
            continue;
        }
        let from = (i, j);
        // at home and below are all the same kind of amphipod
        if j == home_j_of(c)
            && (i + 1..2 + depth).all(|i3| {
                poses
                    .iter()
                    .any(|&(c2, i2, j2)| c2 == c && i2 == i3 && j2 == j)
            })
        {
            continue;
        }
        for to in HALLS {
            if let Some(new_poses) = move_to(poses, from, to) {
                let new_cost = cost + move_cost(c, from, to);
                result.push((new_poses, new_cost));
            }
        }
    }
    result
}

fn is_final(a: &[Amphipod]) -> bool {
    a.iter().all(|&(c, _, j)| j == home_j_of(c))
}

#[test]
fn run() {
    let txt = crate::common::get_input(23).unwrap();
    let mut lines = txt.lines().collect_vec();

    // for part 1
    // let depth = 2u8;
    // for part2
    let depth = 4u8;
    lines.insert(3, "  #D#C#B#A#");
    lines.insert(4, "  #D#B#A#C#");

    let mut poses = vec![];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '#' && c != '.' && c != ' ' {
                poses.push((c, i as u8, j as u8));
            }
        }
    }
    poses.sort_unstable();

    let mut pq = BinaryHeap::new();
    let mut saw = HashSet::new();
    pq.push((Reverse(0), poses));
    while let Some((Reverse(cost), poses)) = pq.pop() {
        if is_final(&poses) {
            dbg!(cost);
            break;
        }
        if saw.contains(&poses) {
            continue;
        }
        saw.insert(poses.clone());
        for (new_poses, new_cost) in move_out(&poses, cost, depth) {
            pq.push((Reverse(new_cost), new_poses));
        }
        for (new_poses, new_cost) in move_in(&poses, cost, depth) {
            pq.push((Reverse(new_cost), new_poses));
        }
    }
}
