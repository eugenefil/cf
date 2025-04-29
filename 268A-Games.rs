fn main() {
    let teams = std::io::stdin().lines().skip(1)
        .map(|r| r.unwrap()
                    .split(' ')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap())
        .collect::<Vec<[u8; 2]>>();
    let mut res = 0u16;
    for (i, [h, _]) in teams.iter().enumerate() {
        for (j, [_, a]) in teams.iter().enumerate() {
            if i == j {
                continue;
            }
            if h == a {
                res += 1;
            }
        }
    }
    println!("{res}");
}
