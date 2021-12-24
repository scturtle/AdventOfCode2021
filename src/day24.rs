use std::collections::BTreeSet;

#[test]
fn run() {
    let txt = crate::common::get_input(24).unwrap();
    let lines: Vec<_> = txt.lines().collect();
    let mut abcs: Vec<(i32, i32, i32)> = vec![];
    for i in 0..14 {
        let a: i32 = lines[i * 18 + 4]
            .split_whitespace()
            .last()
            .and_then(|t| t.parse::<i32>().ok())
            .unwrap();
        let b: i32 = lines[i * 18 + 5]
            .split_whitespace()
            .last()
            .and_then(|t| t.parse::<i32>().ok())
            .unwrap();
        let c: i32 = lines[i * 18 + 15]
            .split_whitespace()
            .last()
            .and_then(|t| t.parse::<i32>().ok())
            .unwrap();
        abcs.push((a, b, c));
    }

    fn forward(w: i32, z: i32, abc: (i32, i32, i32)) -> i32 {
        // x = z0 % 26 + b != w
        // z = (z0 / a) * (25 * x + 1) + (w + c) * x
        let (a, b, c) = abc;
        let x = ((z % 26 + b) != w) as i32;
        z / a * (25 * x + 1) + (w + c) * x
    }

    fn backward(z: i32, abc: (i32, i32, i32)) -> BTreeSet<(i32, i32)> {
        let (a, b, c) = abc;
        let mut w0z0 = BTreeSet::new();
        for w0 in 1..=9 {
            // all z >= 0 because of z % 26
            if a == 1 {
                // when a == 1, all b > 9, x != 0
                // ==> z - w - c == z0 * 26
                let z_w0_c = z - w0 - c;
                if z_w0_c >= 0 && z_w0_c % 26 == 0 {
                    let z0 = z_w0_c / 26;
                    w0z0.insert((w0, z0));
                }
            } else {
                // a == 26
                // if x == 0, z0 % 26 + b == w0 && z == z0 / 26
                for z0 in 26 * z..26 * (z + 1) {
                    if z0 % 26 + b == w0 {
                        w0z0.insert((w0, z0));
                    }
                }
                // if x == 1, z0 % 26 + b != w0 &&
                // z - w - c == z0 // 26 * 26
                let z_w0_c = z - w0 - c;
                if z_w0_c >= 0 && z_w0_c % 26 == 0 {
                    let t = z_w0_c / 26;
                    for z0 in 26 * t..26 * (t + 1) {
                        if z0 % 26 + b != w0 {
                            w0z0.insert((w0, z0));
                        }
                    }
                }
            }
        }
        w0z0
    }

    // the final z should be 0
    let mut expect_zs: BTreeSet<_> = [0].into_iter().collect();
    let mut w0z0s = vec![];
    for &abc in abcs.iter().rev() {
        // analyze backward all possible w0 and z0 at each step given next needed z's
        let mut w0z0 = BTreeSet::new();
        for z in expect_zs {
            w0z0.append(&mut backward(z, abc));
        }
        expect_zs = w0z0.iter().map(|&(_, z)| z).collect();
        w0z0s.push(w0z0);
    }
    w0z0s.reverse();

    // part1
    let mut ans1 = String::new();
    let mut z = 0;
    for (w0z0, &abc) in w0z0s.iter().zip(abcs.iter()) {
        let w = w0z0
            .iter()
            .filter_map(|&(w0, z0)| if z == z0 { Some(w0) } else { None })
            .max()
            .unwrap();
        z = forward(w, z, abc);
        ans1.push((w as u8 + b'0') as char);
    }
    dbg!(ans1);

    // part2
    let mut ans2 = String::new();
    let mut z = 0;
    for (w0z0, &abc) in w0z0s.iter().zip(abcs.iter()) {
        let w = w0z0
            .iter()
            .filter_map(|&(w0, z0)| if z == z0 { Some(w0) } else { None })
            .min()
            .unwrap();
        z = forward(w, z, abc);
        ans2.push((w as u8 + b'0') as char);
    }
    dbg!(ans2);
}
