#[test]
fn run() {
    let txt = crate::common::get_input(10).unwrap();
    let lines: Vec<_> = txt.lines().collect();
    use std::collections::HashMap;
    let map: HashMap<u8, u8> = [(b'(', b')'), (b'[', b']'), (b'{', b'}'), (b'<', b'>')]
        .into_iter()
        .collect();
    let score1_map: HashMap<u8, u64> = [(b')', 3), (b']', 57), (b'}', 1197), (b'>', 25137)]
        .into_iter()
        .collect();
    let score2_map: HashMap<u8, u64> = [(b'(', 1), (b'[', 2), (b'{', 3), (b'<', 4)]
        .into_iter()
        .collect();
    let mut score1 = 0;
    let mut score2s = vec![];
    for line in lines {
        let mut stack = vec![];
        let mut corrupted = false;
        for c in line.bytes() {
            if map.contains_key(&c) {
                stack.push(c);
            } else {
                let l = stack.pop();
                if l.is_none() || !map.contains_key(&l.unwrap()) || map[&l.unwrap()] != c {
                    corrupted = true;
                    score1 += score1_map[&c];
                    break;
                }
            }
        }
        if !corrupted {
            let mut score2 = 0;
            for l in stack.iter().rev() {
                score2 = score2 * 5 + score2_map[l];
            }
            score2s.push(score2);
        }
    }
    dbg!(score1);
    score2s.sort_unstable();
    dbg!(score2s[score2s.len() / 2]);
}
