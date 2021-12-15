use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Pos(usize, usize);
impl Pos {
    pub fn around(&self, n: usize, m: usize) -> impl Iterator<Item = Pos> {
        let (ci, cj) = (self.0 as i32, self.1 as i32);
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .map(move |(i, j)| (ci + i, cj + j))
            .filter(move |&(i, j)| i >= 0 && j >= 0 && i < n as i32 && j < m as i32)
            .map(|(i, j)| Pos(i as usize, j as usize))
    }
}

fn dijkstra(mat: Vec<Vec<usize>>) -> usize {
    let (n, m) = (mat.len(), mat[0].len());
    let mut pq = BinaryHeap::new();
    let mut dist = HashMap::new();
    let mut saw = HashSet::new();
    pq.push((Reverse(0), Pos(0, 0)));
    dist.insert(Pos(0, 0), 0);
    while let Some((_, u)) = pq.pop() {
        if u == Pos(n - 1, m - 1) {
            return dist[&u];
        }
        if saw.contains(&u) {
            continue;
        }
        saw.insert(u);
        for v in u.around(n, m) {
            let w = mat[v.0][v.1];
            if *dist.get(&v).unwrap_or(&usize::MAX) > dist[&u] + w {
                dist.insert(v, dist[&u] + w);
                pq.push((Reverse(dist[&v]), v));
            }
        }
    }
    unreachable!()
}

#[test]
fn run() {
    let txt = crate::common::get_input(15).unwrap();
    let mat = txt
        .lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as usize).collect_vec())
        .collect_vec();
    let (n, m) = (mat.len(), mat[0].len());
    let mut mat55 = vec![vec![0; m * 5]; n * 5];
    for i in 0..n * 5 {
        for j in 0..m * 5 {
            let mut t = mat[i % n][j % m] + i / n + j / m;
            while t > 9 {
                t -= 9;
            }
            mat55[i][j] = t;
        }
    }
    dbg!(dijkstra(mat));
    dbg!(dijkstra(mat55));
}
