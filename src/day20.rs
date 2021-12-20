use itertools::Itertools;

#[test]
fn run() {
    let txt = crate::common::get_input(20).unwrap();
    let mut lines = txt.lines();
    let table = lines
        .next()
        .unwrap()
        .bytes()
        .map(|b| if b == b'#' { 1usize } else { 0 })
        .collect_vec();
    assert_eq!(table.len(), 512);
    lines.next();
    let mat = lines
        .map(|l| {
            l.bytes()
                .map(|b| if b == b'#' { 1usize } else { 0 })
                .collect_vec()
        })
        .collect_vec();
    let step = |map: Vec<Vec<usize>>, flip: bool| -> Vec<Vec<usize>> {
        let mut nxt = vec![vec![0; map[0].len() + 2]; map.len() + 2];
        for i in -1..=map.len() as i32 {
            for j in -1..=map[0].len() as i32 {
                let mut pos = 0;
                for ii in i - 1..=i + 1 {
                    for jj in j - 1..=j + 1 {
                        pos = pos * 2
                            + if ii < 0
                                || ii >= map.len() as i32
                                || jj < 0
                                || jj >= map[0].len() as i32
                            {
                                if flip {
                                    1
                                } else {
                                    0
                                }
                            } else {
                                map[ii as usize][jj as usize]
                            };
                    }
                }
                nxt[(i + 1) as usize][(j + 1) as usize] = table[pos as usize];
            }
        }
        nxt
    };
    let mut mat = mat;
    for i in 0..50 {
        mat = step(mat, i % 2 == 1);
        if i == 1 {
            dbg!(mat.iter().map(|l| l.iter().sum::<usize>()).sum::<usize>());
        }
    }
    dbg!(mat.iter().map(|l| l.iter().sum::<usize>()).sum::<usize>());
}
