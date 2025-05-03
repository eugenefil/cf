fn main() {
    let mut lines = std::io::stdin().lines().skip(1).map(|r| r.unwrap());
    while let Some(line) = lines.next() {
        let k = line
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u32>>()[1];
        let mut nums = lines.next().unwrap()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u32>>();
        nums.sort();
        let n = nums.len();
        let mut maxlen = 1usize;
        while nums.len() > maxlen {
            let mut balanced = true;
            for i in 0..nums.len() - 1 {
                if nums[i].abs_diff(nums[i + 1]) > k {
                    let left_len = i + 1;
                    maxlen = std::cmp::max(maxlen, left_len);
                    nums.drain(..i + 1);
                    balanced = false;
                    break;
                }
            }
            if balanced {
                maxlen = nums.len();
            }
        }
        println!("{}", n - maxlen);
    }
}
