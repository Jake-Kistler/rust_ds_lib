use pyo3::prelude::*;

mod vector;
mod stack;

use vector::Vector;
use stack::Stack;

#[pymodule]
fn rust_ds_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vector>()?;
    m.add_class::<Stack>()?;
    Ok(())
}
