use itertools::Itertools;
use std::collections::HashMap;

#[test]
fn run() {
    let txt = crate::common::get_input(12).unwrap();

    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for l in txt.lines() {
        let (from, to) = l.split('-').collect_tuple().unwrap();
        adj.entry(from).or_default().push(to);
        adj.entry(to).or_default().push(from);
    }

    fn dfs<'a>(
        adj: &HashMap<&'a str, Vec<&'a str>>,
        u: &'a str,
        saw: &mut HashMap<&'a str, u8>,
        cnt: &mut usize,
        the_single: &mut Option<&'a str>,
    ) {
        if u == "end" {
            *cnt += 1;
            return;
        }
        let lu = u.to_ascii_lowercase();
        if lu == u {
            let c = saw.entry(u).or_default();
            if *c + 1 == 2 {
                if the_single.is_some() {
                    return;
                }
                *the_single = Some(u);
            }
            *c += 1;
        }
        if let Some(vs) = adj.get(u) {
            for &v in vs {
                if v != "start" && saw.get(v).unwrap_or(&0) < &2 {
                    dfs(adj, v, saw, cnt, the_single);
                }
            }
        }
        if lu == u {
            let c = saw.entry(u).or_default();
            if *c == 2 {
                *the_single = None;
            }
            *c -= 1;
        }
    }
    let mut cnt = 0;
    let mut saw = HashMap::new();
    let mut the_single = Some("dummy");
    dfs(&adj, "start", &mut saw, &mut cnt, &mut the_single);
    dbg!(cnt);
    cnt = 0;
    the_single = None;
    dfs(&adj, "start", &mut saw, &mut cnt, &mut the_single);
    dbg!(cnt);
}
