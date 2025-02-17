use pyo3::prelude::*;

/// A simple Rust vector wrapper for Python
#[pyclass]
pub struct Vector {
    data: Vec<i32>,
}

#[pymethods]
impl Vector {
    #[new]
    pub fn new() -> Self {
        Vector { data: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        self.data.get(index).copied()
    }
}
