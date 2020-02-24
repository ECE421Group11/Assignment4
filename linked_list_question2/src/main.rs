use im::list::{List, cons};

#[derive(Debug, PartialEq)]
pub struct LinkedList<T> {
	list: List<T>,
}

impl<T> LinkedList<T> {
	// return an empty linked list
	pub fn empty() -> LinkedList<T> {
		return LinkedList { list: List::new() };
	}

	// creates a new linked list with value t
	pub fn new(t:T) -> LinkedList<T> {
		return LinkedList { list: cons(t, List::new()) }
		// Head(t, Box::new(LinkedList::empty()))
	}

	// prepend t to self
	pub fn push(self, t:T) -> LinkedList<T> {
		return LinkedList { list: cons(t, self.list) }
		// Head(t, Box::new(self))
	}

	// append t to self
	// (recursively search for tail node and replace with real node)
	pub fn push_back(self, t:T) -> Self {
		return LinkedList { list: self.list.append(List::singleton(t)) }
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn it_works(){
		let mut l = LinkedList::new(3);

		l = l.push(4);
		assert_eq!(l.list, List::range(3,4).reverse()); // list should be [4, 3]

		l= l.push_back(2);
		assert_eq!(l.list, List::range(2,4).reverse()); // list should be [4, 3, 2]
	}
}

fn main(){
	
}