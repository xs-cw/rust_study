use std::collections::HashMap;


pub struct Solution {}

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
    pub fn z_change(s: String, num_rows: i32) -> String {
        let len = s.chars().count() as i32;
        let s = s.as_bytes();
        let mut res = Vec::with_capacity(num_rows as usize);
        for r in 0..num_rows {
            let mut i = r;
            loop {
                if i >= len {
                    break;
                }
                res.push(s[i as usize]);
                if r != num_rows - 1 && r != 0 {
                    let next = i + (num_rows - 1) * 2 - 2 * r;
                    if next >= len {
                        break;
                    }
                    res.push(s[next as usize]);
                }
                i += (num_rows - 1) * 2;
            }
        }
        String::from_utf8(res).unwrap()
    }
}
