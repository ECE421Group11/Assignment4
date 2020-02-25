// Disclaimer: Our ECE 421 group worked on this together
// Our code base is adapted from: https://play.rust-lang.org/?gist=d65d605a48d38648737ad2ae38f46434&version=stable

extern crate slab;

use slab::Slab;
use std::fmt;
use std::ops::{Index, IndexMut};

impl<T: fmt::Debug> fmt::Debug for SkipList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        let mut n = self.head;
        
        write!(f, "List(")?;
        while !n.is_null() {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            
            write!(f, "{:?}", self[n].value)?;
            n = self[n].right;
        }
        write!(f, ")")?;
        
        Ok(())
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Pointer(usize);

impl Pointer {
    #[inline]
    fn null() -> Pointer {
        Pointer(!0)
    }
    
    #[inline]
    fn is_null(&self) -> bool {
        *self == Pointer::null()
    }
}

// Just for convenience, so that we can type `self[i]` instead of `self.slab[i]`.
impl<T> Index<Pointer> for SkipList<T> {
    type Output = Node<T>;
    
    fn index(&self, index: Pointer) -> &Node<T> {
        &self.slab[index.0]
    }
}

// Just for convenience, so that we can type `self[i]` instead of `self.slab[i]`.
impl<T> IndexMut<Pointer> for SkipList<T> {
    fn index_mut(&mut self, index: Pointer) -> &mut Node<T> {
        &mut self.slab[index.0]
    }
}

struct Node<T> {
    value: T,
    right: Pointer,
    left: Pointer,
    up: Pointer,
    down: Pointer
}

struct SkipList<T> {
    slab: Slab<Node<T>>,
    head: Pointer,
    tail: Pointer,
}

impl<T> SkipList<T> {
    // Returns a new doubly linked list.
    fn new() -> SkipList<T> {
        SkipList {
            slab: Slab::new(),
            head: Pointer::null(),
            tail: Pointer::null(),
        }
    }
    
    // Inserts a new element at the back of the list.
    fn push_back(&mut self, t: T) -> Pointer {
        let tail = self.tail;
        if tail.is_null() {
            let n = Pointer(self.slab.insert(Node {
                value: t,
                right: Pointer::null(),
                left: Pointer::null(),
                up: Pointer::null(),
                down: Pointer::null(),

            }));
            self.head = n;
            self.tail = n;
            n
        } else {
            self.insert_after(tail, t)
        }
    }
    
    // Inserts a new element at the front of the list.
    fn push_front(&mut self, t: T) -> Pointer {
        let head = self.head;
        if head.is_null() {
            self.push_back(t)
        } else {
            self.insert_before(head, t)
        }
    }
    
    // Inserts a new element after `node`.
    fn insert_after(&mut self, node: Pointer, t: T) -> Pointer {
        let next = self[node].right;
        let n = Pointer(self.slab.insert(Node {
            value: t,
            left: node,
            right: next,
            up: Pointer::null(),
            down: Pointer::null(),
        }));
        
        if next.is_null() {
            self.tail = n;
        } else {
            self[next].left = n;
        }
        self[node].right = n;
        n
    }
    
    // Inserts a new element before `node`.
    fn insert_before(&mut self, node: Pointer, t: T) -> Pointer {
        let prev = self[node].left;
        let n = Pointer(self.slab.insert(Node {
            value: t,
            left: prev,
            right: node,
            up: Pointer::null(),
            down: Pointer::null(),
        }));
        
        if prev.is_null() {
            self.head = n;
        } else {
            self[prev].right = n;
        }
        self[node].left = n;
        n
    }
    
    // Removes `node` from the list and returns its value.
    fn remove(&mut self, node: Pointer) -> T {
        let prev = self[node].left;
        let next = self[node].right;
        
        if prev.is_null() {
            self.head = next;
        } else {
            self[prev].right = next;
        }
        
        if next.is_null() {
            self.tail = prev;
        } else {
            self[next].left = prev;
        }
        
        self.slab.remove(node.0).value
    }
}



fn main() {
    println!("create an empty doubly-linked list");
    let mut list = SkipList::new();
    println!("{:?}\n", list);
    
    println!("push 9 to the back");
    let a = list.push_back(9);
    println!("{:?}\n", list);
    
    println!("push 0 to the front");
    let b = list.push_front(0);
    println!("{:?}\n", list);
    
    println!("insert 3 after {}", list[a].value);
    let c = list.insert_after(a, 3);
    println!("{:?}\n", list);
    
    println!("change {} to 1", list[a].value);
    list[a].value = 1;
    println!("{:?}\n", list);
    
    println!("insert 2 before {}", list[c].value);
    let d = list.insert_before(c, 2);
    println!("{:?}\n", list);
    
    println!("remove {}", list.remove(a));
    println!("{:?}\n", list);
    
    println!("remove {}", list.remove(d));
    println!("{:?}\n", list);
    
    println!("remove {}", list.remove(b));
    println!("{:?}\n", list);
    
    println!("remove {}", list.remove(c));
    println!("{:?}\n", list);
}