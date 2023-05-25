use std::collections::{HashMap, HashSet};

fn main() {
    let ideas = vec!["coffee".to_owned(), 
    "donuts".to_owned(), 
    "time".to_owned(), 
    "toffee".to_owned()];

    let ct = count_valid_names(ideas);
    println!("{ct:?}");

}

fn count_valid_names(ideas: Vec<String>) -> usize {

    let mut suffix_map: HashMap<char, HashSet<String>> = HashMap::new();
    for idea in ideas {
        if let Some(c) = idea.chars().next() {
            if let None = suffix_map.get_mut(&c) {
                suffix_map.insert(c, HashSet::new());
            }
            
            suffix_map.get_mut(&c).unwrap().insert(idea[1..].to_owned());
        }
    }

    println!("{suffix_map:?}");
    let mut count_names = 0usize;
    for char_1 in suffix_map.keys() {
        for char_2 in suffix_map.keys() {
            if char_1 ==  char_2 {continue;}

            let hs_1 = suffix_map.get(&char_1).unwrap();
            let hs_2 = suffix_map.get(&char_2).unwrap();

            let union = hs_1.union(hs_2).count();
            let intersect = hs_1.intersection(&hs_2).count();
            println!("{}, {}", union, intersect);
            count_names += union - intersect;
        }
    }

    count_names
}
