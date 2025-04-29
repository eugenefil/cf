fn main() {
    let line = std::io::stdin().lines().skip(1).next().unwrap().unwrap();
    let mut heights = line
        .split(' ')
        .map(|s| s.parse::<u8>().unwrap())
        .enumerate();
    let (mut imin, mut min) = heights.next().unwrap();
    let (mut imax, mut max) = (imin, min);
    let mut last = imin;
    for (i, h) in heights {
        if h > max {
            max = h;
            imax = i;
        }
        if h <= min { // also take one which is equal but more to the right
            min = h;
            imin = i;
        }
        last = i;
    }
    let move_max = imax - 0;
    let mut move_min = last - imin;
    if imax > imin {
        // if max was to the right of min, min was displaced by 1 position when moving
        // max to the left and so became 1 position closer to the right; subtract that
        move_min -= 1;
    }
    println!("{}", move_max + move_min);
}
