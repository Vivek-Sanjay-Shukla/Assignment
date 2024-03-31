fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as i32;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}

fn main() {
    let n = 17;

    if is_prime(n) {
        println!("It is a prime number");
    } else {
        println!("It is not a prime number");
    }
}
