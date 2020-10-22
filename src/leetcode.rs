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
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let (name, typed) = (name.into_bytes(), typed.into_bytes());
        let mut i = 0;
        for c in typed {
            if i >= name.len(){
                i = name.len() -1;
            };
            if c == name[i] {
                i += 1;
                continue;
            } else {
                if i == 0 {
                    return false;
                }
                if c == name[i - 1] {
                    continue;
                } else {
                    return false;
                }
            }
        }
        i == name.len()
    }
}
fn main() {
    print!(
        "{}",
        Solution::find_repeat_number(vec![1, 2, 3, 5, 2, 4, 1])
    );
}
