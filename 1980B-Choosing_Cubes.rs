fn main() {
    let mut lines = std::io::stdin().lines().skip(1).map(|r| r.unwrap());
    while let Some(line) = lines.next() {
        let [n, f, k] = line
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        let mut nums = lines.next().unwrap()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u8>>();
        if k == n {
            println!("YES");
            continue;
        }
        let fav = nums[f - 1];
        nums.sort_by_key(|&n| std::cmp::Reverse(n));
        let kth = nums[k - 1];
        let next = nums[k];
        if kth < fav {
            println!("YES");
        } else if kth > fav {
            println!("NO");
        } else if kth == next {
            println!("MAYBE");
        } else {
            println!("YES");
        }
    }
}
