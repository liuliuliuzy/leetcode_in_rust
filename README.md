# leetcode in rust

以刷leetcode的方式学习并熟悉Rust的基本语法。

模板来源：https://github.com/aylei/leetcode-rust

## 编写规范

每个题对应一个文件，文件名为`nxxxx_problem_name.rs`，位于`./src/solution`目录下，`xxxx`填写题号。

比如，做第一个题目：两数之和。那么就在`./src/solution`目录下创建`n0001_two_sum.rs`。

每个题目的文件中应当包含解法与测试函数。测试函数命名为`test_xxxx`，同理，`xxxx`为题号。测试时，在项目主目录运行：

```bash
cargo test test_xxxx
```

## 其它

`./src/main.rs`中可以模仿模板中的`main.rs`，尝试着自己写一些功能函数。