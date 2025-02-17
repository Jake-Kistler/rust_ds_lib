use pyo3::prelude::*;

#[pyclass]
struct RustVector {
    data: Vec<i32>,
}

#[pymethods]
impl RustVector {
    #[new]
    fn new() -> Self {
        RustVector { data: Vec::new() }
    }

    fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn get(&self, index: usize) -> Option<i32> {
        self.data.get(index).copied()
    }
}

#[pymodule]
fn rust_ds_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RustVector>()?;
    Ok(())
}
