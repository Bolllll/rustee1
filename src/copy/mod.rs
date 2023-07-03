mod animal;
use animal::*;

mod shape;
use shape::*;

mod value;
use value::*;

pub mod mytrait;
use mytrait::*;

// 演示copy/clone/swap 等
// std::mem::swap 是目前系统的标准实现
fn is_copy<T: Copy>() {}
fn is_clone<T: Clone>() {}

// 适用于primitive类型以及由primitive类型构成的struct
// 要求实现Copy trait
pub fn swap<T: Copy>(a: &mut T, b: &mut T) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

// 适用于实现了clone的struct
pub fn swap_clone<T: Clone>(a: &mut T, b: &mut T) {
    let tmp = a.clone();
    *a = b.clone();
    *b = tmp;
}

// fn swap_std<T>(x: &mut T, y: &mut T) {
//     let temp = std::mem::take(x);
//     *x = std::mem::take(y);
//     *y = temp;
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_swap() {
        let mut a = 1;
        let mut b = 2;
        swap(&mut a, &mut b);
        assert_eq!(a, 2);
        assert_eq!(b, 1);

        swap_clone(&mut a, &mut b);
        assert_eq!(a, 1);
        assert_eq!(b, 2);

        std::mem::swap(&mut a, &mut b);
        assert_eq!(a, 2);
        assert_eq!(b, 1);
    }

    #[test]
    fn test_swap_std_clone() {       
        let mut a = Person {
            name: String::from("Alice"),
            age: 20,
        };
        let mut b = Person {
            name: String::from("Bob"),
            age: 22,
        };

        std::mem::swap(&mut a, &mut b);
        assert_eq!(a.age, 22);
        assert_eq!(b.age, 20);
        assert_eq!(a.name, "Bob");

        // 要求实现 PartialEq，可以用等号判断
        assert_ne!(a, b);
        assert_eq!(a == b, false);

        let bob = Person{name: String::from("Bob"), age: 22};
        assert_eq!(a, bob);
        assert_eq!(a == bob, true);
    }

    #[test]
    fn test_swap_std_copy() {
        #[derive(Debug, Copy, Clone)]
        struct Point {
            x: f64,
            y: f64,
        }

        let mut a = Point {
            x: 1.0,
            y: 2.0,
        };
        let mut b = Point {
            x: 3.0,
            y: 4.0,
        };
        std::mem::swap(&mut a, &mut b);
        assert_eq!(a.x, 3.0);
        assert_eq!(b.y, 2.0);
    }

    #[test]
    fn test_is_copy_clone() {
        is_copy::<u32>();
        is_clone::<u32>();
        // is_copy::<String>(); // can't compile
        is_clone::<String>();
    }

    #[test]
    fn test_is_type() {
        use std::any::{Any, TypeId};

        let val: u8 = 10;
        println!("Type ID of u8: {:?}", val.type_id());
        assert_eq!(val.type_id(), TypeId::of::<u8>());

        let x = &true as &dyn Any;
        assert!(x.is::<bool>());
    }
}