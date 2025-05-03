fn main() {
    for n in std::io::stdin().lines().skip(1)
        .map(|r| r.unwrap().parse::<usize>().unwrap())
    {
        if n == 1 {
            println!("1");
        } else if n == 2 {
            println!("-1");
        } else {
            let odd = (1..=n * n).step_by(2);
            let even = (2..=n * n).step_by(2);
            let mut row = vec![];
            for x in odd.chain(even) {
                row.push(x);
                if row.len() == n {
                    println!(
                        "{}",
                        row
                            .iter()
                            .map(usize::to_string)
                            .collect::<Vec<String>>()
                            .join(" ")
                    );
                    row.clear();
                }
            }
        }
    }
}
