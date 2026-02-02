//! Run this file with `cargo test --test 09c_linked_list`.

struct LinkedListNode<T> {
    data: T,
    next: Option<Box<LinkedListNode<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<LinkedListNode<T>>>,
}

impl<T> LinkedList<T> {
    fn push_front(&mut self, data: T) {
        let mut node = Some(Box::new(LinkedListNode {
            next: self.head.take(),
            data,
        }));
        self.head = node;
        // std::mem::swap(&mut self.head, &mut node);
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
