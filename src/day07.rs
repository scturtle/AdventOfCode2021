#[test]
fn run() {
    let txt = crate::common::get_input(7).unwrap();
    let ns: Vec<_> = txt
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    fn cost(ns: &[i32], p: i32) -> i32 {
        ns.iter().map(|n| (n - p).abs()).sum()
    }
    fn cost2(ns: &[i32], p: i32) -> i32 {
        ns.iter()
            .map(|n| {
                let c = (n - p).abs();
                c * (c + 1) / 2
            })
            .sum()
    }
    let min = *ns.iter().min().unwrap();
    let max = *ns.iter().max().unwrap();
    dbg!((min..=max).map(|p| cost(&ns, p)).min().unwrap());
    dbg!((min..=max).map(|p| cost2(&ns, p)).min().unwrap());
}
