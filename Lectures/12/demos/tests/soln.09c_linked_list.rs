//! Run this file with `cargo test --test 09c_linked_list`.

struct LinkedListNode<T> {
    data: T,
    next: Option<Box<LinkedListNode<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<LinkedListNode<T>>>,
}

impl<T> LinkedList<T> {
    fn push_front(&mut self, d: T) {
        // what we'd like (doesn't work bc self.head would have no value after the let!)
        let old_head = self.head;
        self.head = None;
        let new = LinkedListNode {
            data: d,
            next: old_head,
        };

        // what we need
        //let mut old_head = None;
        //std::mem::swap(&mut old_head, &mut self.head);
        //let new = LinkedListNode {data: d, next: old_head};
        // let new = LinkedListNode {data: d, next: std::mem::replace(&mut self.head, None)};
        // let new = LinkedListNode {data: d, next: self.head.take()};
        // let new = LinkedListNode {data: d, next: std::mem::take(&mut self.head)};
        self.head = Some(Box::new(new));
    }
    fn push_back(&mut self, d: T) {
        let new = Some(Box::new(LinkedListNode {
            data: d,
            next: None,
        }));
        let mut cur = self.head.as_mut();
        if cur.is_none() {
            self.head = new;
            return;
        }
        while cur.is_some() {
            let inner_node = cur.expect("we know cur isn't none!");

            if inner_node.next.is_none() {
                inner_node.next = new;
                break;
            }
            cur = inner_node.next.as_mut();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;

    #[test]
    fn make_list() {
        let mut ll = LinkedList { head: None };
        ll.push_back("world");
        ll.push_front("hello");
    }
}
