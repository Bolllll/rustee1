use std::any::Any;

pub trait Animal {
    fn make_sound(&self);
    //
    fn as_any(&self) -> &dyn Any;
}

pub struct Cat;

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upcasting() {
        let cat: Cat = Cat;
        let animal: Box<dyn Animal> = Box::new(cat); // Upcasting-安全的
        animal.make_sound(); // 调用父类型的方法

        assert!(true);
    }

    #[test]
    fn downcasting_any() {
        // 如下会报错-说Animal没有downcast_ref
        // let animal: Box<dyn Animal> = Box::new(Cat);
        let animal: Box<dyn Any> = Box::new(Cat);
            
        if let Some(cat) = animal.as_ref().downcast_ref::<Cat>() {
            println!("Animal is a Cat!");
            cat.make_sound();
            assert!(true);
        } else {
            println!("Animal is not a Cat!");
            assert!(false);
        }
    }

    // #[test]
    // fn downcasting_2() {
    //      // Create your struct that implements your trait
    //     let cat = Cat;

    //     // Use the trait to abstract away everything that is not needed
    //     let animal: &dyn Animal = &cat;

    //     // Now there's an edge case that uses the original type..
    //     // How do you change it back?
    //     let cat_2: &Cat = animal;

    //     cat_2.make_sound();
    // }
}
