use crate::circular_list::Node::{Cons, Nil};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::ops::Deref;

#[derive(Debug)]
pub enum Node {
    Nil(RefCell<Weak<RefCell<Node>>>),
    Cons(i32, Rc<RefCell<Node>>)
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Nil(_) => if let Nil(_) = other {
                true
            } else {
                false
            },
            Cons(i, tail) => match other {
                Nil(_) => false,
                Cons(j, other_tail) =>
                    i == j && tail.borrow().eq(&other_tail.borrow())
            }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Nil(cycle) => match cycle.borrow().upgrade() {
                Some(head) => write!(f, "⟲"),
                _ => write!(f, "⏚")
            },
            Cons(i, next) => {
                let tail = next.borrow();
                write!(f, "{},{}", i, tail)
            }
        }
    }
}

impl Node {
    pub fn new() -> Node {
        Nil(RefCell::new(Weak::new()))
    }

    pub fn from_vec(v: Vec<i32>) -> Node {
        let mut list = Node::new();
        for item in v.into_iter().rev() {
            list = list.cons(item);
        }
        list
    }

    pub fn cons(self, i: i32) -> Node {
        Cons(i, Rc::new(RefCell::new(self)))
    }

    pub fn head(&self) -> Option<i32> {
        match self {
            Cons(i, _) => Some(*i),
            _ => None,
        }
    }

    pub fn tail(&self) -> Option<&Rc<RefCell<Node>>> {
        match self {
            Cons(_, tail) => Some(tail),
            _ => None,
        }
    }

    pub fn length(&self) -> u32 {
        match self {
            Cons(_, tail) => 1 + tail.borrow().length(),
            _ => 0,
        }
    }

    pub fn get(&self, index: u32) -> Option<i32> {
        match self {
            Cons(i, tail) => match index {
                0 => {
                    //println!("found at index = {}", index);
                    Some(*i)
                },
                _ => {
                    //println!("get.cons index = {}", index);
                    tail.borrow().get(index - 1)
                },
            },
            Nil(cycle) => match cycle.borrow().upgrade() {
                Some(head) => {
                    //println!("get.nil index = {}", index);
                    head.borrow().get(index)
                },
                _ => None
            }
        }
    }

    pub fn set(&mut self, index: u32, val: i32) {
        match self {
            Cons(i, tail) => match index {
                0 => *i = val,
                _ => tail.as_ref().borrow_mut().set(index - 1, val),
            },
            _ => ()
        }
    }

    pub fn del(ref_list: &mut Rc<RefCell<Node>>, index: u32) {
        if index == 0 {
            let maybe_next_tail_clone: Option<Rc<RefCell<Node>>> = ref_list.borrow().tail().map(Rc::clone);
            if let Some(tail_clone) = maybe_next_tail_clone {
                *ref_list = tail_clone
            }
        } else {
            let list: &mut Node = &mut ref_list.as_ref().borrow_mut();
            list.delete_from_tail(index);
        }
    }

    pub fn delete_from_tail(&mut self, index: u32) {
        match self {
            Cons(_, tail) => match index {
                0 => panic!("should not call with index = 0"),
                1 => {
                    let maybe_next_tail_clone: Option<Rc<RefCell<Node>>> = tail.borrow().tail().map(Rc::clone);
                    if let Some(tail_clone) = maybe_next_tail_clone {
                        *tail = tail_clone
                    }
                },
                _=> tail.borrow_mut().delete_from_tail(index - 1)
            },
            _ => ()
        }
    }

    pub fn clear(&mut self) {
        *self = Node::new()
    }

    pub fn append(&mut self, i: i32) {
        match self {
            Nil(_) => *self = Cons(i, Rc::new(RefCell::new(Node::new()))),
            Cons(_, tail) => {
                tail.borrow_mut().append(i)
            }
        }
    }

    pub fn save_cycle(ref_list: &Rc<RefCell<Node>>) {
        ref_list.borrow().find_and_save_cycle(ref_list);
    }

    fn find_and_save_cycle(&self, head: &Rc<RefCell<Node>>) {
        match self {
            Nil(cycle) =>
                *cycle.borrow_mut() = Rc::downgrade(head),
            Cons(_, tail) =>
                tail.borrow().find_and_save_cycle(head)
        }
    }
}

pub struct CircularList(Rc<RefCell<Node>>);

impl fmt::Display for CircularList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.borrow())
    }
}

impl PartialEq for CircularList {
    fn eq(&self, other: &Self) -> bool {
        let first: &Node = &self.0.borrow();
        let second: &Node = &other.0.borrow();
        first == second
    }
}

pub struct CircularListIterator<'a> {
    list: &'a CircularList,
    index: u32,
}

impl Iterator for CircularListIterator<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let node: &Node = &self.list.0.borrow();
        let result = node.get(self.index);
        self.index += 1;
        result
    }
}

impl CircularList {
    pub fn new() -> CircularList {
        CircularList(Rc::new(RefCell::new(Node::new())))
    }

    pub fn from_vec(v: Vec<i32>) -> CircularList {
        let node = Node::from_vec(v);
        CircularList(Rc::new(RefCell::new(node)))
    }

    pub fn del(&mut self, index: u32) {
        if index == 0 {
            let maybe_next_tail_clone: Option<Rc<RefCell<Node>>> = self.0.borrow().tail().map(Rc::clone);
            if let Some(tail_clone) = maybe_next_tail_clone {
                *self = CircularList(tail_clone)
            }
        } else {
            let node: &mut Node = &mut self.0.as_ref().borrow_mut();
            node.delete_from_tail(index);
        }
    }

    pub fn save_cycle(&self) {
        self.0.borrow().find_and_save_cycle(self);
    }

    pub fn iter(&self) -> CircularListIterator {
        CircularListIterator {
            list: self,
            index: 0,
        }
    }
}

impl Clone for CircularList {
    fn clone(&self) -> Self {
        CircularList(Rc::clone(self))
    }
}

impl Deref for CircularList {
    type Target = Rc<RefCell<Node>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[macro_export]
macro_rules! node {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            Node::from_vec(temp_vec)
        }
    };
}

#[macro_export]
macro_rules! list {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            CircularList::from_vec(temp_vec)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        struct Case<'a> {
            l: Node,
            expected: &'a str
        }
        let test_cases = vec![
            Case { l: node![], expected: "⏚", },
            Case { l: node![1], expected: "1,⏚", },
            Case { l: node![1,2], expected: "1,2,⏚", },
            Case { l: node![1,2,3], expected: "1,2,3,⏚", }
        ];
        for test_case in test_cases.iter() {
            let actual = format!("{}", test_case.l);

            assert_eq!(test_case.expected, actual);
        }
    }

    #[test]
    fn test_eq() {
        struct Case {
            l1: Node,
            l2: Node,
            expected: bool
        }
        let test_cases = vec![
            Case { l1: node![], l2: node![], expected: true, },
            Case { l1: node![42], l2: node![42], expected: true, },
            Case { l1: node![42, 777], l2: node![42, 777], expected: true, },
            Case { l1: node![42], l2: node![], expected: false, },
            Case { l1: node![], l2: node![42], expected: false, },
            Case { l1: node![42], l2: node![24], expected: false, },
            Case { l1: node![42, 777], l2: node![42], expected: false, },
        ];
        for test_case in test_cases.iter() {
            let actual = test_case.l1 == test_case.l2;

            assert_eq!(test_case.expected, actual);
        }
    }

    #[test]
    fn test_length() {
        struct Case {
            l: Node,
            expected: u32
        }
        let test_cases = vec![
            Case { l: node![], expected: 0, },
            Case { l: node![42], expected: 1, },
            Case { l: node![1,2], expected: 2, },
            Case { l: node![42,777,666], expected: 3, },
        ];
        for test_case in test_cases.iter() {
            let actual = test_case.l.length();

            assert_eq!(test_case.expected, actual);
        }
    }

    #[test]
    fn test_cons() {
        struct Case {
            l: Node,
            i: i32,
            expected: Node
        }
        let test_cases = vec![
            Case { l: node![], i: 42, expected: node![42] },
            Case { l: node![777], i: 42, expected: node![42, 777] },
            Case { l: node![1,2,3], i: 4, expected: node![4,1,2,3] },
        ];
        for test_case in test_cases.into_iter() {
            let actual = test_case.l.cons(test_case.i);

            assert_eq!(test_case.expected, actual)
        }
    }

    #[test]
    fn test_get() {
        struct Case {
            l: Node,
            i: u32,
            expected: Option<i32>,
        }
        let test_cases = vec![
            Case { l: node![], i: 0, expected: None },
            Case { l: node![1,2,3], i: 0, expected: Some(1) },
            Case { l: node![1,2,3], i: 1, expected: Some(2) },
            Case { l: node![1,2,3], i: 2, expected: Some(3) },
            Case { l: node![1,2,3], i: 3, expected: None },
        ];
        for test_case in test_cases.into_iter() {
            let actual = test_case.l.get(test_case.i);

            assert_eq!(test_case.expected, actual)
        }
    }

    #[test]
    fn test_set() {
        struct Case {
            l: Node,
            i: u32,
            val: i32,
            expected: Node,
        }
        let test_cases = vec![
            Case { l: node![], i: 0, val: 42, expected: node![] },
            Case { l: node![0], i: 0, val: 42, expected: node![42] },
            Case { l: node![1,2,3,4], i: 0, val: 42, expected: node![42,2,3,4] },
            Case { l: node![1,2,3,4], i: 2, val: 42, expected: node![1,2,42,4] },
        ];
        for mut test_case in test_cases.into_iter() {
            test_case.l.set(test_case.i, test_case.val);

            assert_eq!(test_case.expected, test_case.l)
        }
    }

    #[test]
    fn test_del() {
        struct Case {
            l: Node,
            i: u32,
            expected: Node,
        }
        let test_cases = vec![
            Case { l: node![], i: 0, expected: node![] },
            Case { l: node![], i: 1, expected: node![] },
            Case { l: node![], i: 2, expected: node![] },
            Case { l: node![0], i: 0, expected: node![] },
            Case { l: node![0], i: 1, expected: node![0] },
            Case { l: node![0], i: 2, expected: node![0] },
            Case { l: node![1,2], i: 0, expected: node![2] },
            Case { l: node![1,2], i: 1, expected: node![1] },
            Case { l: node![1,2,3], i: 0, expected: node![2,3] },
            Case { l: node![1,2,3], i: 1, expected: node![1,3] },
            Case { l: node![1,2,3], i: 2, expected: node![1,2] },
        ];
        for test_case in test_cases.into_iter() {
            let mut ref_list = Rc::new(RefCell::new(test_case.l));

            Node::del(&mut ref_list, test_case.i);

            let actual = ref_list.borrow().eq(&test_case.expected);
            assert!(actual);
        }
    }

    #[test]
    fn test_append() {
        struct Case {
            l: Node,
            i: i32,
            expected: Node,
        }
        let test_cases = vec![
            Case { l: node![], i: 0, expected: node![0] },
            Case { l: node![0], i: 1, expected: node![0, 1] },
            Case { l: node![0,1], i: 2, expected: node![0, 1, 2] },
            Case { l: node![0,1,2,3], i: 4, expected: node![0,1,2,3,4] },
        ];
        for mut test_case in test_cases.into_iter() {
            test_case.l.append(test_case.i);

            assert_eq!(test_case.expected, test_case.l);
        }
    }
}
