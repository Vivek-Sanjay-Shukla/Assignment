
fn is_palindrome(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    return true;
}

fn main(){

	let s = "abcba";
	
	if is_palindrome(s) {
	    println!("{} is a palindrome", s);
	}else {
	    println!("{} is not a palindrome", s);
	}

	
}
