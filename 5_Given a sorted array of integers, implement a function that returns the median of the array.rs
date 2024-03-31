fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7];
    let n = v.len();

    if n % 2 == 0 {
        println!("The median is: {}", (v[n / 2] + v[n / 2 - 1]) / 2);
    } else {
        println!("The median is: {}", v[n / 2]);
    }
}
