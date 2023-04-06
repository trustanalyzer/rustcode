fn main() {
    let s = String::from("abacaba");
    let res = min_partitions(&s);
    dbg!(res);
}

fn min_partitions(str: &str) -> i32 {
    use std::collections::HashSet;
    let mut curr_set = HashSet::new();
    let mut partitions = 0;

    for c in str.chars() {
        if curr_set.contains(&c) {
            let curr_set: HashSet<char> = HashSet::new();
            partitions += 1;
        }
        curr_set.insert(c);
    }

    partitions
}
