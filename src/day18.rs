use itertools::Itertools;

#[derive(Debug, Clone)]
enum Elem {
    Num(usize),
    Pair(Box<Elem>, Box<Elem>),
}

impl std::fmt::Display for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Elem::Num(n) => write!(f, "{}", n),
            Elem::Pair(l, r) => write!(f, "[")
                .and(l.fmt(f))
                .and(write!(f, ","))
                .and(r.fmt(f))
                .and(write!(f, "]")),
        }
    }
}

impl From<&[u8]> for Elem {
    fn from(s: &[u8]) -> Self {
        if s[0].is_ascii_digit() {
            let s = String::from_utf8_lossy(s);
            Elem::Num(s.parse().unwrap())
        } else {
            let mut i = 0;
            let mut cnt = 0;
            loop {
                if cnt == 1 && s[i] == b',' {
                    break;
                }
                cnt = match s[i] {
                    b'[' => cnt + 1,
                    b']' => cnt - 1,
                    _ => cnt,
                };
                i += 1;
            }
            Elem::Pair(
                Box::new(Elem::from(&s[1..i])),
                Box::new(Elem::from(&s[i + 1..s.len() - 1])),
            )
        }
    }
}

impl Elem {
    fn magnitude(&self) -> usize {
        match self {
            Elem::Num(n) => *n,
            Elem::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
    fn add_to_rightest(&mut self, val: usize) {
        match self {
            Elem::Num(n) => *n += val,
            Elem::Pair(_, r) => r.add_to_rightest(val),
        }
    }
    fn add_to_leftest(&mut self, val: usize) {
        match self {
            Elem::Num(n) => *n += val,
            Elem::Pair(l, _) => l.add_to_leftest(val),
        }
    }
    fn do_explode(&mut self, nested_cnt: usize) -> (bool, Option<usize>, Option<usize>) {
        match self {
            Elem::Num(_) => (false, None, None),
            Elem::Pair(l, r) => {
                if nested_cnt >= 4 {
                    let pair_of_two_nums: Option<(usize, usize)> = match (l.as_ref(), r.as_ref()) {
                        (&Elem::Num(l), &Elem::Num(r)) => Some((l, r)),
                        _ => None,
                    };
                    if let Some((l, r)) = pair_of_two_nums {
                        *self = Elem::Num(0);
                        return (true, Some(l), Some(r));
                    }
                }
                let (lexp, ll, lr) = l.do_explode(nested_cnt + 1);
                if lexp {
                    if let Some(val) = lr {
                        r.add_to_leftest(val);
                    }
                    (true, ll, None)
                } else {
                    let (rexp, rl, rr) = r.do_explode(nested_cnt + 1);
                    if rexp {
                        if let Some(val) = rl {
                            l.add_to_rightest(val);
                        }
                        (true, None, rr)
                    } else {
                        (false, None, None)
                    }
                }
            }
        }
    }
    fn do_split(&mut self) -> bool {
        match self {
            Elem::Num(n) => {
                let n = *n;
                if n >= 10 {
                    *self = Elem::Pair(Box::new(Elem::Num(n / 2)), Box::new(Elem::Num(n - n / 2)));
                    true
                } else {
                    false
                }
            }
            Elem::Pair(l, r) => {
                let lsp = l.do_split();
                if lsp {
                    true
                } else {
                    r.do_split()
                }
            }
        }
    }
}

impl std::ops::Add for Elem {
    type Output = Elem;
    fn add(self, other: Elem) -> Elem {
        let mut root = Elem::Pair(Box::new(self), Box::new(other));
        loop {
            let (exploded, _, _) = root.do_explode(0);
            if exploded {
                continue;
            }
            let splitted = root.do_split();
            if splitted {
                continue;
            }
            return root;
        }
    }
}

#[test]
fn run() {
    let txt = crate::common::get_input(18).unwrap();
    let mut lines = txt.lines();
    let mut res = Elem::from(lines.next().unwrap().as_bytes());
    for line in lines {
        let nxt = Elem::from(line.as_bytes());
        res = res + nxt;
    }
    dbg!(res.magnitude());

    let nums = txt.lines().map(|l| Elem::from(l.as_bytes())).collect_vec();
    let mut max_magnitude = 0;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            max_magnitude = max_magnitude.max((nums[i].clone() + nums[j].clone()).magnitude());
            max_magnitude = max_magnitude.max((nums[j].clone() + nums[i].clone()).magnitude());
        }
    }
    dbg!(max_magnitude);
}
