fn main() {
    for line in std::io::stdin().lines().skip(1).map(|r| r.unwrap()) {
        let mut n = 0u32;
        for d in line.split(' ').map(|s| s.parse::<u32>().unwrap()) {
            // doubled digit will xor itself out
            n ^= 1u32.rotate_left(d);
        }
        println!("{}", n.trailing_zeros());
    }
}
