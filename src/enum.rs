pub struct List {
    head: Link,
}

enum Link {
    Empty,
    Next(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::Next(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::Next(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::Next(mut node) = cur {
            cur = std::mem::replace(&mut node.next, Link::Empty);
        }
    }
}
