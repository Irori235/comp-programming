struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut ans: Vec<i32> = Vec::new();
        let mut zero_count = 0;
        for num in nums.iter() {
            if *num == 0 {
                zero_count += 1;
            } else {
                ans.push(*num);
            }
        }

        ans.append(&mut vec![0; zero_count]);
        *nums = ans
    }
}

fn main() {
    let mut nums: Vec<i32> = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);
}
