fn main() {
    let mut h = std::collections::HashMap::new();
    for (i, p) in std::io::stdin().lines().skip(1).next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .enumerate()
    {
        h.insert(p, i + 1);
    }
    println!(
        "{}",
        (1..=h.len()).map(|p| h[&p].to_string()).collect::<Vec<String>>().join(" ")
    );
}
