// Disclaimer: Our ECE 421 group worked on this together
// Our code base is adapted from: https://play.rust-lang.org/?gist=d65d605a48d38648737ad2ae38f46434&version=stable

extern crate slab;

use slab::Slab;
use std::fmt;
use std::ops::{Index, IndexMut};
use num::iter::range;
use rand::StdRng;
use statrs::distribution::{Geometric, Distribution};

// A doubly linked SkipList
struct SkipList<T> {
    // All nodes get stored into this slab. A slab is basically just a
    // `Vec<Option<T>>` in disguse. We use it as a simple node allocator.
    slab: Slab<Node<T>>,
    // first node when pushing from the front
    head: Pointer,
    // first node when pushing from the back
    tail: Pointer,
    // the highest level of the list, 0 is the bottom linked list
    maxlevel: usize,
    // the number of items in level 0
    length: usize,
}

// A node in a doubly-linked list.
struct Node<T> {
    // The value stored in this node.
    // none valued nodes are either head or tail nodes
    value: Option<T>,
    // The next node in the list.
    next: Pointer,
    // The previous node in the list.
    prev: Pointer,
    // The pointer above this node
    above: Pointer,
    // The pointer below this node
    below: Pointer,
}

// impl<T> Node<T> {
// 	// a "null" node is used to represent a head or tail node with no value
// 	fn null() -> Self {
		
// 	}
// }

// A `Pointer` is just an index that refers to a node in the slab.
#[derive(Eq, PartialEq, Copy, Clone)]
struct Pointer(usize);

impl Pointer {
    // The null pointer is `!0`, which is the largest possible value of type
    // `usize`. There's no way we'll ever have a legitimate index that large.
    #[inline]
    fn null() -> Pointer {
        Pointer(!0)
    }
    
    // Returns `true` if this pointer is null.
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

impl<T: std::cmp::PartialOrd> SkipList<T> {
    // Returns a new doubly linked SkipList.
    fn new() -> SkipList<T> {
        SkipList {
            slab: Slab::new(),
            head: Pointer::null(), 
            tail: Pointer::null(),
            maxlevel: 0, // no items
            length: 0,   // no items
        }
    }

    // returns the number of elements at level 0 of the skip list.
    fn len(&self) -> usize {
    	self.length
	}

	// checks if the skip list is empty.
	fn is_empty(&self) -> bool{
		// if the head.next is null, the skiplist is empty
		self[self.head].next.is_null()
	}

    // Inserts a new element beginning at the front of the list.
    fn push_front(&mut self, t: T) {

    	// the first element in the list
    	let mut current = self.head;

        // null first node means list is empty
        if current.is_null() {

            // insert first element, new_node
        	let new = Pointer(self.slab.insert(
        		Node {
				    value: Some(t),
				    next: Pointer::null(),  // no other nodes exist
				    prev: Pointer::null(),  // no other nodes exist
				    above: Pointer::null(), // no other nodes exist
				    below: Pointer::null(), // no other nodes exist
	        	}
	        ));

	        // first node
	        self.length = 1;

	        // generate random height for new node
	        self.maxlevel = self.rand_level();

	        current = new;
	        for level in range(0, self.maxlevel) {

	        	// add another node above current, and shift current node up 
	        	let new = Pointer(self.slab.insert(
	        		Node {
					    value: Some(t),
					    next: Pointer::null(),  // no other values exist
					    prev: Pointer::null(),  // no other values exist
					    above: Pointer::null(), // so far none are above
					    below: current, // the previous node
		        	}
		        ));

	        	// point up to new node
	        	self[current].above = new;

	        	// re-adjust current pointer
		        current = new;

	        }

	        // once at max height, assign head and tail pointers to current node
	       	self.head = current;
	        self.tail = current;

        } else {
        	// search for location to insert t:
        	// for each level, starting at the top
        	for level in range(0, self.maxlevel + 1).rev() {

        		// while the next node is not null and its less than the to-be-inserted value
        		while !self[current].next.is_null() && self[self[current].next].value < t {
        			// move right
        			current = self[current].next;
        		}

        		// if there is lower to go, go lower (redundancy by using &&)
        		if level > 0 && !self[current].below.is_null() {
        			// move lower
        			current = self[current].below;
        		}
        	}

        	// "current" is at level 0, and current.next is where new node goes:
        	// create new node
        	let new = Pointer(self.slab.insert(
        		Node {
				    value: Some(t),
				    next: self[current].next,
				    prev: current,
				    above: Pointer::null(), // this will be changed below when we add height!
				    below: Pointer::null(), // level 0, down is null
	        	}
	        ));

	        // pointers to new node


	        // generate nodes above new node

        }
    }
    
 //    // Inserts a new element beginning at the back of the list.
 //    fn push_back(&mut self, t: T) -> Pointer {
 //        // let tail = self.tail;
 //        // if tail.is_null() {
 //        //     let n = Pointer(self.slab.insert(Node {
 //        //         value: t,
 //        //         prev: Pointer::null(),
 //        //         next: Pointer::null(),
 //        //         above: Pointer::null(),
 //        //         below: Pointer::null(),
 //        //     }));
 //        //     self.head = n;
 //        //     self.tail = n;
 //        //     n
 //        // } else {
 //        //     self.insert_after(tail, t)
 //        // }
 //    }

 	// sample a level from a geometric distribution of p = 0.5
    fn rand_level(&self) -> usize {

    	let mut r = rand::StdRng::new().unwrap();
		let n = Geometric::new(0.5).unwrap(); 
		return n.sample::<StdRng>(&mut r) as usize;
	}
   
}

impl<T: fmt::Debug> fmt::Debug for SkipList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        let mut n = self.head;
        
        write!(f, "SkipList(")?;
        while !n.is_null() {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            
            write!(f, "{:?}", self[n].value)?;
            n = self[n].next;
        }
        write!(f, ")")?;
        
        Ok(())
    }
}

fn main() {
	let mut sl: SkipList<f64> = SkipList::new();

	// println!("A few levels: {}, {}, {}, {}, {}", sl.rand_level(),sl.rand_level(),sl.rand_level(),sl.rand_level(),sl.rand_level())

    // println!("create an empty doubly-linked list");
    // let mut list = List::new();
    // println!("{:?}\n", list);
    
    // println!("push 9 to the back");
    // let a = list.push_back(9);
    // println!("{:?}\n", list);
    
    // println!("push 0 to the front");
    // let b = list.push_front(0);
    // println!("{:?}\n", list);
    
    // println!("insert 3 after {}", list[a].value);
    // let c = list.insert_after(a, 3);
    // println!("{:?}\n", list);
    
    // println!("change {} to 1", list[a].value);
    // list[a].value = 1;
    // println!("{:?}\n", list);
    
    // println!("insert 2 before {}", list[c].value);
    // let d = list.insert_before(c, 2);
    // println!("{:?}\n", list);
    
    // println!("remove {}", list.remove(a));
    // println!("{:?}\n", list);
    
    // println!("remove {}", list.remove(d));
    // println!("{:?}\n", list);
    
    // println!("remove {}", list.remove(b));
    // println!("{:?}\n", list);
    
    // println!("remove {}", list.remove(c));
    // println!("{:?}\n", list);
}