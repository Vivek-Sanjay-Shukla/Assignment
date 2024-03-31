
fn search(nums: &[i32], target: i32) -> Option<usize> {
    let mut ans = None;
    let mut start = 0;
    let mut end = nums.len() - 1;

    while start <= end {
        let mid = start + (end - start) / 2;

        if target < nums[mid] {
            end = mid - 1;
        } else if target > nums[mid] {
            start = mid + 1;
        } else {
            ans = Some(mid);
            end = mid - 1;
        }
    }

    ans
}

fn main() {
    let arr = [1, 2, 2, 3, 3, 4, 7, 8];
    let x = 3;

    match search(&arr, x) {
        Some(index) => println!("First Occurrence: {}", index),
        None => println!("Element not found"),
    }
}
