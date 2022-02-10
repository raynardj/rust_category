use hashbrown::HashMap;
use pyo3::prelude::*;

pub struct CategoryToIndices {
    pad_mst: bool,
    mapper: HashMap<String, u32>,
}

impl CategoryToIndices {
    pub fn new(arr: &Vec<String>, pad_mst: bool) -> Self {
        let mapper = Self::create_mapper(arr);
        CategoryToIndices { pad_mst, mapper }
    }

    pub fn create_mapper(arr: &Vec<String>) -> HashMap<String, u32> {
        let mut mapper = HashMap::with_capacity(arr.len());
        for (i, v) in arr.iter().enumerate() {
            mapper.insert(v.to_string(), i as u32);
        }
        mapper
    }

    pub fn __str__(&self) -> String {
        format!("CategoryToIndices: Length{:?}", self.mapper.len())
    }

    pub fn __repr__(&self) -> String {
        format!("{}", self.__str__())
    }

    pub fn __len__(&self) -> usize {
        self.mapper.len()
    }

    pub fn len(&self) -> usize {
        self.mapper.len()
    }

    pub fn get_int(&self, keyword: &str) -> u32 {
        /* from category string to index */
        match self.mapper.get(keyword) {
            Some(v) => *v,
            None => {
                if self.pad_mst {
                    0
                } else {
                    panic!("[KeyError]'{}' not found", keyword);
                }
            }
        }
    }

    pub fn __call__(&self, keyword: &str) -> u32 {
        self.get_int(keyword)
    }

    pub fn __getitem__(&self, keyword: &str) -> u32 {
        self.get_int(keyword)
    }
}

#[pyclass]
pub struct Category {
    #[pyo3(get)]
    pad_mst: bool,
    #[pyo3(get)]
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

    pub fn categories_to_indices(&self, arr: Vec<String>) -> Vec<u32> {
        let mut res = Vec::with_capacity(arr.len());
        for cate in arr.iter() {
            res.push(self.c2i.get_int(cate));
        }
        res
    }

    pub fn string_to_indices(&self, inputs: String, spliter: &str) -> Vec<u32> {
        let mut res = Vec::new();
        for v in inputs.split(spliter) {
            res.push(self.c2i.get_int(v));
        }
        res
    }

    pub fn indices_to_categories(&self, arr: Vec<u32>) -> Vec<String> {
        let mut res = Vec::with_capacity(arr.len());
        // extract each element as integer
        for &v in arr.iter() {
            res.push(self.i2c[v as usize].clone());
        }
        res
    }

    pub fn category_to_onehot(&self, category: String) -> Vec<f32> {
        let mut res = vec![0.0; self.c2i.len()];
        res[self.c2i.get_int(&category) as usize] = 1.0;
        res
    }

    pub fn categories_to_onehot(&self, arr: Vec<String>) -> Vec<Vec<f32>> {
        let empty = vec![0.0; self.c2i.len()];
        let indices = self.categories_to_indices(arr);
        let mut res = Vec::with_capacity(indices.len());
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

    pub fn __len__(&self) -> usize {
        self.c2i.len()
    }

    pub fn __str__(&self) -> String {
        format!("Category: Length..{:?}", self.c2i.len())
    }

    pub fn __repr__(&self) -> String {
        format!("Category: Length..{:?}", self.c2i.len())
    }
}
