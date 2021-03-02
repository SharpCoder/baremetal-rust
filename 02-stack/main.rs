#![allow(dead_code)]
pub mod mem;

#[derive(Copy, Clone)]
struct Node<T : Copy> {
    item: T,
    next: Option<*mut Node<T>>,
}

pub struct Stack<T : Copy> {
    head: Option<*mut Node<T>>,
    size: usize,
}

impl <T : Copy> Stack<T> {
    pub fn new() -> Self {
        return Stack { head: None, size: 0 };
    }

    pub fn push(&mut self, item: T) {
        let ptr = mem::kalloc();
        unsafe {
            (*ptr) = Node {
                item: item,
                next: self.head,
            };
        }

        self.head = Some(ptr);
        self.size = self.size + 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head {
            None => {
                return None;
            },
            Some(node) => {
                // Copy the reference
                let indirect = node.clone();
                let node_item = unsafe { *indirect };

                let result = node_item.item;
                self.head = node_item.next;
                self.size = self.size - 1;
                return Some(result);
            },
        };  
    }
}

#[cfg(test)]
mod test { 

    use super::*;

    #[test]
    fn stack() {
        let mut list = Stack::new();
        list.push(128);
        list.push(256);

        assert_eq!(list.pop(), Some(256));
        assert_eq!(list.pop(), Some(128));
        assert_eq!(list.pop(), None);
    }

}