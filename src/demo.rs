use rand::Rng;
use std::{cmp::Ordering, io, thread, time::Duration};

#[allow(dead_code)]
async fn jon() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}
#[allow(dead_code)]
async fn learn_and_sing() {
    learn().await;
    sing().await;
}
#[allow(dead_code)]
async fn learn() {
    println!("learn sleeping");
    thread::sleep(Duration::from_secs(2 as u64));
    println!("i am learning");
}
#[allow(dead_code)]
async fn sing() {
    println!("singing sleeping");
    thread::sleep(Duration::from_secs(2 as u64));
    println!("i am singing");
    thread::sleep(Duration::from_secs(3 as u64));
    println!("sing down");
}
#[allow(dead_code)]
async fn dance() {
    println!("dance");
}

#[allow(dead_code)]
fn z_change(s: String, num_rows: i32) -> String {
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

#[allow(dead_code)]
fn ad_str(s1: &str, s2: &str) -> String {
    let mut res = String::from(s1);
    res.push_str(s2);
    return res;
}
#[allow(dead_code)]
fn guess_number() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("origin {}", secret_number);
    loop {
        let mut guess = String::new();
        println!("input your number");
        io::stdin().read_line(&mut guess).expect("error");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        let guess = &guess;
        let a = guess;
        println!("your guess {}", guess);
        println!("your guess {}", guess);
        println!("your guess {}", a);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("winner");
                break;
            }
            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("too small"),
        };
    }
}
