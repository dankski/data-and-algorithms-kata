pub mod single;

#[derive(Debug)]
pub struct Node {
    pub elem: i32,
    pub next: Link,
}

pub type Link = Option<Box<Node>>;
