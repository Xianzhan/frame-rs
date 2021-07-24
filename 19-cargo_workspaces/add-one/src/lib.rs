//! # add-one
//!
//! crate lib 描述
//! `add-one` 提供一个方法来进行加一操作

mod internal;

/// 重新导出
/// 减少路径
pub use internal::add_two;

/// rust 文档描述
/// 参数 x + 1
/// # Examples
/// ```
/// let five = 5;
/// assert_eq!(6, add_one::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        assert_eq!(3, add_one(2));
    }
}
