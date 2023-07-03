// ref: https://ysantos.com/blog/downcast-rust

trait Value {
    fn as_int(&self) -> Option<isize> { None }
    fn as_text(&self) -> Option<&str> { None }
}

impl Value for isize {
    fn as_int(&self) -> Option<isize> { Some(*self) }
}

impl Value for String {
    fn as_text(&self) -> Option<&str> { Some(self) }
}

fn print_value<T: Value>(val: T) {
    if let Some(num) = val.as_int() {
        println!("A number w/ {} ones in binary", num.count_ones());
    }
    if let Some(string) = val.as_text() {
        println!("A string as bytes: {:?}", string.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
fn test_value() {
    print_value(110);
    print_value(String::from("hello"));
}
}