fn main() {
    let n: u32 = std::io::stdin().lines().next().unwrap().unwrap().parse().unwrap();
    let mut a1 = n / 2;
    let mut a2 = a1;
    if n % 2 > 0 {
        a2 += 1;
    }
    fn is_prime(x: u32) -> bool {
        if x % 2 == 0 { return false; }
        for d in (3..=(x as f32).sqrt() as u32).step_by(2) {
            if x % d == 0 { return false; }
        }
        return true;
    }
    while a1 > 3 && (is_prime(a1) || is_prime(a2)) {
        a1 -= 1;
        a2 += 1;
    }
    println!("{a1} {a2}");
}
