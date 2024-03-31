fn main() {
    let v1 = vec![1, 3, 5];
    let v2 = vec![2, 4, 6];
    let mut i = 0;
    let mut j = 0;
    let mut ans = Vec::new();

    while i < v1.len() && j < v2.len() {
        if v1[i] < v2[j] {
            ans.push(v1[i]);
            i += 1;
        } else {
            ans.push(v2[j]);
            j += 1;
        }
    }

    while i < v1.len() {
        ans.push(v1[i]);
        i += 1;
    }

    while j < v2.len() {
        ans.push(v2[j]);
        j += 1;
    }

    print!("The resultant array is: ");
    for num in &ans {
        print!("{} ", num);
    }
    println!();
}
