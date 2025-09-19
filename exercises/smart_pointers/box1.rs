#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>), // Step 1: 用 Box 包裹递归部分
    Nil,
}

impl List {
    // 辅助构造函数，用起来更顺手
    pub fn cons(val: i32, next: List) -> Self {
        List::Cons(val, Box::new(next))
    }
}

pub fn create_empty_list() -> List {
    List::Nil // Step 2: 空列表
}

pub fn create_non_empty_list() -> List {
    List::cons(42, List::Nil) // Step 2: 非空列表
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!("This is a non-empty cons list: {:?}", create_non_empty_list());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}