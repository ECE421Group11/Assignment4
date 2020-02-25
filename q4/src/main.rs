// Disclaimer: Our ECE 421 group worked on this together
// Our code base is adapted from: https://play.rust-lang.org/?gist=d65d605a48d38648737ad2ae38f46434&version=stable

extern crate slab;

use statrs::distribution::{Geometric, Distribution};
use std::ops::{Index, IndexMut};
use num::iter::range;
use rand::StdRng;
use slab::Slab;
use std::fmt;

const MAXHEIGHT: usize = 16;

// A doubly linked SkipList
struct SkipList<T> {
    // All nodes get stored into this slab. A slab is basically just a
    // `Vec<Option<T>>` in disguse. We use it as a simple node allocator.
    slab: Slab<Node<T>>,
    // first node for each level when pushing from the front
    headnodes: [Pointer; MAXHEIGHT], // 0 is lowest level
    // "first" from each level when pushing from the back
    tailnodes: [Pointer; MAXHEIGHT], // 0 is lowest level
}

// A node in a doubly-linked list.
struct Node<T> {
    // none valued nodes are either head or tail nodes
    // (not to be confused with null pointers which represent end of up/down direction)
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

// A `Pointer` is just an index that refers to a node in the slab.
#[derive(Default, Eq, PartialEq, Copy, Clone)]
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

impl<T: std::clone::Clone + std::cmp::PartialOrd + std::fmt::Debug> SkipList<T> {
    // Returns a new doubly linked SkipList.
    fn new() -> SkipList<T> {
    	let mut slab = Slab::new();
    	let mut headnodes: [Pointer; MAXHEIGHT] = Default::default();
    	let mut tailnodes: [Pointer; MAXHEIGHT] = Default::default();
    	
    	// make all the nodes (Levels 0-15)
    	for level in range(0, MAXHEIGHT) {
    		headnodes[level] = Pointer(slab.insert(
        		Node {
				    value: None, // makes it a non-values node
				    next: Pointer::null(),
				    prev: Pointer::null(),
				    above: Pointer::null(),
				    below: Pointer::null(),
	        	}
	        ));

	        tailnodes[level] = Pointer(slab.insert(
        		Node {
				    value: None, // makes it a non-values node
				    next: Pointer::null(),
				    prev: Pointer::null(),
				    above: Pointer::null(),
				    below: Pointer::null(),
	        	}
	        ));

	        // point to eachother
	        slab[headnodes[level].0].next = tailnodes[level];
	        slab[tailnodes[level].0].prev = headnodes[level];
    	}

    	// link upwards level 0 -> 14
    	for level in range(0, MAXHEIGHT - 1) {
    		// link current level to the above one
    		slab[headnodes[level].0].above = headnodes[level+1];
    		slab[tailnodes[level].0].above = tailnodes[level+1];
    	}

    	// link downwards level 15 -> 1
    	for level in range(1, MAXHEIGHT).rev() {
    		// link current level to the above one
    		slab[headnodes[level].0].below = headnodes[level-1];
    		slab[tailnodes[level].0].below = tailnodes[level-1];
    	}

        SkipList {
            slab: slab,
            headnodes: headnodes, 
            tailnodes: tailnodes,
        }
    }

    // returns the number of elements at level 0 of the skip list.
    fn len(&self) -> usize {
    	// start from the 0th header and count till tail is found
    	let mut current = self.headnodes[0];
    	let mut length: usize = 0;

    	loop {
    		// progress current pointer to the right
    		current = self[current].next;

    		match self[current].value {
    			Some(ref val) => {
    				length = &length + 1;
    			},
    			None => {
    				return length;
    			},
    		};
    	};
	}

	// returns the number of elements at the requested level of the skip list.
    fn lev_len(&self, height: usize) -> usize {
    	// start from the nth header and count till tail is found
    	let mut current = self.headnodes[height];
    	let mut length: usize = 0;

    	loop {
    		// progress current pointer to the right
    		current = self[current].next;

    		match self[current].value {
    			Some(ref val) => {
    				length = &length + 1;
    			},
    			None => {
    				return length;
    			},
    		};
    	};
	}

	// checks if the skip list is empty.
	fn is_empty(&self) -> bool{
		// if the head.next is null, the skiplist is empty
		self.len() == 0
	}

    // Inserts a new element beginning at the front of the list.
    fn push_front(&mut self, t: T) {
		// the starting node
    	let mut current = self.headnodes[MAXHEIGHT-1];

    	// for each level from 15 -> 0
    	for level in range(0, MAXHEIGHT).rev() {

    		// if there is no value node in the level, we must force it to skip
    		match self[current].value {
    			Some(_) => { }, // nothing to do
    			None => {
    				match self[self[current].next].value {
    					Some(_) => { }, // nothing to do
    					None => {
    						// No value in this level, skip
    						// if there is lower to go, go lower
				    		if level > 0 {
				    			// move lower
				    			current = self[current].below;
				    		};
    						continue;
    					}
    				}
    			}
    		}

    		// while the next node has a value and the value
    		// is less than the to-be-inserted value, move right
    		loop {
    			match self[self[current].next].value {
    				None => {
    					// done moving right
    					break;
    				},
    				Some(ref val_ahead) => {
    					// check value
    					if val_ahead < &t {
    						// move right
    						current = self[current].next;
    					} else {
    						break;
    					};
    				},
    			};
    		};

    		// if there is lower to go, go lower
    		if level > 0 {
    			// move lower
    			current = self[current].below;
    		}
    	}

    	// Now "current" is at level 0, and current.next is where new node will go:
    	assert!(self[current].below == Pointer::null());

    	// this is how many layers tall the new node will be
    	let height = self.rand_level();

    	// create the first node on level 0
    	let mut new_node = Pointer::null();
    	let mut old_node = Pointer(self.slab.insert(
    		Node {
			    value: Some(t.clone()),
			    next: self[current].next, 
			    prev: current, 
			    above: Pointer::null(),
			    below: Pointer::null(),
        	}	
        ));

        // attach sides to "old_node"
        let right = self[current].next;
		self[right].prev = old_node;
        self[current].next = old_node;

    	// make a new node for each level of height over 0
    	for level in range(1, height + 1) {
    		
	    	// find where the node goes horiontally in this level

	    	// the starting node
    		current = self.headnodes[level];
	        
	        // if there is no value node in the level we don't search horiontally
 			// head pointing to a tail means no value node in level
			match self[self[current].next].value {
				Some(_) => {}, // nothing to do, value exists
				None => {
					// This case means there is only a head and tail node on this level:
					// make the new node for this level
					new_node = Pointer(self.slab.insert(
			    		Node {
						    value: Some(t.clone()),
						    next: self[current].next, 
						    prev: current, 
						    above: Pointer::null(),
						    below: old_node,
			        	}	
			        ));

					// assign pointers
					let right = self[current].next;
					self[right].prev = new_node; 
					self[current].next = new_node;
					self[old_node].above = new_node;
					
					old_node = new_node;
					continue;
				}
			}
    			
	        // while the next node has a value and the value
    		// is less than the to-be-inserted value, move right
    		loop {
    			match self[current].value {
    				None => {
    					// done moving right
    					break;
    				},
    				Some(ref cur_val) => {
    					// check value
    					if (cur_val < &t) {
    						// move right
    						current = self[current].next;
    					};
    				},
    			};
    		};

    		// current node is now in position:
	        // make the new node for this level
			new_node = Pointer(self.slab.insert(
	    		Node {
				    value: Some(t.clone()),
				    next: self[current].next, 
				    prev: current, 
				    above: Pointer::null(),
				    below: old_node,
	        	}	
	        ));

			// assign pointers
			let right = self[current].next;
			self[right].prev = new_node; 
			self[current].next = new_node;
			self[old_node].above = new_node;
			
			old_node = new_node;
    	}       
    }
    
    // Inserts a new element beginning at the back of the list.
    fn push_back(&mut self, t: T) {
    	// the starting node
    	let mut current = self.tailnodes[MAXHEIGHT-1];

    	// for each level from 15 -> 0
    	for level in range(0, MAXHEIGHT).rev() {

    		// if there is no value node in the level, we must force it to skip
    		match self[current].value {
    			Some(_) => { }, // nothing to do
    			None => {
    				match self[self[current].prev].value {
    					Some(_) => { }, // nothing to do
    					None => {
    						// No value in this level, skip
    						// if there is lower to go, go lower
				    		if level > 0 {
				    			// move lower
				    			current = self[current].below;
				    		};
    						continue;
    					}
    				}
    			}
    		}

    		// while the next node has a value and the value
    		// is less than the to-be-inserted value, move right
    		loop {
    			match self[self[current].prev].value {
    				None => {
    					// done moving right
    					break;
    				},
    				Some(ref val_ahead) => {
    					// check value
    					if val_ahead > &t {
    						// move right
    						current = self[current].prev;
    					} else {
    						break;
    					};
    				},
    			};
    		};

    		// if there is lower to go, go lower
    		if level > 0 {
    			// move lower
    			current = self[current].below;
    		}
    	}

    	// Now "current" is at level 0, and current.next is where new node will go:
    	assert!(self[current].below == Pointer::null());

    	// this is how many layers tall the new node will be
    	let height = self.rand_level();

    	// create the first node on level 0
    	let mut new_node = Pointer::null();
    	let mut old_node = Pointer(self.slab.insert(
    		Node {
			    value: Some(t.clone()),
			    prev: self[current].prev, 
			    next: current, 
			    above: Pointer::null(),
			    below: Pointer::null(),
        	}	
        ));

        // attach sides to "old_node"
        let left = self[current].prev;
		self[left].next = old_node;
        self[current].prev = old_node;

    	// make a new node for each level of height over 0
    	for level in range(1, height + 1) {
    		
	    	// find where the node goes horiontally in this level

	    	// the starting node
    		current = self.tailnodes[level];
	        
	        // if there is no value node in the level we don't search horiontally
 			// head pointing to a tail means no value node in level
			match self[self[current].prev].value {
				Some(_) => {}, // nothing to do, value exists
				None => {
					// This case means there is only a head and tail node on this level:
					// make the new node for this level
					new_node = Pointer(self.slab.insert(
			    		Node {
						    value: Some(t.clone()),
						    prev: self[current].prev, 
						    next: current, 
						    above: Pointer::null(),
						    below: old_node,
			        	}	
			        ));

					// assign pointers
					let left = self[current].prev;
					self[left].next = new_node; 
					self[current].prev = new_node;
					self[old_node].above = new_node;
					
					old_node = new_node;
					continue;
				}
			}
    			
	        // while the next node has a value and the value
    		// is less than the to-be-inserted value, move right
    		loop {
    			match self[current].value {
    				None => {
    					// done moving right
    					break;
    				},
    				Some(ref cur_val) => {
    					// check value
    					if (cur_val > &t) {
    						// move right
    						current = self[current].prev;
    					};
    				},
    			};
    		};

    		// current node is now in position:
	        // make the new node for this level
			new_node = Pointer(self.slab.insert(
	    		Node {
				    value: Some(t.clone()),
				    prev: self[current].prev, 
				    next: current, 
				    above: Pointer::null(),
				    below: old_node,
	        	}	
	        ));

			// assign pointers
			let left = self[current].prev;
			self[left].next = new_node; 
			self[current].prev = new_node;
			self[old_node].above = new_node;
			
			old_node = new_node;
    	}
    }

 	// sample a level from a geometric distribution of p = 0.5
    fn rand_level(&self) -> usize {
    	let mut r = rand::StdRng::new().unwrap();
		let n = Geometric::new(0.5).unwrap(); 
		return (((n.sample::<StdRng>(&mut r) as usize) - 1) % MAXHEIGHT); // % is to limit height to maxheight
	}
}

impl<T: fmt::Debug> fmt::Debug for SkipList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	for level in range(0, MAXHEIGHT).rev() {
	        let mut n = self.headnodes[level];
        	let mut first = true;	        
	        write!(f, "level: {:?} (", level)?;
	        while !n.is_null() {
	            if !first {
	                write!(f, ", ")?;
	            }
	            first = false;
	            
	            match self[n].value {
	            	Some(ref val) => {
	            		write!(f, "{:?}", val)?;
	            	},
	            	None => {
	            		match self[n].next {
	            			Pointer(ref val) => {
	            				if *val == !0 as usize {
	            					write!(f, "Tail")?;
	            				} else {
	            					write!(f, "Head")?;
	            				};
	            			},
	            		};
	            		
	            	}
	            }
	            
	            n = self[n].next;
	        }
	        write!(f, ")\n")?;
	    }
	    Ok(())

    }
}

fn main() {
	let mut s0 = SkipList::new();

	s0.push_back(5);
	s0.push_back(6);
	s0.push_back(1);
	s0.push_back(10);

	let mut sl = SkipList::new();

	sl.push_front(5);
	sl.push_front(6);
	sl.push_front(1);
	sl.push_front(10);


	println!("{:?}", s0);
	println!("{:?}", sl);
	println!("Empty: {:?}, Length: {}", sl.is_empty(), sl.len());

	sl.push_front(6);
	sl.push_front(3);
	sl.push_front(8);
	sl.push_front(100);
	sl.push_front(64);
	sl.push_front(6);
	sl.push_front(105);
	sl.push_front(19);
	sl.push_front(54);
	sl.push_front(652);
	sl.push_front(11);
	sl.push_front(101);

	println!("{:?}", sl);
	println!("Empty: {:?}, Length: {}", sl.is_empty(), sl.len());

}