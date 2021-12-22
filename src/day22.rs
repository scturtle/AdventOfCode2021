use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Cube(i32, i32, i32, i32, i32, i32);

impl Cube {
    fn intersect_with(self, other: Cube) -> bool {
        !(self.0 > other.1
            || self.1 < other.0
            || self.2 > other.3
            || self.3 < other.2
            || self.4 > other.5
            || self.5 < other.4)
    }
    fn contains(self, other: Cube) -> bool {
        self.0 <= other.0
            && other.1 <= self.1
            && self.2 <= other.2
            && other.3 <= self.3
            && self.4 <= other.4
            && other.5 <= self.5
    }
    fn cut_along(self, axis: usize, pos: i32) -> Vec<Cube> {
        let Cube(x0, x1, y0, y1, z0, z1) = self;
        match axis {
            0 => {
                if pos < x0 || pos + 1 > x1 {
                    vec![self]
                } else {
                    vec![
                        Cube(x0, pos, y0, y1, z0, z1),
                        Cube(pos + 1, x1, y0, y1, z0, z1),
                    ]
                }
            }
            1 => {
                if pos < y0 || pos + 1 > y1 {
                    vec![self]
                } else {
                    vec![
                        Cube(x0, x1, y0, pos, z0, z1),
                        Cube(x0, x1, pos + 1, y1, z0, z1),
                    ]
                }
            }
            _ => {
                if pos < z0 || pos + 1 > z1 {
                    return vec![self];
                } else {
                    vec![
                        Cube(x0, x1, y0, y1, z0, pos),
                        Cube(x0, x1, y0, y1, pos + 1, z1),
                    ]
                }
            }
        }
    }
    fn cut_by(self, other: Cube) -> Vec<Cube> {
        let Cube(x0, x1, y0, y1, z0, z1) = other;
        [self]
            .into_iter()
            .flat_map(|c| c.cut_along(0, x0 - 1).into_iter())
            .flat_map(|c| c.cut_along(0, x1).into_iter())
            .flat_map(|c| c.cut_along(1, y0 - 1).into_iter())
            .flat_map(|c| c.cut_along(1, y1).into_iter())
            .flat_map(|c| c.cut_along(2, z0 - 1).into_iter())
            .flat_map(|c| c.cut_along(2, z1).into_iter())
            .collect_vec()
    }
    fn count(self) -> i64 {
        let Cube(x0, x1, y0, y1, z0, z1) = self;
        (x1 - x0 + 1) as i64 * (y1 - y0 + 1) as i64 * (z1 - z0 + 1) as i64
    }
}

#[test]
fn run() {
    let txt = crate::common::get_input(22).unwrap();
    let mut steps: Vec<(Cube, bool)> = vec![];
    for line in txt.lines() {
        let (on, region) = line.split(' ').collect_tuple().unwrap();
        let ((x0, x1), (y0, y1), (z0, z1)) = region
            .split(',')
            .map(|r| {
                r[2..]
                    .split("..")
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();
        steps.push((Cube(x0, x1, y0, y1, z0, z1), on == "on"));
    }
    let mut cubes: Vec<Cube> = vec![];
    for (cur, is_on) in steps {
        let mut nxt_cubes = vec![];
        if is_on {
            nxt_cubes.push(cur);
        }
        for pre in cubes {
            // no cut if two cubes are not intersected
            // otherwise there are too many cubes
            if !pre.intersect_with(cur) {
                nxt_cubes.push(pre);
                continue;
            }
            for cutted in pre.cut_by(cur) {
                if cur.contains(cutted) {
                    continue;
                } else {
                    nxt_cubes.push(cutted);
                }
            }
        }
        cubes = nxt_cubes;
    }
    dbg!(cubes.into_iter().map(Cube::count).sum::<i64>());
}
