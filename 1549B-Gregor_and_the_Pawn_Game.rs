fn main() {
    let mut lines = std::io::stdin().lines().skip(1).map(|r| r.unwrap());
    while let Some(_) = lines.next() {
        let mut theirs = lines.next().unwrap().chars().collect::<Vec<char>>();
        let ours = lines.next().unwrap();
        let mut k = 0u32;
        for (i, o) in ours.chars().enumerate() {
            if o == '0' { continue; }
            if theirs[i] == '0' {
                k += 1;
            } else if i > 0 && theirs[i - 1] == '1' {
                k += 1;
            } else if i + 1 < theirs.len() && theirs[i + 1] == '1' {
                k += 1;
                theirs[i + 1] = '0';
            }
        }
        println!("{k}");
    }
}
