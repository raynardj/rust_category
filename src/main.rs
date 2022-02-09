mod single;
use single::Category;
mod multi;
use multi::MultiCategory;
use pyo3::prelude::*;

#[pymodule]
fn rust_category(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Category>()?;
    m.add_class::<MultiCategory>()?;
    Ok(())
}

pub fn main(){

}