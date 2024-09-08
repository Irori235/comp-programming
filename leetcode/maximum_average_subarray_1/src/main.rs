
struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let max_val = nums
            .windows(k as usize)
            .map(|arr| -> i32 { arr.iter().sum() })
            .max()
            .unwrap_or(0);

        max_val as f64 / k as f64
    }
}

fn main() {
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 4;
    let result = Solution::find_max_average(nums, k);
    println!("The maximum average is {}", result);
}
