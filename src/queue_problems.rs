//expand string problem
//https://leetcode.com/problems/decode-string/description/
//Input: s = "3[a]2[bc]"
// Output: "aaabcbc"

extern crate queue;
use queue::*;


pub fn my_queues(){
    let mut q:Queue<u32>=Queue::new();
    q.queue(3).unwrap();
    q.queue(4).unwrap();

    println!("{}",q.len());
    println!("{}",repeat_string("aa",2))
}

fn repeat_string(raw_string:&str,multiplier:u32) -> String{
    let mut result=String::from("");
    for _number in 0..multiplier{
        result.push_str(raw_string);
    }
    result
}