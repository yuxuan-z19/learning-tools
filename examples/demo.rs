// 这是一个示例Rustlings练习文件
// 可以用来测试本地评测工具

// 这个练习会通过测试
#[cfg(test)]
mod tests {
    #[test]
    fn test_success() {
        assert_eq!(2 + 2, 4);
    }
}

fn main() {
    println!("Hello, Rustlings!");
}