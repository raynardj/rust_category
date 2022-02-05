mod single;
#[cfg(test)]
mod tests {
    use crate::single::{Category, CategoryToIndices};
    #[test]
    pub fn test_category_to_indices() {
        let arr = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let c2i = CategoryToIndices::new(&arr, false);

        assert_eq!(c2i.get_int("a"), 0);
        assert_eq!(c2i.get_int("b"), 1);
        assert_eq!(c2i.get_int("c"), 2);
    }
    #[test]
    pub fn test_category() {
        let arr = vec![
            "alpha".to_string(),
            "beta".to_string(),
            "charlie".to_string(),
            "delta".to_string(),
        ];
        let c = Category::new(&arr, false);
        let test_array = vec![vec![
            "alpha".to_string(),
            "charlie".to_string(),
            "delta".to_string(),
            "charlie".to_string(),
        ]];
        println!("[test] Category.categories_to_indices (pad_mst = false)");
        assert_eq!(c.categories_to_indices(&test_array[0]), vec![0, 2, 3, 2]);

        let c2 = Category::new(&arr, true);
        let test_array = vec![vec![
            "alpha".to_string(),
            "charlie".to_string(),
            "delta".to_string(),
            "e".to_string(),
        ]];
        println!("[test] Category.categories_to_indices (pad_mst = true)");
        assert_eq!(c2.categories_to_indices(&test_array[0]), vec![1, 3, 4, 0]);

        println!("[test] Category.indices_to_categories (pad_mst = false)");
        assert_eq!(
            c2.indices_to_categories(&c2.categories_to_indices(&test_array[0])),
            vec!["alpha".to_string(), "charlie".to_string(), "delta".to_string(), "[MST]".to_string()]
        );

        println!("[test] [past] Category]");
    }
}
