use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point(i32, i32, i32);

impl Point {
    #[rustfmt::skip]
    fn rotate(&self, n: usize) -> Self {
        let &Point(x, y, z) = self;
        assert!(n < 24);
        let (x, y, z) = [
            (x, z, -y), (-z, x, -y), (-x, -z, -y), (z, -x, -y), (z, -y, x), (y, z, x),
            (-z, y, x), (-y, -z, x), (-y, x, z), (-x, -y, z), (y, -x, z), (x, y, z),
            (-z, -x, y), (x, -z, y), (z, x, y), (-x, z, y), (-x, y, -z), (-y, -x, -z),
            (x, -y, -z), (y, x, -z), (y, -z, -x), (z, y, -x), (-y, z, -x), (-z, -y, -x),
        ][n];
        Point(x, y, z)
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        let Point(x0, y0, z0) = self;
        let Point(x1, y1, z1) = rhs;
        Point(x0 + x1, y0 + y1, z0 + z1)
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        let Point(x0, y0, z0) = self;
        let Point(x1, y1, z1) = rhs;
        Point(x0 - x1, y0 - y1, z0 - z1)
    }
}

struct Scanner {
    pos: Point,
    points: Vec<Point>,
    aligned: bool,
}

impl From<&str> for Scanner {
    fn from(s: &str) -> Self {
        let mut points = vec![];
        let mut lines = s.lines();
        assert!(lines.next().unwrap().starts_with("---"));
        for line in lines {
            let (x, y, z) = line
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            points.push(Point(x, y, z));
        }
        Scanner {
            pos: Point(0, 0, 0),
            points,
            aligned: false,
        }
    }
}

impl Scanner {
    fn align_to(&self, target: &Scanner) -> Option<Scanner> {
        assert!(target.aligned);
        for n in 0..24 {
            let points = self.points.iter().map(|p| p.rotate(n)).collect_vec();
            for i in 0..points.len() {
                for j in 0..target.points.len() {
                    let diff = target.points[j] - points[i];
                    let aligned_points = points.iter().map(|p| *p + diff).collect_vec();
                    assert_eq!(aligned_points[i], target.points[j]);
                    let mut cnt = 0;
                    for aligned_point in &aligned_points {
                        for target_point in &target.points {
                            if aligned_point == target_point {
                                cnt += 1;
                            }
                        }
                    }
                    if cnt >= 12 {
                        return Some(Scanner {
                            pos: diff,
                            points: aligned_points,
                            aligned: true,
                        });
                    }
                }
            }
        }
        None
    }
}

#[test]
fn run() {
    let txt = crate::common::get_input(19).unwrap();
    let mut scanners: Vec<Scanner> = txt.split("\n\n").map(Scanner::from).collect();
    // let all align to scanner 0
    scanners[0].aligned = true;
    // pairwise alignment
    while !scanners.iter().all(|s| s.aligned) {
        for i in 0..scanners.len() {
            for j in 0..scanners.len() {
                if !scanners[i].aligned && scanners[j].aligned {
                    if let Some(aligned_scanner) = scanners[i].align_to(&scanners[j]) {
                        scanners[i] = aligned_scanner;
                    }
                }
            }
        }
    }
    // part 1
    let all_points: std::collections::BTreeSet<_> =
        scanners.iter().flat_map(|s| s.points.iter()).collect();
    dbg!(all_points.len());
    // part 2
    let mut max_distance = 0;
    for i in 0..scanners.len() {
        for j in i + 1..scanners.len() {
            let dis = scanners[i].pos - scanners[j].pos;
            max_distance = max_distance.max(dis.0.abs() + dis.1.abs() + dis.2.abs());
        }
    }
    dbg!(max_distance);
}
