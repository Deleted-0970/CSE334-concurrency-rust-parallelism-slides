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
        let new_head = self.head.take();

        self.head = Some(Box::new(LinkedListNode {
            next: new_head,
            data,
        }));
    }

    fn push_back(&mut self, data: T) {
        let current = &mut self.head;
        if current.is_none() {
            self.head = Some(Box::new(LinkedListNode { data, next: None }));
            return;
        }
        let mut current = current.as_mut().expect("i know i'm right");
        loop {
            match &mut current.next {
                Some(q) => {
                    current = q;
                },
                None => {
                    current.next = Some(Box::new(LinkedListNode { data, next: None }));
                    break;
                },
            }
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
