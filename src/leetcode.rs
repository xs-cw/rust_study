use rocket::http::ext::IntoCollection;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    // 重复数字
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
    //
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let (name, typed) = (name.into_bytes(), typed.into_bytes());
        let mut i = 0;
        for c in typed {
            if i >= name.len() {
                i = name.len() - 1;
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
    // 只出现一次的数字 II
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut hs = HashMap::new();
        for n in nums {
            let v = hs.entry(n).or_insert(0);
            *v += 1;
            if *v == 3 {
                hs.remove(&n);
                continue;
            }
        }
        hs.keys().fold(0, |_i, x| *x)
        // for n in hs.keys() {
        //     return *n;
        // }
        // 0
    }
    // 只出现一次的数字 II
    pub fn single_num(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |(once, twice), num| {
                (
                    (once | twice | num) & !(once & twice & num),
                    once & (twice ^ num),
                )
            })
            .0
    }
    // 反推补码
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut tmp = first;
        res.reserve(encoded.len() + 1);
        res.push(first);
        for i in encoded {
            tmp ^= i;
            res.push(tmp);
        }
        res
    }
}


fn main() {
    let a = [6, 2, 7, 3];
    println!("{:?}", Solution::decode(Vec::from(a), 4));
    // the sum of all of the elements of the array
    let sum = a.iter().fold(0, |acc, &x| {
        println!("{},{}", acc, x);
        acc + x
    });
    println!("{}", sum);
    print!(
        "{}",
        Solution::single_number(vec![111, 2, 3, 3, 3, 2, 2])
    );
}
