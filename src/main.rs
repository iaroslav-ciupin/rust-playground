use std::rc::Rc;
use std::cell::{RefCell, Ref};

mod circular_list;
use circular_list::*;
use crate::circular_list::CircularList::{Cons, Nil};

fn main() {
    let c1 = Rc::new(RefCell::new(String::from("Jora")));
    let c2 = Rc::clone(&c1);
    println!("{:?}", c2);
    *c1.borrow_mut() = String::from("Valera");
    println!("{:?}", c2);

    let l1 = Rc::new(RefCell::new(list![1,2,3]));
    let l2 = Rc::clone(&l1);
    println!("L2 before: {}", l2.as_ref().borrow());
    *l1.borrow_mut() = list![999,888,777];
    println!("L2 after: {}", l2.as_ref().borrow());

    println!("L1: {}", l1.as_ref().borrow());
    println!("L1 tail: {}", l1.as_ref().borrow().tail().unwrap().as_ref().borrow());

    //let l = l1.as_ref().borrow_mut();
    //let mut tail = l.tail().unwrap().as_ref().borrow_mut();
    //*tail = Cons(666, Rc::new(RefCell::new(Nil)));
    //drop(tail);
   // let l1 = Rc::clone();
    //println!("L1: {}", l1.as_ref().borrow());
}
