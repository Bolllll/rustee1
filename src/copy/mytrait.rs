// https://ysantos.com/blog/downcast-rust
// 不需要 Any
pub trait Trait {}

impl Trait for String {}
impl Trait for u8 {}

impl dyn Trait {
    // SAFETY: I hope you know what you're doing
    // 1- 虽然这个本身不是定义在Trait上，但本质是一回事
    // 2- 下面的转换看不明白
    pub unsafe fn downcast<T>(&self) -> &T {
        &*(self as *const dyn Trait as *const T)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
fn test_traits() {
    let a: &dyn Trait = &42_u8;
    let b: &dyn Trait = &String::from("hello");

    let _number: u8 = *unsafe { a.downcast::<u8>() };
    let _text: &str = unsafe { b.downcast::<String>() };
}
}