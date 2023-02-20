use crate::list::Link;
use crate::list::Node;

pub struct List {
    head: Link,
}

impl List {

    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn insert(&mut self, val: i32) {
        let node = Box::new(Node {elem: val, next: self.head.take()});
        self.head = Some(node);
    }

    pub fn search(&self, needle: i32) -> Option<&Box<Node>> {
    
        let mut cur = self.head.as_ref();

        while cur.is_some() == true {

            if cur.unwrap().elem == needle {
                return cur.clone();
            }

            cur = cur.unwrap().next.as_ref();
        }

        return None;
    }

    pub fn predecessor(node: Option<&Box<Node>>, needle: i32) {
        
    }

    pub fn print(&self) {
        let mut cur = self.head.as_ref();
        while cur.is_some() == true {
            print!(" {} ", cur.unwrap().elem);
            cur = cur.unwrap().next.as_ref();
        } 
    }
}