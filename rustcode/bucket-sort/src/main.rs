use std::collections::HashMap;

fn main() {
    let nums = vec![3,5,2,9,15,9,19,2,9];
    
    let mut counter: HashMap<i32, u32> = HashMap::new();
    for num in nums {
        if let Some(count) = counter.get_mut(&num) {
            //println!("{:?}", &count);
            *count += 1;
        } else {
            counter.insert(num, 1);
        }
    }

    println!("{:?}", counter);
}
