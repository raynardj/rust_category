use crate::single::Category;
use pyo3::prelude::*;

#[pyclass]
pub struct MultiCategory {
    #[pyo3(get)]
    pad_mst: bool,
    category: Category,
    spliter: String,
    length: usize,
}

#[pymethods]
impl MultiCategory {
    #[new]
    pub fn new(arr: Vec<String>, pad_mst: bool, spliter: String) -> Self {
        let category = Category::new(arr, pad_mst);
        let length = category.c2i.len();
        MultiCategory {
            pad_mst,
            category,
            spliter,
            length: length,
        }
    }

    pub fn categories_to_indices(&self, arr: Vec<Option<String>>) -> Vec<Vec<u32>> {
        /*
        Take in a list of string
        A string is a row of data
        split each string by spliter character
        convert each splited and stripped to integer
        Return a list of list of integer
        */
        let mut res = Vec::new();
        for v in arr.iter() {
            let mut tmp = Vec::new();
            match v {
                Some(v) => {
                    let v_str = v.to_string();
                    for cate in v_str.split(&self.spliter).map(|x| x.trim()) {
                        tmp.push(self.category.c2i.get_int(cate));
                    }
                    res.push(tmp);
                }
                None => {
                    // empty vector
                    res.push(vec![]);
                }
            }
        }
        res
    }

    pub fn categories_to_nhot(&self, arr: Vec<Option<String>>) -> Vec<Vec<f32>> {
        /*
        Take in a list of string
        A string is a row of data
        split each string by spliter character
        convert each splited and stripped to integer
        Return a list of list of integer, which is the nhot encoding
        */
        let mut res = Vec::new();
        let zero_vec = vec![0.0; self.length];
        for v in arr.iter() {
            let mut tmp = zero_vec.clone();
            match v {
                Some(v) => {
                    let v_str = v.to_string();
                    for cate in v_str.split(&self.spliter).map(|x| x.trim().to_string()) {
                        tmp[self.category.c2i.get_int(&cate) as usize] = 1.0;
                    }
                    res.push(tmp);
                }
                None => {
                    // empty vector
                    res.push(zero_vec.clone());
                }
            }
        }
        res
    }
}
