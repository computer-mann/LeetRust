
extern crate queue;
use queue::*;
use crate::decode_string_problem::repeat_string;

pub fn my_queues(){
    let mut q:Queue<u32>=Queue::new();
    q.queue(3).unwrap();
    q.queue(4).unwrap();

    println!("{}",q.len());
    println!("{}",repeat_string("aa",2))
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    //#[should_panic]
    pub fn test_no_panic(){
        assert!(true)
    }
}