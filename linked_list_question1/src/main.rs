#[derive(Debug, PartialEq)]

pub enum LinkedList<T>{
	Tail,
	Head(T,Box<LinkedList<T>>),
}
use self::LinkedList::*;

impl<T> LinkedList<T>{
	//add your code here:
	pub fn empty() -> Self {
		return Tail;
	}

	pub fn new(t:T) -> Self {
		return Head(t, Box::new(Tail))
	}

	pub fn push(self, t:T) -> Self {
		return Head(t, Box::new(self));
	}

	pub fn push_back(self, t:T) -> Self {
		match self{
			Tail => Head(t, Box::new(Tail)),
			Head(val, list) => Head(val, Box::new(list.push_back(t))),
		}
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn it_works(){
		let mut l = LinkedList::new(3);
		l = l.push(4);
		assert_eq!(l,Head(4,Box::new(Head(3,Box::new(Tail)))));
		
		l = l.push_back(2);
		assert_eq!(l,Head(4,Box::new(Head(3,Box::new(Head(2,Box::new(Tail)))))));
	}
}
fn main(){
	let mut l = LinkedList::new(3);
	l = l.push(4);
	l = l.push(6);
	l = l.push(5);
	println!("{:?}", l);
}