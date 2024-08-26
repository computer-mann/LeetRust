//expand string problem
//https://leetcode.com/problems/decode-string/description/
//Input: s = "3[a]2[bc]"
// Output: "aaabcbc"

extern crate queue;
use queue::*;

pub fn decode(){

}

pub fn decode_string(){

}


pub fn repeat_string(raw_string:&str,multiplier:u32) -> String{
    let mut result=String::from("");
    for _number in 0..multiplier{
        result.push_str(raw_string);
    }
    result
}