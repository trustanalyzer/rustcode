
use std::collections::HashMap;

fn main() {

    let res = good_strings_count(3, 3, 1, 1);
    println!("{res:?}");
    
}

fn good_strings_count(low: usize, high: usize, zero: usize, one: usize) -> usize {

    let mut counter = HashMap::new();
    counter.insert(0,1);

    for size in 1..=high {
        
        let mut ways = 0usize;
        if size >= zero {
            ways +=  counter.get(&(&size - &zero)).unwrap_or(&0);
        }

        if size >= one {
            ways +=  counter.get(&(&size - &one)).unwrap_or(&0);
        }

        counter.insert(size, ways);

    }

    let mut num_ways = 0usize;
    for size in low..=high {
        num_ways += counter.get(&size).unwrap_or(&0);
    }

    num_ways

}
