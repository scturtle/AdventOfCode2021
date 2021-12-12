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

#[test]
fn run() {
    let txt = crate::common::get_input(9).unwrap();
    let m: Vec<Vec<u8>> = txt
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();
    let h = m.len();
    let w = m[0].len();
    let mut ans1: usize = 0;
    let mut basins = vec![];
    for (i, l) in m.iter().enumerate() {
        for (j, &height) in l.iter().enumerate() {
            let mut is_low_point = true;
            for Pos(ni, nj) in Pos(i, j).around(h, w) {
                if m[ni][nj] <= height {
                    is_low_point = false;
                }
            }
            if is_low_point {
                ans1 += 1 + height as usize;
            } else {
                continue;
            }
            let mut saw = std::collections::BTreeSet::new();
            let mut q = std::collections::VecDeque::new();
            saw.insert(Pos(i, j));
            q.push_back(Pos(i, j));
            while let Some(cur) = q.pop_front() {
                for nxt in cur.around(h, w) {
                    if saw.contains(&nxt) || m[nxt.0][nxt.1] == 9 {
                        continue;
                    }
                    saw.insert(nxt);
                    q.push_back(nxt);
                }
            }
            basins.push(saw.len());
        }
    }
    dbg!(ans1);
    basins.sort_unstable();
    basins.reverse();
    dbg!(basins[0] * basins[1] * basins[2]);
}
