use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let s = "Helledoword".to_lowercase().to_string();
    let labels = partition_labels(&s);
    dbg!(labels);
}

fn partition_labels(string: &String) -> Vec<usize> {
    
    let mut char_index_map = HashMap::new();
    for (i, c) in string.chars().enumerate() {
        char_index_map.entry(c)
        .and_modify(|idx| *idx=i)
        .or_insert(i);
    }

    
    let (mut size, mut end) = (0usize, 0usize);
    let mut partition_labels = Vec::new();
    for (i, c) in string.chars().enumerate() {
        size += 1;
        end = max(end, char_index_map[&c]);

        if end == i {
            partition_labels.push(size);
            size = 0;
        }
    }

    partition_labels
}

