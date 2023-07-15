use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct GlobalStack<T>(Rc<RefCell<Vec<T>>>);

impl<T> GlobalStack<T> {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(Vec::new())))
    }

    fn push(&self, value: T) {
        self.0.borrow_mut().push(value);
    }

    fn pop(&self) -> Option<T> {
        self.0.borrow_mut().pop()
    }

    fn len(&self) -> usize {
        self.0.borrow().len()
    }
}

fn main() {
    let stack = GlobalStack::new();
    stack.push(1);
    stack.pop();
    println!("stack len: {}", stack.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    mod push {
        use super::*;

        #[test]
        fn should_successfully_add_value_by_shared_reference() {
            let stack = GlobalStack::new();
            stack.push(1);
            assert_eq!(stack.len(), 1);
        }
    }

    mod pop {
        use super::*;

        #[test]
        fn should_successfully_remove_value_by_shared_reference() {
            let stack = GlobalStack::new();

            stack.push(1);
            let value = stack.pop().unwrap();

            assert_eq!(value, 1);
            assert_eq!(stack.len(), 0);
        }
    }

    mod clone {
        use super::*;

        #[test]
        fn produces_pointer_so_multiple_owners_mutate_the_same_data() {
            let stack = GlobalStack::new();
            stack.push(1);

            let stack_clone = stack.clone();
            stack_clone.push(2);

            assert_eq!(stack.len(), 2);
            let value = stack.pop().unwrap();
            assert_eq!(value, 2);
        }
    }
}
