#[cfg(test)]
mod tests {
    use rustic_ml::data_utils::datacolumn::DataColumn;

    #[test]
    fn test_new() {
        let column: DataColumn<i32> = DataColumn::new(vec![Some(1), None, Some(3)]);
        assert_eq!(column.size(), 3);
        assert_eq!(column.data_type, "i32");
    }

    #[test]
    fn test_size() {
        let column: DataColumn<i32> = DataColumn::new(vec![Some(1), None, Some(3)]);
        assert_eq!(column.size(), 3);
    }

    #[test]
    fn test_none_count() {
        let column: DataColumn<i32> = DataColumn::new(vec![Some(1), None, Some(3)]);
        assert_eq!(column.none_count(), 1);
    }

    #[test]
    fn test_some_count() {
        let column: DataColumn<i32> = DataColumn::new(vec![Some(1), None, Some(3)]);
        assert_eq!(column.some_count(), 2);
    }

    #[test]
    fn test_get() {
        let column: DataColumn<i32> = DataColumn::new(vec![Some(1), None, Some(3)]);
        assert_eq!(column.get(0), Some(&1));
        assert_eq!(column.get(1), None);
        assert_eq!(column.get(2), Some(&3));
        assert_eq!(column.get(3), None);
        assert_eq!(column.get(1234), None);
    }

    #[test]
    fn test_set() {
        let mut column: DataColumn<i32> = DataColumn::new(vec![Some(1), None, Some(3)]);
        column.set(1, 5); // Setting index 1
        assert_eq!(column.get(1), Some(&5));
    }

    #[test]
    fn test_remove() {
        let mut column: DataColumn<i32> = DataColumn::new(vec![Some(1), Some(2), Some(3)]);
        column.remove(1); // Removing the value at index 1
        assert_eq!(column.get(1), None);
    }

    #[test]
    fn test_append() {
        let mut column: DataColumn<i32> = DataColumn::new(vec![Some(1), None]);
        column.append(10);
        assert_eq!(column.size(), 3);
        assert_eq!(column.get(2), Some(&10));
    }

    #[test]
    fn test_reset() {
        let mut column: DataColumn<i32> = DataColumn::new(vec![Some(1), Some(2), Some(3)]);
        column.reset();

        // All should be None
        assert!(column.iter_column().all(|x| x.is_none()));
    }

    #[test]
    fn test_reset_default() {
        let mut column: DataColumn<i32> = DataColumn::new(vec![Some(1), None, Some(3)]);
        column.reset_default();

        // All should be the default, which is Some(0) for Option<i32>
        assert!(column.iter_column().all(|x| x.is_some_and(|val| val == 0)));
    }
}
