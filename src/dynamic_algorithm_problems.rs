
pub fn climbing_stairs(n:u32)-> u32{
    let mut pre=1;
    let mut curr=1;
    let mut old_curr=0;
    for i in 1..n{
        old_curr=curr;
        curr=curr+pre;
        pre=old_curr;
    }
    curr
}

#[cfg(test)]
mod climbing_tests{

    use super::*;

    #[test]
    pub fn climbing_stairs_tests(){
        assert_eq!(climbing_stairs(2),2);
        assert_eq!(climbing_stairs(3),3);
        assert_eq!(climbing_stairs(4),5)
    }
}