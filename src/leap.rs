// implements leap.rs as a lib, and it in the top folder
// it can be also in leap/mod.rs, they are the same.
// Notice: if want to test thi, we need import to lib.rs or main.rs
// if we don't import, the code will not run.
pub fn is_leap(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    //(year % 4 == 0) && !(year % 100 == 0 && year % 400 != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap() {
        assert_eq!(is_leap(2000), true);
        assert_eq!(is_leap(2004), true);
        assert_eq!(is_leap(2001), false);
        assert_eq!(is_leap(2100), false);
    }
}