use pyo3::prelude::*;

/// A node in the stack
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

/// A stack implemented as a linked list
#[pyclass]
pub struct Stack {
    head: Option<Box<Node>>,
    size: usize,
}

#[pymethods]
impl Stack {
    #[new]
    pub fn new() -> Self {
        Stack { head: None, size: 0 }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.value
        })
    }

    pub fn peek(&self) -> Option<i32> {
        self.head.as_ref().map(|node| node.value)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}
