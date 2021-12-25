use itertools::Itertools;

#[test]
fn run() {
    let txt = crate::common::get_input(25).unwrap();

    let mut map = txt.lines().map(|l| l.bytes().collect_vec()).collect_vec();
    let n = map.len();
    let m = map[0].len();
    let mut step = 0;
    loop {
        let mut to_move_east = vec![];
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            for j in 0..m {
                if map[i][j] == b'>' && map[i][(j + 1) % m] == b'.' {
                    to_move_east.push((i, j));
                }
            }
        }
        for &(i, j) in &to_move_east {
            map[i][j] = b'.';
            map[i][(j + 1) % m] = b'>';
        }
        let mut to_move_south = vec![];
        for i in 0..n {
            for j in 0..m {
                if map[i][j] == b'v' && map[(i + 1) % n][j] == b'.' {
                    to_move_south.push((i, j));
                }
            }
        }
        for &(i, j) in &to_move_south {
            map[i][j] = b'.';
            map[(i + 1) % n][j] = b'v';
        }
        step += 1;
        if to_move_east.is_empty() && to_move_south.is_empty() {
            dbg!(step);
            break;
        }
    }
}
