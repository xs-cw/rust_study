struct Solution {}
impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        let mut tmp = nums[0];
        let mut t = tmp;
        loop {
            if nums[tmp as usize] == tmp {
                return tmp;
            } else {
                t = tmp;
                tmp = nums[tmp as usize];
                nums[t as usize] = t;
            }
        }
    }
}
fn main() {
    print!(
        "{}",
        Solution::find_repeat_number(vec![1, 2, 3, 5, 2, 4, 1])
    );
}
