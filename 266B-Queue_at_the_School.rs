fn main() {
    let mut lines = std::io::stdin().lines().map(|r| r.unwrap());
    let k = lines.next().unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u8>>()[1];
    let mut q: Vec<char> = lines.next().unwrap().chars().collect();
    for _ in 0..k {
        let mut i = 0usize;
        while i < q.len() - 1 {
            if q[i] == 'B' && q[i + 1] == 'G' {
                q[i] = 'G';
                q[i + 1] = 'B';
                i += 2;
                continue;
            }
            i += 1;
        }
    }
    println!("{}", q.iter().collect::<String>());
}
