mod hashmap_and_hashset_problems;
mod sliding_window_algorithm;
mod greedy_problems;
mod dynamic_algorithm_problems;
mod divide_and_conquer_problems;
mod tree_binary_problems;
mod queue_problems;
mod decode_string_problem;
mod binary_trees;
mod linked_list_problems;
mod reverse_words_in_a_string;

use crate::hashmap_and_hashset_problems::*;
use crate::queue_problems::my_queues;
use crate::reverse_words_in_a_string::reverse_words;

fn main() {
    // find_first_missing_positive_int();
    // find_first_unique_char_using_hashmap();
   // my_queues()
   println!("{}", reverse_words(String::from("  hello world  ")));


}

