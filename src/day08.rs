use itertools::Itertools;
use std::collections::BTreeMap;

#[rustfmt::skip]
#[allow(dead_code, clippy::many_single_char_names)]
fn guess(mut is: Vec<&str>, os: Vec<&str>) -> (usize, usize) {
    is.sort_by_key(|w| w.len());
    let mut cnt235 = BTreeMap::<char, usize>::new();
    for w in &is[3..6] {
        for c in w.chars() {
            *cnt235.entry(c).or_default() += 1;
        }
    }
    let mut cnt069 = BTreeMap::<char, usize>::new();
    for w in &is[6..9] {
        for c in w.chars() {
            *cnt069.entry(c).or_default() += 1;
        }
    }
    let a = is[1].chars().find(|c| !is[0].contains(|c2| c == &c2)).unwrap();
    let b = *cnt235.iter().find(|(k, &v)| v == 1 && cnt069[k] == 3).unwrap().0;
    let c = *cnt235.iter().find(|(k, &v)| v == 2 && cnt069[k] == 2).unwrap().0;
    let d = *cnt235.iter().find(|(k, &v)| v == 3 && cnt069[k] == 2).unwrap().0;
    let e = *cnt235.iter().find(|(k, &v)| v == 1 && cnt069[k] == 2).unwrap().0;
    let f = *cnt235.iter().find(|(k, &v)| v == 2 && cnt069[k] == 3).unwrap().0;
    let g = *cnt235.iter().find(|(k, &v)| v == 3 && cnt069[k] == 3 && k != &&a).unwrap().0;
    let aft = [a, b, c, d, e, f, g];
    let correct: BTreeMap<char, char> = aft.into_iter().zip('a'..='g').collect();
    let mut res1 = 0;
    let mut res2 = 0;
    for w in os {
        let w: String = w.chars().map(|c| correct[&c]).sorted().collect();
        let d = match w.as_str() {
            "abcefg" => 0, "cf" => 1, "acdeg" => 2, "acdfg" => 3, "bcdf" => 4,
            "abdfg" => 5, "abdefg" => 6, "acf" => 7, "abcdefg" => 8, "abcdfg" => 9,
            _ => panic!("{}", w),
        };
        if [1, 4, 7, 8].contains(&d) {
            res1 += 1;
        }
        res2 = res2 * 10 + d;
    }
    (res1, res2)
}

#[test]
fn run() {
    let txt = crate::common::get_input(8).unwrap();
    let mut res1 = 0;
    let mut res2 = 0;
    for l in txt.lines() {
        let (is, os) = l.split(" | ").collect_tuple::<(_, _)>().unwrap();
        let is = is.split_ascii_whitespace().collect_vec();
        let os = os.split_ascii_whitespace().collect_vec();
        let t = guess(is, os);
        res1 += t.0;
        res2 += t.1;
    }
    dbg!(res1, res2);
}
