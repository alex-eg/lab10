use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use std::cmp::Ordering;
use std::option::Option;
use std::clone::Clone;

enum Tree<T> {
    Null,
    Node {
        data: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>
    }
}


impl<T> Tree<T> {

    fn new_empty() -> Tree<T> {
        Tree::Null
    }

    fn new_filled(data: T) -> Tree<T> {
        Tree::Node { data: data,
                     left: Box::new(Tree::Null),
                     right: Box::new(Tree::Null) }
    }
}

impl<T: Display> Tree<T> {

    fn _print(&self, prefix: String, is_tail: bool, f: &mut Formatter, is_first: bool)
              -> Result {
        match self {
            &Tree::Node { ref data, ref left, ref right } => {
                let _ = writeln!(f, "{}{}{}", prefix.clone(),
                                 if is_first { "" } else if is_tail { "└ " } else { "├ " },
                                 data.to_string());
                match **right {
                    Tree::Node { .. } => {
                        let _ = right._print(prefix.clone() +
                                             if is_first { "" } else
                                             if !is_tail { "│ " } else { "  " },
                                             match **left {
                                                 Tree::Null => true,
                                                 _ => false },
                                             f, false);
                    },
                    _ => ()
                };
                match **left {
                    Tree::Node { .. }  => {
                        left._print(prefix.clone() +
                                    if is_first { "" } else
                                    if is_tail { "  " } else { "│ " },
                                    true,
                                    f,
                                    false)
                    }
                    _ => Ok(())
                }
            }
            _ => Ok(())
        }
    }
}

impl<T: Clone> Tree<T> {

    fn find_max(&self) -> Option<T> {
        match self {
            &Tree::Null => None,
            &Tree::Node { ref data, ref right, .. } => {
                match **right {
                    Tree::Null => Some(data.clone()),
                    Tree::Node { .. }=> { right.find_max() }
                }
            }
        }
    }
}

impl<T: PartialOrd> Tree<T> {

    fn put(&mut self, new_data: T) -> () {
        match self {
            &mut Tree::Null => *self = Tree::new_filled(new_data),
            &mut Tree::Node { ref data,
                              ref mut left,
                              ref mut right } => {
                if new_data >= *data {
                    right.put(new_data);
                } else {
                    left.put(new_data);
                }
            }
        }
    }
}

impl <T: PartialEq> PartialEq for Tree<T> {
    fn eq(&self, other: &Tree<T>) -> bool {
        match self {
            &Tree::Null => match other { &Tree::Null => true, _ => false },
            &Tree::Node { ref data, .. } => match other {
                &Tree::Node { data: ref o_data, .. } => *data == *o_data,
                _ => false
            }
        }
    }
}

impl <T: PartialOrd> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Tree<T>) -> Option<Ordering> {
        match self {
            &Tree::Null => match other { &Tree::Null => Some(Ordering::Equal),
                                          _ => Some(Ordering::Less) },
            &Tree::Node { ref data, .. } => match other {
                &Tree::Node { data: ref o_data, .. } => data.partial_cmp(o_data),
                _ => Some(Ordering::Greater)
            }
        }
    }
}

impl<T: Display> Display for Tree<T> {

    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &Tree::Null => write!(f, "Null"),
            &Tree::Node { .. } => {
                self._print("".to_string(), true, f, true)
            }
        }
    }
}

macro_rules! print_opt {
    ($v:expr, $ok:expr, $fail:expr) => {{
        if let Some(val) = { $v } {
            println!($ok, val);
        } else {
            println!($fail);
        }
    }}
}

fn main() {
    let mut t : Tree<u32> = Tree::new_empty();
    let g = Tree::new_filled(34);

    println!("{}", t);
    print_opt!(t.find_max(),
               "Max in t: {}",
               "Max in t is None");
    t.put(3);
    t.put(5);
    t.put(7);
    t.put(6);
    t.put(10);
    t.put(1);
    t.put(3);
    println!("{}", t);
    println!("{}", g);

    print_opt!(t.find_max(),
               "Max in t: {}",
               "Max in t is None");

    let mut t1 = Tree::new_filled(7.3);
    t1.put(2.3);
    t1.put(2.1);
    t1.put(5.0);
    t1.put(8.0);
    t1.put(6.0);

    let mut t2 = Tree::new_filled(2.2);
    t2.put(1.0);
    t2.put(3.4);

    let mut tt: Tree<Tree<f32>> = Tree::new_empty();
    tt.put(t2);
    tt.put(t1);
    println!("{}", tt);
}
