use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

#[test]
fn run() {
    let txt = crate::common::get_input(14).unwrap();
    let mut lines = txt.lines();
    let init: Vec<char> = lines.next().unwrap().chars().collect_vec();
    lines.next();
    let mut rules = HashMap::new();
    for line in lines {
        let (bef, aft) = line.split(" -> ").collect_tuple().unwrap();
        let (a, b) = bef.chars().collect_tuple().unwrap();
        rules.insert((a, b), aft.chars().next().unwrap());
    }
    let init = {
        let mut s: BTreeMap<(char, char), usize> = BTreeMap::new();
        s.insert((' ', init[0]), 1);
        s.insert((*init.last().unwrap(), ' '), 1);
        for t in init.windows(2) {
            *s.entry((t[0], t[1])).or_default() += 1;
        }
        s
    };
    let step = |s: BTreeMap<(char, char), usize>| -> BTreeMap<(char, char), usize> {
        let mut ns = BTreeMap::new();
        for ((a, b), cnt) in s {
            if let Some(&c) = rules.get(&(a, b)) {
                *ns.entry((a, c)).or_default() += cnt;
                *ns.entry((c, b)).or_default() += cnt;
            } else {
                *ns.entry((a, b)).or_default() += cnt;
            }
        }
        ns
    };
    let mut s = init;
    for _ in 0..40 {
        s = step(s);
    }
    let mut tot = BTreeMap::<char, usize>::new();
    for ((a, b), cnt) in s {
        *tot.entry(a).or_default() += cnt;
        *tot.entry(b).or_default() += cnt;
    }
    tot.remove(&' ');
    let min = tot.values().min().unwrap();
    let max = tot.values().max().unwrap();
    dbg!((max - min) / 2);
}
