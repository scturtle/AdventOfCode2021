use itertools::Itertools;

fn fold(pts: Vec<(i32, i32)>, f: i32, on_x: bool) -> Vec<(i32, i32)> {
    let mut new_pts = vec![];
    for (mut y, mut x) in pts.into_iter() {
        if !on_x {
            if x > f {
                x -= 2 * (x - f);
            }
        } else if y > f {
            y -= 2 * (y - f);
        }
        new_pts.push((y, x));
    }
    new_pts.sort_unstable();
    new_pts.dedup();
    new_pts
}

fn show(pts: &[(i32, i32)]) {
    let maxy = *pts.iter().map(|(y, _)| y).max().unwrap() as usize;
    let maxx = *pts.iter().map(|(_, x)| x).max().unwrap() as usize;
    let mut canvas = vec![vec![b'.'; maxy + 1]; maxx + 1];
    for &(y, x) in pts {
        canvas[x as usize][y as usize] = b'#';
    }
    for l in canvas {
        println!("{}", String::from_utf8_lossy(&l));
    }
}

#[test]
fn run() {
    let txt = crate::common::get_input(13).unwrap();
    let mut lines = txt.lines();
    let mut pts: Vec<(i32, i32)> = vec![];
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        pts.push(
            line.split(',')
                .collect_tuple()
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap(),
        );
    }
    for line in lines {
        let (axis, f) = line[11..].split('=').collect_tuple().unwrap();
        let f: i32 = f.parse().unwrap();
        pts = fold(pts, f, axis == "x");
    }
    show(&pts);
}
