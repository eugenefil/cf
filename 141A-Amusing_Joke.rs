fn main() {
    let [host, guest, letters] = std::io::stdin().lines().map(|r| r.unwrap())
        .collect::<Vec<String>>()
        .try_into()
        .unwrap();
    let mut counts = std::collections::HashMap::new();
    for c in letters.chars() {
        counts.entry(c).and_modify(|n| *n += 1).or_insert(1u8);
    }
    for word in [host, guest] {
        for c in word.chars() {
            use std::collections::hash_map::Entry;
            if let Entry::Occupied(mut o) = counts.entry(c) {
                *o.get_mut() -= 1;
                if *o.get() == 0 {
                    o.remove();
                }
            } else {
                println!("NO");
                return;
            }
        }
    }
    if counts.len() > 0 {
        println!("NO");
        return;
    }
    println!("YES");
}
