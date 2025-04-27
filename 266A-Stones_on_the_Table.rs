fn main() {
    let mut n = 0;
    // use fold's accumulator to store prev char, init accumulator with null char (0)
    std::io::stdin().lines().skip(1).next().unwrap().unwrap().bytes().fold(0, |prev, c| {
        if c == prev { n += 1; }
        c
    });
    println!("{n}");
}

/* pure procedural solution
fn main() {
    let mut prev = 0u8;
    let mut n = 0;
    for c in std::io::stdin().lines().skip(1).next().unwrap().unwrap().bytes() {
        if c != prev {
            prev = c;
        } else {
            n += 1;
        }
    }
    println!("{n}");
}
*/
