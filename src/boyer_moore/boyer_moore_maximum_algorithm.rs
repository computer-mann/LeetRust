//https://leetcode.com/problems/majority-element/description/
pub mod boyer_moore_maximum_algorithm {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests{
    use crate::boyer_moore::boyer_moore_maximum_algorithm::boyer_moore_maximum_algorithm::majority_element;
    use super::*;

    #[test]
    pub fn should_return_3(){
        assert_eq!(majority_element(Vec::new()),3);
    }
}