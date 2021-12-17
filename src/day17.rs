use itertools::Itertools;

#[test]
fn run() {
    let txt = crate::common::get_input(17).unwrap();
    let (xs, ys) = txt.trim()[13..].split(", ").collect_tuple().unwrap();
    let xs: (i32, i32) = xs[2..]
        .split("..")
        .map(|n| n.parse().unwrap())
        .collect_tuple()
        .unwrap();
    let ys: (i32, i32) = ys[2..]
        .split("..")
        .map(|n| n.parse().unwrap())
        .collect_tuple()
        .unwrap();

    let shoot = |mut vx: i32, mut vy: i32| -> Option<i32> {
        let (mut x, mut y) = (0, 0);
        let mut ok = false;
        let mut maxy = y;
        loop {
            x += vx;
            y += vy;
            if vx > 0 {
                vx -= 1;
            } else if vx < 0 {
                vx += 1;
            }
            vy -= 1;
            maxy = maxy.max(y);
            ok |= x >= xs.0 && x <= xs.1 && y >= ys.0 && y <= ys.1;
            if y < ys.0 && vy < 0 {
                break;
            }
        }
        if ok {
            Some(maxy)
        } else {
            None
        }
    };
    let mut maxy = i32::MIN;
    let mut cnt = 0;
    for vx in -300..300 {
        for vy in -300..300 {
            if let Some(y) = shoot(vx, vy) {
                cnt += 1;
                maxy = maxy.max(y);
            }
        }
    }
    dbg!(maxy);
    dbg!(cnt);
}
