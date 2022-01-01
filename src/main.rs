// 如果想在main.rs中调用solution模块：https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs
use leetcode_in_rust::solution::n0035_search_insert_position::Solution;
// use leetcode_in_rust::data_structures::linked_list;
fn main() {
    println!("Hello, 这里是zyleo学习rust的leetcode刷题库");
    for i in 0..10 {
        println!(
            "{} ==> {:?}",
            i,
            Solution::search_insert(vec![1, 2, 5, 6, 9], i)
        );
    }
}
