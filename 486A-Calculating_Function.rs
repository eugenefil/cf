fn main() {
    let n: i64 = std::io::stdin().lines().next().unwrap().unwrap().parse().unwrap();
    println!("{}", n / 2 - n * (n % 2));
}
