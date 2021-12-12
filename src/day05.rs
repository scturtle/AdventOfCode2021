use itertools::{iproduct, Itertools};
use std::cmp::Ordering::{Equal, Greater, Less};

#[test]
fn run() {
    let txt = crate::common::get_input(5).unwrap();

    let mut lines = vec![];
    for l in txt.lines() {
        let ns = l
            .replace(" -> ", ",")
            .split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect_vec();
        lines.push(((ns[0], ns[1]), (ns[2], ns[3])));
    }

    let mut m1 = vec![vec![0; 1000]; 1000];
    let mut m2 = vec![vec![0; 1000]; 1000];
    for l in &lines {
        let (mut i, ie) = (l.0 .0, l.1 .0);
        let (mut j, je) = (l.0 .1, l.1 .1);
        let istep = match i.cmp(&ie) {
            Equal => 0,
            Less => 1,
            Greater => -1,
        };
        let jstep = match j.cmp(&je) {
            Equal => 0,
            Less => 1,
            Greater => -1,
        };
        loop {
            if l.0 .0 == l.1 .0 || l.0 .1 == l.1 .1 {
                m1[i as usize][j as usize] += 1;
            }
            m2[i as usize][j as usize] += 1;
            if i == ie && j == je {
                break;
            }
            i += istep;
            j += jstep;
        }
    }
    let mut cnt1 = 0;
    let mut cnt2 = 0;
    for (i, j) in iproduct!((0..1000), (0..1000)) {
        if m1[i][j] > 1 {
            cnt1 += 1;
        }
        if m2[i][j] > 1 {
            cnt2 += 1;
        }
    }
    dbg!(cnt1);
    dbg!(cnt2);
}
