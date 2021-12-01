#[test]
fn run() {
    let txt = crate::common::get_input(1).unwrap();
    let depths: Vec<i32> = txt.lines().map(|line| line.parse().unwrap()).collect();
    let mut cnt1 = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            cnt1 += 1;
        }
    }
    dbg!(cnt1);
    let mut cnt2 = 0;
    for i in 3..depths.len() {
        if depths[i] > depths[i - 3] {
            cnt2 += 1;
        }
    }
    dbg!(cnt2);
}
