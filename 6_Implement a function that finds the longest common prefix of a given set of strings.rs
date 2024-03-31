fn main() {
    let mut v = vec!["Hello", "Hell", "Hel", "Helldg"];
    let n = v.len();

    if n == 1 {
        println!("The longest common prefix is: {}", v[0]);
    } else {
        v.sort();

        let ind = std::cmp::min(v[0].len(), v[n - 1].len());

        let (s1, s2) = (v[0], v[n - 1]);

        let mut i = 0;
        while i < ind && s1.as_bytes()[i] == s2.as_bytes()[i] {
            i += 1;
        }

        let ans = &s1[..i];

        println!("The longest common prefix is: {}", ans);
    }
}
