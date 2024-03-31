fn solve(s: &str) -> String {
    let mut cnt = 0;
    let mut shortest_length = std::i32::MAX;

    for c in s.chars() {
        if c == ' ' {
            shortest_length = shortest_length.min(cnt);
            cnt = 0;
        } else {
            cnt += 1;
        }
    }

    shortest_length = shortest_length.min(cnt);
    cnt = 0;
    let mut ans = String::new();

    for c in s.chars() {
        if c == ' ' {
            if cnt == shortest_length {
                return ans;
            }
            ans.clear();
            cnt = 0;
        } else {
            ans.push(c);
            cnt += 1;
        }
    }

    ans
}

fn main() {
    let s = "This is a string";

    let ans = solve(s);

    println!("Shortest word is: {}", ans);
}
