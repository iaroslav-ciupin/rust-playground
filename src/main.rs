use std::rc::Rc;
use std::cell::RefCell;

mod circular_list;
use circular_list::*;
mod play_refcell;
use play_refcell::*;

fn main() {
    let mut l1 = Rc::new(RefCell::new(list![1,2,3]));
    let mut l2 = Rc::clone(&l1);

    println!("l1: {}, count l1: {}", l1.borrow(), Rc::strong_count(&l1));
    println!("l2: {}, count l2: {}", l2.borrow(), Rc::strong_count(&l2));
    println!();

    println!("deleting l1 first");
    CircularList::del(&mut l1, 0);

    println!();
    println!("l1: {}, count l1: {}", l1.borrow(), Rc::strong_count(&l1));
    println!("l2: {}, count l2: {}", l2.borrow(), Rc::strong_count(&l2));

    l2.borrow_mut().append(42);
    println!("l2: {}, count l2: {}", l2.borrow(), Rc::strong_count(&l2));
}
