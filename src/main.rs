use std::rc::Rc;
use std::cell::{RefCell, Ref};

mod circular_list;
use circular_list::*;
mod play_refcell;
use play_refcell::*;
use crate::circular_list::CircularList::{Cons, Nil};
use std::convert::TryInto;

fn main() {
    // let rr = rc::Rc::new(42);
    // {
    //     let rr1 = rc::Rc::downgrade(&rr);
    // }
    // let rrr: i32 = rc::try_unwrap(rr).unwrap();
    // println!("rrr {}", rrr);

    let s = String::from("Valera");
    let c1 = Rc::new(RefCell::new(String::from("Jora")));
    let c2 = Rc::clone(&c1);
    println!("{:?}", c2);
    *c1.borrow_mut() = s;
    println!("{:?}", c2);

    let l1 = Rc::new(RefCell::new(list![1,2,3]));
    let l2 = Rc::clone(&l1);
    println!("L2 before: {}", l2.borrow());
    *l1.borrow_mut() = list![999,888,777];
    println!("L2 after: {}", l2.borrow());

    println!("L1: {}", l1.borrow());
    println!("L1 tail: {}", l1.borrow().tail().unwrap().borrow());

    l1.borrow_mut().del_second();
    println!("L1 after deleting second: {}", l1.borrow());

    let f = FancyInt::new(42);
    println!("{}", f);
    f.set(777);
    println!("{}", f);
}
