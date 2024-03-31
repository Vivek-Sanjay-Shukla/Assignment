fn main() {
    let v1 = vec![-2, -3, -4, 1, 4, 6, 0, -1, -2];
    let mut msf = std::i32::MIN;
    let mut meh = 0;

    for &num in &v1 {
        meh = meh + num;
        if msf < meh {
            msf = meh;
        }

        if meh < 0 {
            meh = 0;
        }
    }

    println!("Maximum subarray sum is: {}", msf);
}
