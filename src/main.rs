use std::rc::Rc;
use std::cell::{RefCell, Ref};

mod circular_list;
use circular_list::*;
mod play_refcell;
use play_refcell::*;
use crate::circular_list::CircularList::{Cons, Nil};

fn main() {
    let c1 = Rc::new(RefCell::new(String::from("Jora")));
    let c2 = Rc::clone(&c1);
    println!("{:?}", c2);
    *c1.borrow_mut() = String::from("Valera");
    println!("{:?}", c2);

    let l1 = Rc::new(RefCell::new(list![1,2,3]));
    let l2 = Rc::clone(&l1);
    println!("L2 before: {}", l2.borrow());
    *l1.borrow_mut() = list![999,888,777];
    println!("L2 after: {}", l2.borrow());

    println!("L1: {}", l1.borrow());
    println!("L1 tail: {}", l1.borrow().tail().unwrap().borrow());

    //let l = l1.as_ref().borrow_mut();
    //let mut tail = l.tail().unwrap().as_ref().borrow_mut();
    //*tail = Cons(666, Rc::new(RefCell::new(Nil)));
    //drop(tail);
   // let l1 = Rc::clone();
    //println!("L1: {}", l1.as_ref().borrow());
    let f = FancyInt::new(42);
    println!("{}", f);
    f.set(777);
    println!("{}", f);
}
