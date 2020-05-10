use crate::circular_list::CircularList::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::fmt;

#[derive(Debug)]
pub enum CircularList{
    Nil,
    Cons(i32
         ,Rc<RefCell<CircularList>>
         //,Rc<RefCell<Weak<CircularList>>>
    )
}

impl PartialEq for CircularList {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Nil => if let Nil = other {
                true
            } else {
                false
            },
            Cons(i, tail) => match other {
                Nil => false,
                Cons(j, other_tail) =>
                    i == j && tail.borrow().eq(&other_tail.borrow())
            }
        }
    }
}

impl fmt::Display for CircularList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Nil => write!(f, "nil"),
            Cons(i, next) => {
                let tail = next.borrow();
                write!(f, "{},{}", i, tail)
            }
        }
    }
}

impl CircularList {
    pub fn new() -> CircularList {
        Nil
    }

    pub fn cons(self, i: i32) -> CircularList {
        Cons(i, Rc::new(RefCell::new(self)))
    }

    pub fn head(&self) -> Option<i32> {
        match self {
            Nil => None,
            Cons(i, _) => Some(*i)
        }
    }

    pub fn tail(&self) -> Option<&Rc<RefCell<CircularList>>> {
        match self {
            Cons(_, tail) => Some(tail),
            _ => None,
        }
    }

    pub fn length(&self) -> u32 {
        match self {
            Nil => 0,
            Cons(_, tail) => 1 + tail.borrow().length()
        }
    }

    pub fn get(&self, index: u32) -> Option<i32> {
        match self {
            Nil => None,
            Cons(i, tail) => match index {
                0 => Some(*i),
                _ => tail.borrow().get(index - 1),
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

    pub fn del(ref_list: &mut Rc<RefCell<CircularList>>, index: u32) {
        if index == 0 {
            let maybe_next_tail_clone: Option<Rc<RefCell<CircularList>>> = ref_list.borrow().tail().map(Rc::clone);
            if let Some(tail_clone) = maybe_next_tail_clone {
                *ref_list = tail_clone
            }
        } else {
            let list: &mut CircularList = &mut ref_list.as_ref().borrow_mut();
            list.delete_from_tail(index);
        }
    }

    pub fn delete_from_tail(&mut self, index: u32) {
        match self {
            Cons(_, tail) => match index {
                0 => panic!("should not call with index = 0"),
                1 => {
                    let maybe_next_tail_clone: Option<Rc<RefCell<CircularList>>> = tail.borrow().tail().map(Rc::clone);
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
        *self = Nil
    }

    pub fn append(&mut self, i: i32) {
        match self {
            Nil => *self = Cons(i, Rc::new(RefCell::new(Nil))),
            Cons(_, tail) => {
                tail.borrow_mut().append(i)
            }
        }
    }
}

#[macro_export]
macro_rules! list {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec.reverse();
            let mut temp_list = CircularList::new();
            for item in temp_vec.into_iter() {
                temp_list = temp_list.cons(item);
            }
            temp_list
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        struct Case<'a> {
            l: CircularList,
            expected: &'a str
        }
        let test_cases = vec![
            Case { l: Nil, expected: "nil", },
            Case { l: list![1], expected: "1,nil", },
            Case { l: list![1,2], expected: "1,2,nil", },
            Case { l: list![1,2,3], expected: "1,2,3,nil", }
        ];
        for test_case in test_cases.iter() {
            let actual = format!("{}", test_case.l);

            assert_eq!(test_case.expected, actual);
        }
    }

    #[test]
    fn test_eq() {
        struct Case {
            l1: CircularList,
            l2: CircularList,
            expected: bool
        }
        let test_cases = vec![
            Case { l1: Nil, l2: Nil, expected: true, },
            Case { l1: list![42], l2: list![42], expected: true, },
            Case { l1: list![42, 777], l2: list![42, 777], expected: true, },
            Case { l1: list![42], l2: Nil, expected: false, },
            Case { l1: Nil, l2: list![42], expected: false, },
            Case { l1: list![42], l2: list![24], expected: false, },
            Case { l1: list![42, 777], l2: list![42], expected: false, },
        ];
        for test_case in test_cases.iter() {
            let actual = test_case.l1 == test_case.l2;

            assert_eq!(test_case.expected, actual);
        }
    }

    #[test]
    fn test_length() {
        struct Case {
            l: CircularList,
            expected: u32
        }
        let test_cases = vec![
            Case { l: Nil, expected: 0, },
            Case { l: list![42], expected: 1, },
            Case { l: list![1,2], expected: 2, },
            Case { l: list![42,777,666], expected: 3, },
        ];
        for test_case in test_cases.iter() {
            let actual = test_case.l.length();

            assert_eq!(test_case.expected, actual);
        }
    }

    #[test]
    fn test_cons() {
        struct Case {
            l: CircularList,
            i: i32,
            expected: CircularList
        }
        let test_cases = vec![
            Case { l: list![], i: 42, expected: list![42] },
            Case { l: list![777], i: 42, expected: list![42, 777] },
            Case { l: list![1,2,3], i: 4, expected: list![4,1,2,3] },
        ];
        for test_case in test_cases.into_iter() {
            let actual = test_case.l.cons(test_case.i);

            assert_eq!(test_case.expected, actual)
        }
    }

    #[test]
    fn test_get() {
        struct Case {
            l: CircularList,
            i: u32,
            expected: Option<i32>,
        }
        let test_cases = vec![
            Case { l: list![], i: 0, expected: None },
            Case { l: list![1,2,3], i: 0, expected: Some(1) },
            Case { l: list![1,2,3], i: 1, expected: Some(2) },
            Case { l: list![1,2,3], i: 2, expected: Some(3) },
            Case { l: list![1,2,3], i: 3, expected: None },
        ];
        for test_case in test_cases.into_iter() {
            let actual = test_case.l.get(test_case.i);

            assert_eq!(test_case.expected, actual)
        }
    }

    #[test]
    fn test_set() {
        struct Case {
            l: CircularList,
            i: u32,
            val: i32,
            expected: CircularList,
        }
        let test_cases = vec![
            Case { l: list![], i: 0, val: 42, expected: list![] },
            Case { l: list![0], i: 0, val: 42, expected: list![42] },
            Case { l: list![1,2,3,4], i: 0, val: 42, expected: list![42,2,3,4] },
            Case { l: list![1,2,3,4], i: 2, val: 42, expected: list![1,2,42,4] },
        ];
        for mut test_case in test_cases.into_iter() {
            test_case.l.set(test_case.i, test_case.val);

            assert_eq!(test_case.expected, test_case.l)
        }
    }

    #[test]
    fn test_del() {
        struct Case {
            l: CircularList,
            i: u32,
            expected: CircularList,
        }
        let test_cases = vec![
            Case { l: list![], i: 0, expected: list![] },
            Case { l: list![], i: 1, expected: list![] },
            Case { l: list![], i: 2, expected: list![] },
            Case { l: list![0], i: 0, expected: list![] },
            Case { l: list![0], i: 1, expected: list![0] },
            Case { l: list![0], i: 2, expected: list![0] },
            Case { l: list![1,2], i: 0, expected: list![2] },
            Case { l: list![1,2], i: 1, expected: list![1] },
            Case { l: list![1,2,3], i: 0, expected: list![2,3] },
            Case { l: list![1,2,3], i: 1, expected: list![1,3] },
            Case { l: list![1,2,3], i: 2, expected: list![1,2] },
        ];
        for test_case in test_cases.into_iter() {
            let mut ref_list = Rc::new(RefCell::new(test_case.l));

            CircularList::del(&mut ref_list, test_case.i);

            let actual = ref_list.borrow().eq(&test_case.expected);
            assert!(actual);
        }
    }

    #[test]
    fn test_append() {
        struct Case {
            l: CircularList,
            i: i32,
            expected: CircularList,
        }
        let test_cases = vec![
            Case { l: list![], i: 0, expected: list![0] },
            Case { l: list![0], i: 1, expected: list![0, 1] },
            Case { l: list![0,1], i: 2, expected: list![0, 1, 2] },
            Case { l: list![0,1,2,3], i: 4, expected: list![0,1,2,3,4] },
        ];
        for mut test_case in test_cases.into_iter() {
            test_case.l.append(test_case.i);

            assert_eq!(test_case.expected, test_case.l);
        }
    }
}
