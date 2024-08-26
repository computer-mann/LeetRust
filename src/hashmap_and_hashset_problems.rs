use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use indexmap::IndexMap;

pub fn find_first_unique_char_using_hashmap() -> char{
    let the_word="bloomberg";
    let mut  map:HashMap<char,u32>=HashMap::new();
    //the_word.chars().into_iter().for_each()
    for each_char in the_word.chars(){
        let count=map.entry(each_char).or_insert(0);
        *count+=1;
    }
    let mut result:char=' ';
    for each_char in the_word.chars(){
        if map.get(&each_char).is_some_and(|x| *x == 1){
            println!("the first char is {} every time",each_char);
            result=each_char;
            break;
        }
    }
    result
}

//first missing positive in the string
pub fn find_first_missing_positive_int() -> i32 {

    let array=vec![0,-1,2,1,3,4,5,6,-6,-1,-7];
    let mut set:HashSet<i32> = HashSet::new();
    array.clone().into_iter().for_each(|x| {
        if x >= 0 {
            set.insert(x);
        }
    });
    let mut result:i32 = 0;
    for number in 0..i32::MAX {
        if !set.contains(&number){
            println!("the first missing positve is {}",&number);
            result= number;
            break;
        }
    }
    return result;

}

pub fn find_first_unique_char_using_index_map() -> char{
    let the_word="bloomberg";
    let mut char_count_map:IndexMap<char,u32>=IndexMap::new();
    for each_char in the_word.chars(){
        let count= char_count_map.entry(each_char).or_insert(0);
        *count+=1;
    }
    let char_result = char_count_map.into_iter()
                    .filter_map(|(key,value)| if value == 1 {
                        Some(key)
                    }else { None }).nth(0);

    println!("{:?}",&char_result.unwrap());
    char_result.unwrap()
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    pub fn index_map_unique(){
        assert_eq!(find_first_unique_char_using_index_map(),'l')
    }

    #[test]
    pub fn first_missing_possitive_is_seven(){
        assert_eq!(find_first_missing_positive_int(),7)
    }

    #[test]
    pub fn find_first_unique(){
        assert_eq!(find_first_unique_char_using_hashmap(),'l')
    }


}
