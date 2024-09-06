// https://leetcode.com/problems/reverse-words-in-a-string/description/?envType=study-plan-v2&envId=leetcode-75

pub fn reverse_words(s: String) -> String {
    if(s.is_empty()){
        panic!("Empty string")
    }
    //get the real words from the front, put them in a stack, then return them with spaces between them
    //let stack:Vec<String>=Vec::new();
    let mut backwards=String::new();
    let individual_words=s.split(" ").collect::<Vec<_>>();
    for x in (0..individual_words.iter().len()).rev(){
        if(!individual_words[x].is_empty()){
            backwards.push_str(individual_words[x]);
            backwards.push_str(" ");
        }

    }
    backwards.trim().to_string()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    pub fn test_reverse_words(){
        assert_eq!(reverse_words(String::from( "the sky is blue")) ,String::from("blue is sky the"));
        assert_eq!(reverse_words(String::from("  hello world  ")),String::from("world hello"));
        assert_eq!(reverse_words(String::from("a good   example")), String::from("example good a"))
    }

    #[test]
    #[should_panic]
    pub fn test_reverse_words_should_panic_when_empty_string_is_passed(){
        assert_eq!(reverse_words(String::from("")),String::from(""));
    }
}