use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use xid::Id;

type NodeId = Id;

struct Node<T> {
    id: NodeId,
    item: T,
    prev: Option<NodeId>,
    next: Option<NodeId>,
}

impl<T> Node<T> {
    fn new(item: T) -> Self {
        Self {
            id: xid::new(),
            item,
            prev: None,
            next: None,
        }
    }
}

struct _Inner<T> {
    head: Option<NodeId>,
    tail: Option<NodeId>,
    all_nodes: BTreeMap<NodeId, Node<T>>,
}

impl<T> _Inner<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            all_nodes: BTreeMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct DoublyLinkedList<T>(Arc<Mutex<_Inner<T>>>);

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(_Inner::new())))
    }

    fn len(&self) -> usize {
        self.0.lock().all_nodes.len()
    }

    fn push_back(&self, item: T) {
        let mut new_node = Node::new(item);

        let mut inner = self.0.lock();

        match inner.tail {
            Some(prev_tail) => {
                let prev_tail = inner.all_nodes.get_mut(&prev_tail).unwrap();
                prev_tail.next = Some(new_node.id);
                new_node.prev = Some(prev_tail.id);
                inner.tail = Some(new_node.id);
            }
            None => {
                inner.head = Some(new_node.id);
                inner.tail = Some(new_node.id);
            }
        }

        inner.all_nodes.insert(new_node.id, new_node);
    }

    fn push_front(&self, item: T) {
        let mut new_node = Node::new(item);

        let mut inner = self.0.lock();

        match inner.head {
            Some(prev_head) => {
                let prev_head = inner.all_nodes.get_mut(&prev_head).unwrap();
                prev_head.prev = Some(new_node.id);
                new_node.next = Some(prev_head.id);
                inner.head = Some(new_node.id);
            }
            None => {
                inner.head = Some(new_node.id);
                inner.tail = Some(new_node.id);
            }
        }

        inner.all_nodes.insert(new_node.id, new_node);
    }

    fn pop_back(&self) -> Option<T> {
        let mut inner = self.0.lock();

        inner.tail.take().map(|prev_tail| {
            let prev_tail = inner.all_nodes.remove(&prev_tail).unwrap();

            match prev_tail.prev {
                Some(node) => {
                    let node = inner.all_nodes.get_mut(&node).unwrap();
                    node.next = None;
                    inner.tail = Some(node.id);
                }
                None => {
                    inner.head.take();
                }
            }

            prev_tail.item
        })
    }

    fn pop_front(&self) -> Option<T> {
        let mut inner = self.0.lock();

        inner.head.take().map(|prev_head| {
            let prev_head = inner.all_nodes.remove(&prev_head).unwrap();

            match prev_head.next {
                Some(node) => {
                    let node = inner.all_nodes.get_mut(&node).unwrap();
                    node.prev = None;
                    inner.head = Some(node.id);
                }
                None => {
                    inner.tail.take();
                }
            }

            prev_head.item
        })
    }
}

fn main() {
    println!("hello");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn should_successfully_mutate_dll_on_a_single_thread() {
        let mut list = DoublyLinkedList::new();
        assert_eq!(list.len(), 0);

        list.push_back(5);
        list.push_front(7);
        list.push_back(3);
        assert_eq!(list.len(), 3);

        assert_eq!(list.pop_back().unwrap(), 3);
        assert_eq!(list.len(), 2);

        assert_eq!(list.pop_front().unwrap(), 7);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn should_successfully_mutate_dll_on_multiple_threads() {
        let mut list = DoublyLinkedList::new();
        list.push_back(5);

        thread::scope(|s| {
            s.spawn(|| {
                list.clone().push_front(3);
                list.clone().push_back(7);
            });
        });

        assert_eq!(list.len(), 3);

        thread::scope(|s| {
            s.spawn(|| {
                assert_eq!(list.clone().pop_back().unwrap(), 7);
                assert_eq!(list.clone().pop_front().unwrap(), 3);
            });
        });

        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_back().unwrap(), 5);
    }
}
