use std::rc::Rc;
use std::cell::RefCell;

mod circular_list;
use circular_list::*;

fn main() {
    let c1 = Rc::new(RefCell::new(String::from("Jora")));
    let c2 = Rc::clone(&c1);
    println!("{:?}", c2);
    *c1.borrow_mut() = String::from("Valera");
    println!("{:?}", c2);

    let l1 = Rc::new(RefCell::new(list![1,2,3]));
    let l2 = Rc::clone(&l1);
    println!("L2 before: {}", l2.as_ref().borrow());
    *l1.borrow_mut() = list![42];
    println!("L2 after: {}", l2.as_ref().borrow());
}
