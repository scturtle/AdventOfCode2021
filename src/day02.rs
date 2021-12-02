#[test]
fn run() {
    let txt = crate::common::get_input(2).unwrap();
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;
    let mut depth2 = 0;
    for line in txt.lines() {
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        let n: i64 = words[1].parse().unwrap();
        match words[0] {
            "forward" => {
                horiz += n;
                depth2 += aim * n;
            }
            "down" => {
                depth += n;
                aim += n;
            }
            "up" => {
                depth -= n;
                aim -= n;
            }
            _ => unreachable!(),
        }
    }
    dbg!(horiz * depth);
    dbg!(horiz * depth2);
}
