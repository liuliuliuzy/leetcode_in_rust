// 在这里声明 data_structures 模块，Rust 才能识别到 data_structure/mod.rs 中的定义
// what's more
// #[macro_use] 导出data_structures 中的宏，这样就能够在测试代码中直接使用宏
// 这里的意思相当于是把data_structures中的所有宏引入到了当前作用域
// 所以在 main.rs 中可以直接通过 use leetcode_in_rust::tree; 的方式来引入宏。
// 同理，也可以在当前模块的任何子模块中使用tree宏，比如solution模块
#[macro_use]
pub mod data_structures;

// 而且还要保证solution在 #[macro_use] 的后面
pub mod solution;
