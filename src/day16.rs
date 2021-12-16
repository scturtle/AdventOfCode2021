use std::collections::VecDeque;

struct PackageDecoder {
    q: VecDeque<u8>,
    total_version: usize,
}

impl PackageDecoder {
    fn new(s: &str) -> Self {
        let mut q = VecDeque::new();
        for c in s.bytes() {
            let c = if c <= b'9' { c - b'0' } else { 10 + c - b'A' };
            for i in (0..4).rev() {
                q.push_back((c & (1 << i)) >> i);
            }
        }
        Self {
            q,
            total_version: 0,
        }
    }
    fn get(&mut self, n: usize) -> usize {
        let mut res = 0;
        for _ in 0..n {
            res <<= 1;
            res |= self.q[0] as usize;
            self.q.pop_front();
        }
        res
    }
    fn parse(&mut self) -> usize {
        let ver = self.get(3);
        self.total_version += ver;
        let typ = self.get(3);
        if typ == 4 {
            // parse literal
            let mut cont = 1;
            let mut n = 0;
            while cont != 0 {
                cont = self.get(1);
                n = (n << 4) | self.get(4);
            }
            return n;
        }
        // operator
        let length_type_id = self.get(1);
        let mut ns = vec![];
        if length_type_id == 0 {
            let bits_length = self.get(15);
            let l0 = self.q.len();
            while l0 - self.q.len() < bits_length {
                ns.push(self.parse());
            }
            assert!(l0 - self.q.len() == bits_length);
        } else {
            let pkgs_length = self.get(11);
            for _ in 0..pkgs_length {
                ns.push(self.parse());
            }
        }
        #[rustfmt::skip]
        match typ {
            0 => ns.into_iter().sum(),
            1 => ns.into_iter().product(),
            2 => ns.into_iter().min().unwrap(),
            3 => ns.into_iter().max().unwrap(),
            5 => if ns[0] > ns[1] { 1 } else { 0 },
            6 => if ns[0] < ns[1] { 1 } else { 0 },
            7 => if ns[0] == ns[1] { 1 } else { 0 },
            _ => panic!(),
        }
    }
}

#[test]
fn run() {
    let txt = crate::common::get_input(16).unwrap();
    let mut p = PackageDecoder::new(txt.trim_end());
    let result = p.parse();
    dbg!(p.total_version);
    dbg!(result);
}
