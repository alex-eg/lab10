use std::fmt::Display;
use std::cmp::Ord;
use std::borrow::BorrowMut;
use std::borrow::Borrow;

enum Tree<T: Display + Ord>
{
    Null,
    Node {
        data: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>
    }
}

impl<T> Tree<T>
    where T : Display + Ord {

    fn _print(&self, prefix: String, is_tail: bool)
    {
        match self {
            &Tree::Node { ref data, ref left, ref right } => {
                println!("{}{}{}", prefix.clone(),
                         if is_tail { "└ " } else { "├ " }, data.to_string());
                match right.borrow() {
                    &Tree::Node { .. } => {
                        right._print(prefix.clone() +
                                     if !is_tail { "│ " } else { "  " },
                                     match left.borrow() {
                                         &Tree::Null => true, _ => false });
                    }
                    _ => ()
                }
                match left.borrow() {
                    &Tree::Node { .. }  => {
                        left._print(prefix.clone() +
                                    if !is_tail { "│ " } else { "  " },
                                    true);
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }

    fn new_empty() -> Tree<T>
    {
        Tree::Null
    }

    fn new_filled(data: T) -> Tree<T>
    {
        Tree::Node { data: data,
                     left: Box::new(Tree::Null),
                     right: Box::new(Tree::Null) }
    }

    fn print(&self)
    {
        match self {
            &Tree::Null => println!("Null"),
            &Tree::Node { .. } => {
                self._print("".to_string(), true);
            }
        }
    }

    fn put(&mut self, new_data: T) -> ()
    {
        match self {
            &mut Tree::Null => *self = Tree::new_filled(new_data),
            &mut Tree::Node { ref data,
                              ref mut left,
                              ref mut right } => {
                if new_data > *data {
                    let ref mut unboxed: Tree<T> = *right.borrow_mut();
                    unboxed.put(new_data);
                } else {
                    let ref mut unboxed: Tree<T> = *left.borrow_mut();
                    unboxed.put(new_data);
                }
            }
        }
    }
}

fn main()
{
    let mut t : Tree<u32> = Tree::new_empty();
    let g = Tree::new_filled(34);
    t.print();
    println!("");
    t.put(3);
    t.put(5);
    t.put(7);
    t.put(6);
    t.put(10);
    t.put(1);
    t.print();
    println!("");
    g.print();
}
