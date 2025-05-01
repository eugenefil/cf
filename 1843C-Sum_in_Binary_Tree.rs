fn main() {
    for mut n in std::io::stdin().lines().skip(1)
        .map(|r| r.unwrap().parse::<u64>().unwrap())
    {
        let mut sum = 0u64;
        while n > 0 {
            sum += n;
            n /= 2;
        }
        println!("{sum}");
    }
}
