mod single;
use single::{Category};
use pyo3::prelude::*;
use pyo3::types::*;

#[pymodule]
fn rust_category(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Category>()?;
    Ok(())
}

pub fn main(){

}