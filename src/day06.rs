#[test]
fn run() {
    let txt = crate::common::get_input(6).unwrap();
    let mut orig_cnt: [usize; 9] = [0; 9];
    for n in txt.trim().split(',').map(|n| n.parse::<usize>().unwrap()) {
        orig_cnt[n] += 1;
    }
    fn step(cnt: &mut [usize; 9], nxt_cnt: &mut [usize; 9]) {
        nxt_cnt[..8].clone_from_slice(&cnt[1..9]);
        nxt_cnt[6] += cnt[0];
        nxt_cnt[8] = cnt[0];
        std::mem::swap(cnt, nxt_cnt);
    }
    let mut cnt: [usize; 9] = orig_cnt;
    let mut nxt_cnt: [usize; 9] = [0; 9];
    // part 1
    for _ in 0..80 {
        step(&mut cnt, &mut nxt_cnt);
    }
    dbg!(cnt.iter().sum::<usize>());
    // part 2
    cnt = orig_cnt;
    for _ in 0..256 {
        step(&mut cnt, &mut nxt_cnt);
    }
    dbg!(cnt.iter().sum::<usize>());
}
