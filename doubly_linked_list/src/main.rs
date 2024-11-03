use std::{fmt, cell::RefCell, rc::Rc};

trait List<T> {
    fn new() -> Self;
    // Append an element to the back of a list
    fn push_back(&mut self, elm: T);
    // Append an element to the front of a list
    fn push_front(&mut self, elm: T);
    // Remove an element from the back of a list
    fn pop_back(&mut self) -> Option<T>;
    // Remove an element from the front of a list
    fn pop_front(&mut self) -> Option<T>;
}

#[derive(Clone)]
struct Node<T: Clone> {
    data: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

struct DoublyLinkedList<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> List<T> for DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, elm: T) {
        let mut new_node = Node {
            data: elm,
            prev: None,
            next: None,
        };
        if self.head.is_none() {
            let new_node = Rc::new(RefCell::new(new_node));
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            new_node.prev = self.tail.clone();
            let new_node = Rc::new(RefCell::new(new_node));
            if let Some(tail) = &mut self.tail {
                tail.borrow_mut().next = Some(new_node.clone());
            }
            self.tail = Some(new_node);
        }
    }

    fn push_front(&mut self, elm: T) {
        let mut new_node = Node {
            data: elm,
            prev: None,
            next: None,
        };
        if self.head.is_none() {
            let new_node = Rc::new(RefCell::new(new_node));
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            new_node.next = self.head.clone();
            let new_node = Rc::new(RefCell::new(new_node));
            if let Some(head) = &mut self.head {
                head.borrow_mut().prev = Some(new_node.clone());
            }
            self.head = Some(new_node);
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            None
        } else {
            if let Some(tail) = &mut self.tail {
                let data = tail.borrow().clone().data;
                if tail.borrow().prev.is_none() {
                    self.head = None;
                    self.tail = None;
                } else {
                    let t = tail.clone();
                    let mut t_borrow = t.borrow_mut();
                    self.tail = t_borrow.prev.clone();
                    if let Some(prev) = &mut t_borrow.prev {
                        prev.borrow_mut().next = None;
                    }
                }
                Some(data)
            } else {
                None
            }
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.head.is_none() {
            None
        } else {
            if let Some(head) = &mut self.head {
                let data = head.borrow().clone().data;
                if head.borrow().next.is_none() {
                    self.head = None;
                    self.tail = None;
                } else {
                    let h = head.clone();
                    let mut h_borrow = h.borrow_mut();
                    self.head = h_borrow.next.clone();
                    if let Some(next) = &mut h_borrow.next {
                        next.borrow_mut().prev = None;
                    }
                }
                Some(data)
            } else {
                None
            }
        }
    }
}

impl<T: Clone> fmt::Display for DoublyLinkedList<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current = self.head.clone();
        write!(f, "from head: (")?;
        while let Some(node) = current {
            let n = node.borrow();
            write!(f, "{}", n.data)?;
            write!(f, " @{:p}", &n.data)?;
            current = n.next.clone();
            if current.is_some() {
                write!(f, "<--->")?;
            }
        }
        write!(f, ")")?;

        let mut current = self.tail.clone();
        write!(f, ", from tail: (")?;
        while let Some(node) = current {
            let n = node.borrow();
            write!(f, "{}", n.data)?;
            write!(f, " @{:p}", &n.data)?;
            current = n.prev.clone();
            if current.is_some() {
                write!(f, " <---> ")?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

fn main() {
    let mut list: DoublyLinkedList<i8> = DoublyLinkedList::new();

    println!("[push_back test]");

    println!("{}", list); // ()

    list.push_back(1);
    println!("{}", list); // (1)

    list.push_back(2);
    println!("{}", list); // (1 <---> 2)

    list.push_back(3);
    println!("{}", list); // (1 <---> 2 <---> 3)

    println!("");
    println!("[push_front test]");

    list.push_front(4);
    println!("{}", list); // (4 <---> 1 <---> 2 <---> 3)

    list.push_front(5);
    println!("{}", list); // (5 <---> 4 <---> 1 <---> 2 <---> 3)

    list.push_front(6);
    println!("{}", list); // (6 <---> 5 <---> 4 <---> 1 <---> 2 <---> 3)

    println!("");
    println!("[pop_back test]");

    let removed: Option<i8> = list.pop_back();
    println!("removed node: {}", removed.unwrap());
    println!("{}", list); // (6 <---> 5 <---> 4 <---> 1 <---> 2)

    let removed = list.pop_back();
    println!("removed node: {}", removed.unwrap());
    println!("{}", list); // (6 <---> 5 <---> 4 <---> 1)

    let removed = list.pop_back();
    println!("removed node: {}", removed.unwrap());
    println!("{}", list); // (6 <---> 5 <---> 4)

    println!("");
    println!("[pop_front test]");

    let removed = list.pop_front();
    println!("removed node: {}", removed.unwrap());
    println!("{}", list); // (5 <---> 4)

    let removed = list.pop_front();
    println!("removed node: {}", removed.unwrap());
    println!("{}", list); // (4)

    let removed = list.pop_front();
    println!("removed node: {}", removed.unwrap());
    println!("{}", list); // ()

    let removed = list.pop_front();
    // TODO: make common functioin wrapping this
    match removed {
        Some(_) => println!("removed node: {}", removed.unwrap()),
        None => println!("list is empty"),
    }
    println!("{}", list); // ()
}
