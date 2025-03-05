use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let mut set = HashSet::new();
    for item in input_str.split(',') {
        set.insert(item);
    }
    set.len()
}
