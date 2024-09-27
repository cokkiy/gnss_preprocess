use super::*;

#[test]
fn test_obs_file_item_iter() {
    let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    let obs_file_item = ObsFilesInDay::new(123, obs_files);

    let mut iter = obs_file_item.iter();
    assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file1.obs")));
    assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file2.obs")));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_obs_file_item_iter_empty() {
    let obs_files = Vec::new();
    let obs_file_item = ObsFilesInDay::new(123, obs_files);

    let mut iter = obs_file_item.iter();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_obs_file_item_iter_multiple_items() {
    let obs_files1 = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    let obs_file_item1 = ObsFilesInDay::new(123, obs_files1);

    let obs_files2 = vec!["file3.obs".to_string(), "file4.obs".to_string()];
    let obs_file_item2 = ObsFilesInDay::new(456, obs_files2);

    let mut iter = obs_file_item1.iter().chain(obs_file_item2.iter());
    assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file1.obs")));
    assert_eq!(iter.next(), Some(PathBuf::from("123/daily/file2.obs")));
    assert_eq!(iter.next(), Some(PathBuf::from("456/daily/file3.obs")));
    assert_eq!(iter.next(), Some(PathBuf::from("456/daily/file4.obs")));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_obs_files_tree_item_iter() {
    let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    let obs_file_item = ObsFilesInDay::new(123, obs_files);
    let obs_files_tree_item = ObsFilesInYear::new(2023, vec![obs_file_item]);

    let mut iter = obs_files_tree_item.iter();
    assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file1.obs")));
    assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file2.obs")));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_obs_files_tree_item_iter_empty() {
    let obs_files_tree_item = ObsFilesInYear::new(2023, Vec::new());

    let mut iter = obs_files_tree_item.iter();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_obs_files_tree_item_iter_multiple_items() {
    let obs_files1 = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    let obs_file_item1 = ObsFilesInDay::new(123, obs_files1);

    let obs_files2 = vec!["file3.obs".to_string(), "file4.obs".to_string()];
    let obs_file_item2 = ObsFilesInDay::new(456, obs_files2);

    let obs_files_tree_item = ObsFilesInYear::new(2023, vec![obs_file_item1, obs_file_item2]);

    let mut iter = obs_files_tree_item.iter();
    assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file1.obs")));
    assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file2.obs")));
    assert_eq!(iter.next(), Some(PathBuf::from("2023/456/daily/file3.obs")));
    assert_eq!(iter.next(), Some(PathBuf::from("2023/456/daily/file4.obs")));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_obs_files_tree_get_obs_files() {
    let obs_files1 = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    let obs_file_item1 = ObsFilesInDay::new(123, obs_files1);

    let obs_files2 = vec!["file3.obs".to_string(), "file4.obs".to_string()];
    let obs_file_item2 = ObsFilesInDay::new(456, obs_files2);

    let obs_files_tree_item1 = ObsFilesInYear::new(2023, vec![obs_file_item1]);
    let obs_files_tree_item2 = ObsFilesInYear::new(2024, vec![obs_file_item2]);

    let mut obs_files_tree = ObsFilesTree::new();
    obs_files_tree.add_item(obs_files_tree_item1);
    obs_files_tree.add_item(obs_files_tree_item2);

    let mut iter = obs_files_tree.get_obs_files();
    assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file1.obs")));
    assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file2.obs")));
    assert_eq!(iter.next(), Some(PathBuf::from("2024/456/daily/file3.obs")));
    assert_eq!(iter.next(), Some(PathBuf::from("2024/456/daily/file4.obs")));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_obs_files_tree_get_day_numbers() {
    let obs_files1 = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    let obs_file_item1 = ObsFilesInDay::new(123, obs_files1);

    let obs_files2 = vec!["file3.obs".to_string(), "file4.obs".to_string()];
    let obs_file_item2 = ObsFilesInDay::new(456, obs_files2);

    let obs_files_tree_item1 = ObsFilesInYear::new(2023, vec![obs_file_item1]);
    let obs_files_tree_item2 = ObsFilesInYear::new(2024, vec![obs_file_item2]);

    let mut obs_files_tree = ObsFilesTree::new();
    obs_files_tree.add_item(obs_files_tree_item1);
    obs_files_tree.add_item(obs_files_tree_item2);

    let day_numbers = obs_files_tree.get_day_numbers();
    assert_eq!(day_numbers, 2);
}

#[test]
fn test_obs_files_tree_get_day_numbers_empty() {
    let obs_files_tree = ObsFilesTree::new();

    let day_numbers = obs_files_tree.get_day_numbers();
    assert_eq!(day_numbers, 0);
}

#[test]
fn test_obs_files_tree_get_day_numbers_multiple_items() {
    let obs_files1 = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    let obs_file_item1 = ObsFilesInDay::new(123, obs_files1);

    let obs_files2 = vec!["file3.obs".to_string(), "file4.obs".to_string()];
    let obs_file_item2 = ObsFilesInDay::new(456, obs_files2);

    let obs_files3 = vec!["file5.obs".to_string(), "file6.obs".to_string()];
    let obs_file_item3 = ObsFilesInDay::new(789, obs_files3);

    let obs_files_tree_item1 = ObsFilesInYear::new(2023, vec![obs_file_item1]);
    let obs_files_tree_item2 = ObsFilesInYear::new(2024, vec![obs_file_item2, obs_file_item3]);

    let mut obs_files_tree = ObsFilesTree::new();
    obs_files_tree.add_item(obs_files_tree_item1);
    obs_files_tree.add_item(obs_files_tree_item2);

    let day_numbers = obs_files_tree.get_day_numbers();
    assert_eq!(day_numbers, 3);
}

#[test]
fn test_obs_files_tree_from_data() {
    let mut obs_data = HashMap::new();
    let mut day_files = HashMap::new();
    day_files.insert(123, vec!["file1.obs", "file2.obs"]);
    day_files.insert(456, vec!["file3.obs", "file4.obs"]);
    obs_data.insert(2023, day_files);

    let obs_files_tree = ObsFilesTree::from_data(obs_data);

    let mut iter = obs_files_tree.get_obs_files();
    assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file1.obs")));
    assert_eq!(iter.next(), Some(PathBuf::from("2023/123/daily/file2.obs")));
    assert_eq!(iter.next(), Some(PathBuf::from("2023/456/daily/file3.obs")));
    assert_eq!(iter.next(), Some(PathBuf::from("2023/456/daily/file4.obs")));
    assert_eq!(iter.next(), None);
}
#[test]
fn test_obs_files_tree_from_data_empty() {
    let obs_data = HashMap::new();

    let obs_files_tree = ObsFilesTree::from_data(obs_data);

    let mut iter = obs_files_tree.get_obs_files();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_obs_files_tree_from_data_empty_items() {
    let mut obs_data = HashMap::new();
    let day_files1 = HashMap::new();
    obs_data.insert(2023, day_files1);

    let day_files2 = HashMap::new();
    obs_data.insert(2024, day_files2);

    let obs_files_tree = ObsFilesTree::from_data(obs_data);

    let mut iter = obs_files_tree.get_obs_files();
    assert_eq!(iter.next(), None);
}
#[test]
fn test_obs_files_tree_from_data_empty_days() {
    let mut obs_data = HashMap::new();
    let mut day_files1 = HashMap::new();
    day_files1.insert(123, Vec::new());
    day_files1.insert(456, Vec::new());
    obs_data.insert(2023, day_files1);

    let mut day_files2 = HashMap::new();
    day_files2.insert(789, Vec::new());
    day_files2.insert(1011, Vec::new());
    obs_data.insert(2024, day_files2);

    let obs_files_tree = ObsFilesTree::from_data(obs_data);

    let mut iter = obs_files_tree.get_obs_files();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_obs_files_tree_from_data_empty_items_empty_days() {
    let mut obs_data = HashMap::new();
    let day_files1 = HashMap::new();
    obs_data.insert(2023, day_files1);

    let day_files2 = HashMap::new();
    obs_data.insert(2024, day_files2);

    let day_files3 = HashMap::new();
    obs_data.insert(2025, day_files3);

    let obs_files_tree = ObsFilesTree::from_data(obs_data);

    let mut iter = obs_files_tree.get_obs_files();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_obs_files_tree_from_data_empty_items_empty_days_empty_days() {
    let mut obs_data = HashMap::new();
    let day_files1 = HashMap::new();
    obs_data.insert(2023, day_files1);

    let day_files2 = HashMap::new();
    obs_data.insert(2024, day_files2);

    let day_files3 = HashMap::new();
    obs_data.insert(2025, day_files3);

    let day_files4 = HashMap::new();
    obs_data.insert(2026, day_files4);

    let obs_files_tree = ObsFilesTree::from_data(obs_data);

    let mut iter = obs_files_tree.get_obs_files();
    assert_eq!(iter.next(), None);
}
#[test]
fn test_obs_files_tree_from_data_multiple_items_empty_days_multiple_days_empty_days() {
    let mut obs_data = HashMap::new();
    let mut day_files1 = HashMap::new();
    day_files1.insert(123, Vec::new());
    day_files1.insert(456, Vec::new());
    obs_data.insert(2023, day_files1);

    let day_files2 = HashMap::new();
    obs_data.insert(2024, day_files2);

    let mut day_files3 = HashMap::new();
    day_files3.insert(789, Vec::new());
    day_files3.insert(1011, Vec::new());
    obs_data.insert(2025, day_files3);

    let mut day_files4 = HashMap::new();
    day_files4.insert(1213, vec!["file9.obs", "file10.obs"]);
    day_files4.insert(1415, vec!["file11.obs", "file12.obs"]);
    obs_data.insert(2026, day_files4);

    let day_files5 = HashMap::new();
    obs_data.insert(2027, day_files5);

    let obs_files_tree = ObsFilesTree::from_data(obs_data);

    let mut iter = obs_files_tree.get_obs_files();
    assert_eq!(
        iter.next(),
        Some(PathBuf::from("2026/1213/daily/file9.obs"))
    );
    assert_eq!(
        iter.next(),
        Some(PathBuf::from("2026/1213/daily/file10.obs"))
    );
    assert_eq!(
        iter.next(),
        Some(PathBuf::from("2026/1415/daily/file11.obs"))
    );
    assert_eq!(
        iter.next(),
        Some(PathBuf::from("2026/1415/daily/file12.obs"))
    );
    assert_eq!(iter.next(), None);
}
#[test]
fn test_obs_files_tree_from_data_empty_items_empty_days_multiple_days_empty_days() {
    let mut obs_data = HashMap::new();
    let day_files1 = HashMap::new();
    obs_data.insert(2023, day_files1);

    let day_files2 = HashMap::new();
    obs_data.insert(2024, day_files2);

    let mut day_files3 = HashMap::new();
    day_files3.insert(789, Vec::new());
    day_files3.insert(1011, Vec::new());
    obs_data.insert(2025, day_files3);

    let mut day_files4 = HashMap::new();
    day_files4.insert(1213, vec!["file9.obs", "file10.obs"]);
    day_files4.insert(1415, vec!["file11.obs", "file12.obs"]);
    obs_data.insert(2026, day_files4);

    let day_files5 = HashMap::new();
    obs_data.insert(2027, day_files5);

    let obs_files_tree = ObsFilesTree::from_data(obs_data);

    let mut iter = obs_files_tree.get_obs_files();
    assert_eq!(
        iter.next(),
        Some(PathBuf::from("2026/1213/daily/file9.obs"))
    );
    assert_eq!(
        iter.next(),
        Some(PathBuf::from("2026/1213/daily/file10.obs"))
    );
    assert_eq!(
        iter.next(),
        Some(PathBuf::from("2026/1415/daily/file11.obs"))
    );
    assert_eq!(
        iter.next(),
        Some(PathBuf::from("2026/1415/daily/file12.obs"))
    );
    assert_eq!(iter.next(), None);
}
#[test]
fn test_obs_files_tree_from_data_empty_items_multiple_days_empty_days_empty_days() {
    let mut obs_data = HashMap::new();
    let day_files1 = HashMap::new();
    obs_data.insert(2023, day_files1);

    let mut day_files2 = HashMap::new();
    day_files2.insert(789, Vec::new());
    day_files2.insert(1011, Vec::new());
    obs_data.insert(2024, day_files2);

    let day_files3 = HashMap::new();
    obs_data.insert(2025, day_files3);

    let day_files4 = HashMap::new();
    obs_data.insert(2026, day_files4);

    let day_files5 = HashMap::new();
    obs_data.insert(2027, day_files5);

    let obs_files_tree = ObsFilesTree::from_data(obs_data);

    let mut iter = obs_files_tree.get_obs_files();
    assert_eq!(iter.next(), None);
}
#[test]
fn test_obs_files_tree_from_data_empty_items_empty_days_empty_days_empty_days() {
    let mut obs_data = HashMap::new();
    let day_files1 = HashMap::new();
    obs_data.insert(2023, day_files1);

    let day_files2 = HashMap::new();
    obs_data.insert(2024, day_files2);

    let day_files3 = HashMap::new();
    obs_data.insert(2025, day_files3);

    let day_files4 = HashMap::new();
    obs_data.insert(2026, day_files4);

    let day_files5 = HashMap::new();
    obs_data.insert(2027, day_files5);

    let obs_files_tree = ObsFilesTree::from_data(obs_data);

    let mut iter = obs_files_tree.get_obs_files();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_1year_obs_files_tree_split_by_percent() {
    let mut obs_data = HashMap::new();
    let mut day_files1 = HashMap::new();
    day_files1.insert(123, vec!["file1.obs", "file2.obs"]);
    day_files1.insert(200, vec!["file3.obs", "file4.obs"]);
    day_files1.insert(5, vec!["file5.obs", "file6.obs"]);
    day_files1.insert(10, vec!["file7.obs", "file8.obs"]);
    day_files1.insert(111, vec!["file9.obs", "file10.obs"]);
    obs_data.insert(2023, day_files1);

    let obs_files_tree = ObsFilesTree::from_data(obs_data);

    let (left, right) = obs_files_tree.split_by_percent(50);
    assert_eq!(left.get_day_numbers(), 3);
    assert_eq!(right.get_day_numbers(), 2);

    let (left, right) = obs_files_tree.split_by_percent(30);
    assert_eq!(left.get_day_numbers(), 2);
    assert_eq!(right.get_day_numbers(), 3);

    let (left, right) = obs_files_tree.split_by_percent(80);
    assert_eq!(left.get_day_numbers(), 4);
    assert_eq!(right.get_day_numbers(), 1);
}

#[test]
fn test_2year_obs_files_tree_split_by_percent() {
    let mut obs_data = HashMap::new();
    let mut day_files1 = HashMap::new();
    day_files1.insert(123, vec!["file1.obs", "file2.obs"]);
    day_files1.insert(200, vec!["file3.obs", "file4.obs"]);
    obs_data.insert(2023, day_files1);

    let mut day_files2 = HashMap::new();
    day_files2.insert(5, vec!["file5.obs", "file6.obs"]);
    day_files2.insert(10, vec!["file7.obs", "file8.obs"]);
    obs_data.insert(2024, day_files2);

    let obs_files_tree = ObsFilesTree::from_data(obs_data);

    let (left, right) = obs_files_tree.split_by_percent(50);
    assert_eq!(left.get_day_numbers(), 2);
    assert_eq!(right.get_day_numbers(), 2);

    let (left, right) = obs_files_tree.split_by_percent(30);
    assert_eq!(left.get_day_numbers(), 1);
    assert_eq!(right.get_day_numbers(), 3);

    let (left, right) = obs_files_tree.split_by_percent(80);
    assert_eq!(left.get_day_numbers(), 3);
    assert_eq!(right.get_day_numbers(), 1);
}

#[test]
fn test_3year_obs_files_tree_split_by_percent() {
    let mut obs_data = HashMap::new();
    let mut day_files1 = HashMap::new();
    day_files1.insert(123, vec!["file1.obs", "file2.obs"]);
    day_files1.insert(200, vec!["file3.obs", "file4.obs"]);
    obs_data.insert(2023, day_files1);

    let mut day_files2 = HashMap::new();
    day_files2.insert(5, vec!["file5.obs", "file6.obs"]);
    day_files2.insert(10, vec!["file7.obs", "file8.obs"]);
    obs_data.insert(2024, day_files2);

    let mut day_files3 = HashMap::new();
    day_files3.insert(50, vec!["file10.obs", "file11.obs"]);
    day_files3.insert(100, vec!["file12.obs", "file13.obs"]);
    day_files3.insert(110, vec!["file15.obs", "file16.obs"]);
    obs_data.insert(2022, day_files3);

    let obs_files_tree = ObsFilesTree::from_data(obs_data);

    let (left, right) = obs_files_tree.split_by_percent(50);
    assert_eq!(left.get_day_numbers(), 4);
    assert_eq!(right.get_day_numbers(), 3);
    assert_eq!(
        left.get_files().next(),
        Some((2022, 50, PathBuf::from("2022/050/daily/file10.obs")))
    );
    assert_eq!(
        left.get_files().last(),
        Some((2023, 123, PathBuf::from("2023/123/daily/file2.obs")))
    );
    assert_eq!(
        right.get_files().nth(0),
        Some((2023, 200, PathBuf::from("2023/200/daily/file3.obs")))
    );

    let (left, right) = obs_files_tree.split_by_percent(30);
    assert_eq!(left.get_day_numbers(), 2);
    assert_eq!(right.get_day_numbers(), 5);
    assert_eq!(
        left.get_files().last(),
        Some((2022, 100, PathBuf::from("2022/100/daily/file13.obs")))
    );
    assert_eq!(
        right.get_files().next(),
        Some((2022, 110, PathBuf::from("2022/110/daily/file15.obs")))
    );

    let (left, right) = obs_files_tree.split_by_percent(80);
    assert_eq!(left.get_day_numbers(), 6);
    assert_eq!(right.get_day_numbers(), 1);
    assert_eq!(right.get_obs_files().count(), 2);
    assert_eq!(
        left.get_files().next(),
        Some((2022, 50, PathBuf::from("2022/050/daily/file10.obs")))
    );
    assert_eq!(
        left.get_files().last(),
        Some((2024, 5, PathBuf::from("2024/005/daily/file6.obs")))
    );
    assert_eq!(
        right.get_files().next(),
        Some((2024, 10, PathBuf::from("2024/010/daily/file7.obs")))
    );
}

#[test]
fn test_3year_obs_files_tree_split_by_100_percent() {
    let mut obs_data = HashMap::new();
    let mut day_files1 = HashMap::new();
    day_files1.insert(123, vec!["file1.obs", "file2.obs"]);
    day_files1.insert(200, vec!["file3.obs", "file4.obs"]);
    obs_data.insert(2023, day_files1);

    let mut day_files2 = HashMap::new();
    day_files2.insert(5, vec!["file5.obs", "file6.obs"]);
    day_files2.insert(10, vec!["file7.obs", "file8.obs"]);
    obs_data.insert(2024, day_files2);

    let mut day_files3 = HashMap::new();
    day_files3.insert(50, vec!["file10.obs", "file11.obs"]);
    day_files3.insert(100, vec!["file12.obs", "file13.obs"]);
    day_files3.insert(110, vec!["file15.obs", "file16.obs"]);
    obs_data.insert(2022, day_files3);

    let obs_files_tree = ObsFilesTree::from_data(obs_data);

    let (left, right) = obs_files_tree.split_by_percent(100);
    assert_eq!(left.get_day_numbers(), 7);
    assert_eq!(right.get_day_numbers(), 0);
}

#[test]
fn test_get_file() {
    let mut obs_data = HashMap::new();
    let mut day_files1 = HashMap::new();
    day_files1.insert(123, vec!["file1.obs", "file2.obs"]);
    day_files1.insert(200, vec!["file3.obs", "file4.obs"]);
    obs_data.insert(2023, day_files1);

    let mut day_files2 = HashMap::new();
    day_files2.insert(5, vec!["file5.obs", "file6.obs"]);
    day_files2.insert(10, vec!["file7.obs", "file8.obs"]);
    obs_data.insert(2024, day_files2);

    let mut day_files3 = HashMap::new();
    day_files3.insert(50, vec!["file10.obs", "file11.obs"]);
    day_files3.insert(100, vec!["file12.obs", "file13.obs"]);
    day_files3.insert(110, vec!["file15.obs", "file16.obs"]);
    obs_data.insert(2022, day_files3);

    let obs_files_tree = ObsFilesTree::from_data(obs_data);
    let file1 = obs_files_tree.get_files().next();
    assert!(file1.is_some());
    assert_eq!(
        file1.unwrap(),
        (2022, 50, PathBuf::from("2022/050/daily/file10.obs"))
    );
    let file2 = obs_files_tree.get_files().nth(1);
    assert!(file2.is_some());
    assert_eq!(
        file2.unwrap(),
        (2022, 50, PathBuf::from("2022/050/daily/file11.obs"))
    );
}

#[test]
fn test_obs_files_tree_find_next_file() {
    let mut obs_files_tree = ObsFilesTree::new();
    let year = 2023;
    let obs_files = vec!["file1.obs".to_string(), "file2.obs".to_string()];
    let obs_file_item1 = ObsFilesInDay::new(123, obs_files.clone());
    let obs_file_item2 = ObsFilesInDay::new(124, obs_files);
    let obs_files_tree_item = ObsFilesInYear::new(year, vec![obs_file_item1, obs_file_item2]);
    obs_files_tree.add_item(obs_files_tree_item);
    let next_file = obs_files_tree.find_next_file("file1", 2023, 123);
    assert_eq!(next_file, Some(PathBuf::from("2023/124/daily/file1.obs")));
}
