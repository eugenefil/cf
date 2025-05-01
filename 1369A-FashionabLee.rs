fn main() {
    for n in std::io::stdin().lines().skip(1)
        .map(|r| r.unwrap().parse::<u32>().unwrap())
    {
        println!("{}", if n % 4 == 0 { "YES" } else { "NO" });
    }
}
