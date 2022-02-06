#[macro_use]
use numpy::{ToPyArray, PyArray};

use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::*;


pub struct CategoryToIndices {
    pad_mst: bool,
    mapper: HashMap<String, i32>,
}

impl CategoryToIndices {
    pub fn new(arr: &Vec<String>, pad_mst: bool) -> Self {
        let mapper = Self::create_mapper(arr);
        CategoryToIndices {
            pad_mst,
            mapper,
        }
    }

    pub fn create_mapper(arr: &Vec<String>) -> HashMap<String, i32> {
        let mut mapper = HashMap::new();
        for (i, v) in arr.iter().enumerate() {
            mapper.insert(v.to_string(), i as i32);
        }
        mapper
    }

    pub fn len(&self) -> usize {
        self.mapper.len()
    }

    pub fn get_int(&self, keyword: &str) -> i32 {
        if self.pad_mst {
            if self.mapper.contains_key(keyword) {
                *self.mapper.get(keyword).unwrap()
            } else {
                0
            }
        } else {
            *self.mapper.get(keyword).unwrap()
        }
    }
}

#[pyclass]
pub struct Category{
    #[pyo3(get)]
    pad_mst: bool,
    i2c: Vec<String>,
    c2i: CategoryToIndices,
}

#[pymethods]
impl Category{
    #[new]
    pub fn new(arr: &PyList, pad_mst: bool) -> Self {
        let mut i2c: Vec<String>;
        if pad_mst {
            i2c = vec!["[MST]".to_string()];
            i2c.extend(arr.iter().map(|x| x.to_string()));
        } else {
            i2c = arr.iter().map(|x| x.to_string()).collect();
        }
            
        let c2i = CategoryToIndices::new(&i2c, pad_mst);
        Category {
            pad_mst,
            i2c,
            c2i,
        }
    }

    pub fn categories_to_indices(&self, arr: &PyList) -> PyResult<Vec<i32>> {
        let mut res = Vec::new();
        for v in arr.iter() {
            res.push(self.c2i.get_int(v.to_string().as_str()));
        }
        Ok(res)
    }

    pub fn indices_to_categories(&self, arr: &PyList) -> PyResult<Vec<String>> {
        let mut res = Vec::new();
        // extract each element as integer
        let arr_vec: Vec<i32> = arr.iter().map(|x| x.extract::<i32>().unwrap()).collect();
        for v in arr_vec {
            res.push(self.i2c[v as usize].clone());
        }
        Ok(res)
    }

    pub fn category_to_onehot(&self, category: &str) -> Vec<f32> {
        let mut res = vec![0.0; self.c2i.len()];
        res[self.c2i.get_int(category) as usize] = 1.0;
        res
    }

    pub fn categories_to_onehot(&self, arr: &PyList) -> PyResult<Vec<Vec<f32>>> {
        let mut res = Vec::new();
        for v in arr {
            res.push(self.category_to_onehot(v.to_string().as_str()));
        }
        Ok(res)
    }

    pub fn onehot_to_category(&self, arr: &PyList) -> PyResult<String> {
        // find 1 in arr and return the corresponding category
        let mut i = 0;
        for v in arr {
            if v.extract::<f32>().unwrap() == 1.0 {
                return Ok(self.i2c[i].clone());
            }
            i += 1;
        }
        panic!("onehot_to_category: no hot in the encoded");
    }
}

