use std::cell::RefCell;
use std::fmt;

#[derive(Debug)]
pub struct FancyInt {
    internal: RefCell<i32>
}

impl fmt::Display for FancyInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.internal.borrow())
    }
}

impl Drop for FancyInt {
    fn drop(&mut self) {
        println!("Dropping {:?}", self)
    }
}

impl FancyInt {
    pub fn new(i: i32) -> FancyInt {
        FancyInt {
            internal: RefCell::new(i)
        }
    }

    pub fn get(&self) -> i32 {
        *self.internal.borrow()
    }

    pub fn set(&self, new: i32) {
        *self.internal.borrow_mut() = new;
    }
}
