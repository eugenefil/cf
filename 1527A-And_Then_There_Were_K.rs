fn main() {
    for n in std::io::stdin().lines().skip(1)
        .map(|r| r.unwrap().parse::<u32>().unwrap())
    {
        println!("{}", 1u32.rotate_right(n.leading_zeros() + 1) - 1);
    }
}
