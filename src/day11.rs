#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Pos(usize, usize);
impl Pos {
    #[rustfmt::skip]
    #[allow(dead_code)]
    pub fn around(&self, n: usize, m: usize) -> impl Iterator<Item = Pos> {
        let (ci, cj) = (self.0 as i32, self.1 as i32);
        [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1),]
        .iter()
        .map(move |(i, j)| (ci + i, cj + j))
        .filter(move |&(i, j)| i >= 0 && j >= 0 && i < n as i32 && j < m as i32)
        .map(|(i, j)| Pos(i as usize, j as usize))
    }
}

#[allow(dead_code)]
fn show(octs: &[Vec<u8>]) {
    let mut s = String::new();
    for l in octs {
        for c in l {
            if *c == 0 {
                s.push_str("\x1b[1;33m");
                s.push((c + b'0') as char);
                s.push_str("\x1b[0m");
            } else {
                s.push((c + b'0') as char);
            }
        }
        s.push('\n');
    }
    println!("{}", s);
}

#[allow(dead_code)]
fn step(octs: &mut Vec<Vec<u8>>) -> usize {
    for l in octs.iter_mut() {
        for c in l {
            *c += 1;
        }
    }
    let mut cnt = 0;
    loop {
        let mut new = false;
        for i in 0..10 {
            for j in 0..10 {
                if octs[i][j] > 9 {
                    octs[i][j] = 0;
                    cnt += 1;
                    new = true;
                    for Pos(ni, nj) in Pos(i, j).around(10, 10) {
                        if octs[ni][nj] != 0 {
                            // not flashed in this step
                            octs[ni][nj] += 1;
                        }
                    }
                }
            }
        }
        if !new {
            // no new flashed
            break;
        }
    }
    cnt
}

#[test]
fn run() {
    let txt = crate::common::get_input(11).unwrap();
    let mut octs: Vec<Vec<u8>> = txt
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();
    let mut cnt1 = 0;
    for i in 1.. {
        let flashed = step(&mut octs);
        cnt1 += flashed;
        if i == 100 {
            dbg!(cnt1);
        }
        if flashed == 100 {
            dbg!(i);
            break;
        }
    }
}
