use std::fmt::Display;
use std::cmp::Ord;

enum Tree<'a, T: 'a + Display + Ord>
{
    Null,
    Node {
        data: T,
        left: Box<Tree<'a, T>>,
        right: Box<Tree<'a, T>>
    }
}

impl<'a, T: 'a + Display + Ord> Tree<'a, T> {
    fn _print(&self, prefix: String, is_tail: bool)
    {
        match self {
            &Tree::Node { ref data, left, right } => {
                println!("{}{}{}", prefix.clone(), if is_tail { "`-- " } else { "+-- " }, data.to_string());
                match right {
                     &Tree::Node { .. } => {
                         right._print(prefix.clone() + if !is_tail { "|   " } else { "    " },
                                      match left { &Tree::Null => true, _ => false });
                     }
                    _ => ()
                }
                match left {
                    ref left @ &Tree::Node { .. }  => {
                        left._print(prefix.clone() + if !is_tail { "|   " } else { "    " },
                                    true);
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }


    fn print(&self)
    {
        match self {
            &Tree::Null => println!("Null"),
            &Tree::Node { ref data, .. } => {
                println!("{}", data);
                self._print("".to_string(), true);
            }
        }
    }

    fn put(&mut self, new_data: T)
    {
        match self {
            &mut Tree::Null => *self = Tree::Node{ data: new_data, left: Tree::Null, right: Tree::Null },
            &mut Tree::Node { ref data, ref left, ref right } => {
                if new_data > *data {
                    right.put(new_data);
                } else {
                    left.put(new_data);
                }
            }
        }
    }
}

fn main()
{
    let t : Tree<u32> = Tree::Null;
    let g = Tree::Node { data: 34, left: &Tree::Null, right: &Tree::Null };
/*    t.put(3);
    t.put(5);
    t.put(7);
    t.put(10);
    t.put(1); */
    t.print();
    g.print();
}
