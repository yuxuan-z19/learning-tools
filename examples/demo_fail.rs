// 这是一个示例Rustlings练习文件
// 这个练习会失败，用于测试本地评测工具的失败处理

// 这个测试会失败
#[cfg(test)]
mod tests {
    #[test]
    fn test_failure() {
        assert_eq!(2 + 2, 5, "这个测试应该失败");
    }
}

fn main() {
    println!("这个练习会失败！");
}