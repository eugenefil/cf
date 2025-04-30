fn main() {
    let mut cards = std::io::stdin().lines().skip(1).next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<std::collections::VecDeque<u32>>();
    let mut p1 = 0u32;
    let mut p2 = 0u32;
    let mut i = 0usize;
    while cards.len() > 0 {
        let n = if *cards.front().unwrap() > *cards.back().unwrap() {
            cards.pop_front().unwrap()
        } else {
            cards.pop_back().unwrap()
        };
        if i % 2 == 0 {
            p1 += n;
        } else {
            p2 += n;
        }
        i += 1;
    }
    println!("{p1} {p2}");
}
