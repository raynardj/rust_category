use pyo3::prelude::*;
use std::collections::HashMap;

pub struct CategoryToIndices {
    pad_mst: bool,
    mapper: HashMap<String, i32>,
}

impl CategoryToIndices {
    pub fn new(arr: &Vec<String>, pad_mst: bool) -> Self {
        let mapper = Self::create_mapper(arr);
        CategoryToIndices { pad_mst, mapper }
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
        let option = self.mapper.get(keyword);
        match option {
            Some(v) => *v,
            None => {
                if self.pad_mst {
                    0
                } else {
                    -1
                }
            }
        }
    }
}

#[pyclass]
pub struct Category {
    #[pyo3(get)]
    pad_mst: bool,
    i2c: Vec<String>,
    pub c2i: CategoryToIndices,
}

#[pymethods]
impl Category {
    #[new]
    pub fn new(arr: Vec<String>, pad_mst: bool) -> Self {
        let mut i2c: Vec<String>;
        if pad_mst {
            i2c = vec!["[MST]".to_string()];
            i2c.extend(arr.iter().map(|x| x.to_string()));
        } else {
            i2c = arr.iter().map(|x| x.to_string()).collect();
        }
        let c2i = CategoryToIndices::new(&i2c, pad_mst);
        Category { pad_mst, i2c, c2i }
    }

    pub fn categories_to_indices(&self, arr: Vec<String>) -> Vec<i32> {
        let mut res = Vec::new();
        for v in arr.iter() {
            res.push(self.c2i.get_int(v));
        }
        res
    }

    pub fn string_to_indices(&self, inputs: String, spliter: &str) -> Vec<i32> {
        let mut res = Vec::new();
        for v in inputs.split(spliter) {
            res.push(self.c2i.get_int(v));
        }
        res
    }

    pub fn indices_to_categories(&self, arr: Vec<i32>) -> Vec<String> {
        let mut res = Vec::new();
        // extract each element as integer
        for &v in arr.iter() {
            res.push(self.i2c[v as usize].clone());
        }
        res
    }

    pub fn category_to_onehot(&self, category: &str) -> Vec<f32> {
        let mut res = vec![0.0; self.c2i.len()];
        res[self.c2i.get_int(category) as usize] = 1.0;
        res
    }

    pub fn categories_to_onehot(&self, arr: Vec<String>) -> Vec<Vec<f32>> {
        let mut res = Vec::new();
        let empty = vec![0.0; self.c2i.len()];

        let indices = self.categories_to_indices(arr);
        for &i in indices.iter() {
            let mut tmp = empty.clone();
            tmp[i as usize] = 1.0;
            res.push(tmp);
        }
        res
    }

    pub fn onehot_to_category(&self, arr: Vec<f32>) -> String {
        // find 1 in arr and return the corresponding category
        let mut i = 0;
        for &v in arr.iter() {
            if v == 1.0 {
                return self.i2c[i].clone();
            }
            i += 1;
        }
        panic!("onehot_to_category: no hot in the encoded");
    }
}
