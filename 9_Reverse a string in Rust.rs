fn main() {
    let mut s = String::from("Hello");
    let n = s.len();

    for i in 0..n / 2 {
        let other_index = n - i - 1;
        let temp = s.chars().nth(i).unwrap();
        s.replace_range(i..=i, &s.chars().nth(other_index).unwrap().to_string());
        s.replace_range(other_index..=other_index, &temp.to_string());
    }

    println!("The reversed string is: {}", s);
}
