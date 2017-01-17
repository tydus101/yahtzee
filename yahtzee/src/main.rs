use std::thread;

struct Node {
	c1: Option<Box<Node>>,
	c2: Option<Box<Node>>,
	value: i32,
	itt: i32,
	thread: thread,

}

impl Node {
	fn do_work(&mut self) {
		thread::spawn
		self.value = self.value * 2;
		println!("Did work.") ;
	}

	fn create_children(&mut self) {
		let n = self.itt/2;
		self.c1 = Some(Box::new(Node::new(n)));
		self.c2 = Some(Box::new(Node::new(n)));
		if n > 2 {
			self.c1.as_mut().unwrap().create_children();
			self.c2.as_mut().unwrap().create_children();
		}
		else {
			self.c1.as_mut().unwrap().do_work();
			self.c2.as_mut().unwrap().do_work();
		}
	}
	fn new(itt: i32) -> Node{
		let child = thread::spawn(move || {
    		println!("Spawned child thread!");
		});
		Node {
			c1: None,
			c2: None,
			value: 0,
			itt: itt,
			thread: child,
		}

	}
}




fn main() {
	let mut n = Node::new(160000);
	n.create_children();
    println!("{}", n.c1.unwrap().c1.unwrap().itt);
}
