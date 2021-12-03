#[test]
fn run() {
    let txt = crate::common::get_input(3).unwrap();
    let lines = txt.lines().collect::<Vec<_>>();
    let n = lines.len();
    let m = lines[0].len();
    {
        let mut cnt1 = vec![0; m];
        for l in &lines {
            for (j, c) in l.bytes().enumerate() {
                cnt1[j] += (c == b'1') as usize;
            }
        }
        let mut gamma = 0;
        let mut epsilon = 0;
        for (j, cnt1) in cnt1.into_iter().enumerate() {
            let cnt0 = n - cnt1;
            if cnt1 > cnt0 {
                gamma |= 1 << (m - 1 - j);
            } else {
                epsilon |= 1 << (m - 1 - j);
            }
        }
        dbg!(gamma * epsilon);
    }
    // part2
    let mut keep_o = vec![true; n];
    let mut keep_c = vec![true; n];
    let mut tot_o = n;
    let mut tot_c = n;
    let mut rate_o = 0;
    let mut rate_c = 0;
    for j in 0..m {
        let mut cnt1_o = 0;
        let mut cnt1_c = 0;
        for (i, l) in lines.iter().enumerate() {
            let c_is_1 = l.as_bytes()[j] == b'1';
            cnt1_o += (keep_o[i] && c_is_1) as usize;
            cnt1_c += (keep_c[i] && c_is_1) as usize;
        }
        let cnt0_o = tot_o - cnt1_o;
        let cnt0_c = tot_c - cnt1_c;
        for (i, l) in lines.iter().enumerate() {
            let c_is_0 = l.as_bytes()[j] == b'0';
            if (cnt1_o > cnt0_o && c_is_0)
                || (cnt1_o < cnt0_o && !c_is_0)
                || (cnt1_o == cnt0_o && c_is_0)
            {
                keep_o[i] = false;
            }
            if (cnt1_c > cnt0_c && !c_is_0)
                || (cnt1_c < cnt0_c && c_is_0)
                || (cnt1_c == cnt0_c && !c_is_0)
            {
                keep_c[i] = false;
            }
        }
        tot_o = keep_o.iter().map(|&b| b as usize).sum::<usize>();
        tot_c = keep_c.iter().map(|&b| b as usize).sum::<usize>();
        if tot_o == 1 {
            let pos = keep_o.iter().position(|&b| b).unwrap();
            rate_o = i32::from_str_radix(lines[pos], 2).unwrap();
        }
        if tot_c == 1 {
            let pos = keep_c.iter().position(|&b| b).unwrap();
            rate_c = i32::from_str_radix(lines[pos], 2).unwrap();
        }
    }
    dbg!(rate_o * rate_c);
}
