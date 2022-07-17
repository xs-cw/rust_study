mod leetcode;
use leetcode::Solution;


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