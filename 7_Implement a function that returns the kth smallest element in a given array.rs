fn main() {
    let v = vec![7, 8, 5, 1, 5, 23, 6];
    let k = 3;

    let mut maxi = v[0];
    for &num in &v[1..] {
        maxi = maxi.max(num);
    }

    let mut mp = vec![0; maxi as usize + 1];
    for &num in &v {
        mp[num as usize] += 1;
    }

    let mut cnt = 0;
    for (i, &count) in mp.iter().enumerate() {
        if count != 0 {
            cnt += count;
            if cnt >= k {
                println!("The kth smallest element is: {}", i);
                break;
            }
        }
    }
}
