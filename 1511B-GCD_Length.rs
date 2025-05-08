fn main() {
    fn is_prime(x: u32) -> bool {
        // x >= 3 and is odd
        if x < 9 { return true; }
        for d in (3..=(x as f32).sqrt() as u32).step_by(2) {
            if x % d == 0 { return false; }
        }
        return true;
    }

    struct Primes { start: u32, end: u32 }
    impl Iterator for Primes {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            if self.start > self.end { return None; }
            if self.start < 2 {
                self.start = 2;
                return Some(1);
            }
            if self.start == 2 {
                self.start = 3;
                return Some(2);
            }
            if self.start % 2 == 0 {
                self.start += 1;
            }
            // self.start >= 3 and is odd
            while self.start <= self.end {
                let res = self.start;
                self.start += 2;
                if is_prime(res) { return Some(res); }
            }
            return None;
        }
    }

    for line in std::io::stdin().lines().skip(1).map(|r| r.unwrap()) {
        let [a, b, c] = line
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();
        let [x_min, x_max] = [10u32.pow(a - 1), 10u32.pow(a) - 1];
        let [y_min, y_max] = [10u32.pow(b - 1), 10u32.pow(b) - 1];
        let [gcd_min, gcd_max] = [10u32.pow(c - 1), 10u32.pow(c) - 1];
        let [mut x, mut y] = [0u32, 0u32];
        for gcd in gcd_min..=gcd_max {
            let mut x_div = 0u32;
            for div in (Primes { start: x_min / gcd, end: x_max / gcd }) {
                let x = div * gcd;
                if x >= x_min && x <= x_max {
                    x_div = div;
                    break;
                }
            }
            if x_div == 0 { continue; }

            let mut y_div = 0u32;
            for div in (Primes { start: y_min / gcd, end: y_max / gcd }) {
                if div == x_div { continue; }
                let y = div * gcd;
                if y >= y_min && y <= y_max {
                    y_div = div;
                    break;
                }
            }
            if y_div == 0 { continue; }

            x = x_div * gcd;
            y = y_div * gcd;
            break;
        }
        println!("{x} {y}");
    }
}
