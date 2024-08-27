
struct BinaryTree<'a>{
    pub value: u32,
    pub left:&'a BinaryTree<'a>,
    pub right:&'a BinaryTree<'a>
}

// impl BinaryTree{
//     pub fn new(root:u32,left:u32,right:u32)-> BinaryTree{
//         BinaryTree{
//             value:root,
//             left: BinaryTree {},
//             right: BinaryTree {},
//         }
//     }
// }

pub fn is_valid_bst(tree:BinaryTree)-> bool{
    false
}

#[cfg(test)]
mod tree_tests{
    use super::*;
    #[test]
    fn test_is_valid_bst(){
        assert!(true);
    }

}