fn main() {
    let mut breaks = 0u32;
    std::io::stdin().lines().skip(1).map(|r| r.unwrap()).reduce(|prev, cur| {
        if cur != prev {
            breaks += 1;
        }
        cur
    });
    println!("{}", breaks + 1);
}
