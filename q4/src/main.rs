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
}

impl<T: PartialOrd> SkipList<T> {
    // Returns a new doubly linked list.
    pub fn new() -> Self {
        SkipList {
            slab: Slab::new(),
            head: Pointer::null(),
        }
    }

    pub fn len(&self) -> usize{
        return self.slab.len();
    }

    pub fn is_empty(&self) -> bool{
        return self.len() == 0;
    }

    // Inserts a new element at the front of the list.
    pub fn push(&mut self, t: T){
        let head = self.head;
        if head.is_null() {
            let new_node = Pointer(self.slab.insert(Node {
                value: t,
                right: Pointer::null(),
                left: Pointer::null(),
                up: Pointer::null(),
                down: Pointer::null(),
            }));
            self.head = new_node;
        } else {
            self.insert(head, t);
        }
    }

    fn insert(&mut self, node: Pointer, value:T){
        let right = self[node].right;
        let down = self[node].down;
        let left = self[node].left;

        // on bottom layer
        if down.is_null(){
            if right.is_null(){
                if self[node].value > value{
                    self.insert_before(node, value);
                }
                else{
                    self.insert_after(node, value);
                }
            }
            else if self[right].value > value{
                self.insert_after(node, value);
            }
            else if self[right].value == value{
                panic!("Duplicate Value added")
            }
            else{
                if left.is_null(){
                    self.head = self.insert_before(node, value); 
                }
                else{
                    self.insert(right, value);
                }
            }
        }
        else if right.is_null(){
            self.insert(down, value);
        }
        else if self[right].value > value{
            self.insert(down, value);
        }
        else if self[right].value == value{
            panic!("Duplicate Value added")
        }
        else{
            self.insert(right, value);
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
        
        if !next.is_null() {
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

    // Inserts a new element at the back of the list.
    pub fn push_back(&mut self, t: T){
    }

}



fn main() {
    println!("create an empty doubly-linked list");
    let mut list = SkipList::new();
    println!("{:?}\n", list);
    
    println!("push 9");
    let a = list.push(9);
    println!("{:?}\n", list);
    
    println!("push 1");
    let b = list.push(1);
    println!("{:?}\n", list);

    println!("push 2");
    let b = list.push(2);
    println!("{:?}\n", list);

    println!("push 4");
    let b = list.push(4);
    println!("{:?}\n", list);

    // println!("push 12");
    // let b = list.push(12);
    // println!("{:?}\n", list);

    // println!("push 11");
    // let b = list.push(11);
    // println!("{:?}\n", list);

    // println!("push 0");
    // let b = list.push(0);
    // println!("{:?}\n", list);

    println!("{:?}\n", list);
}