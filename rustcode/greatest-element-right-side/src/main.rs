use std::cmp::max;

fn main() {
    let mut arr = vec![17,18,5,4,6,1];
    let mut right_max = -1;
    for i in (0..arr.len()).rev() {
        let new_max = max(right_max, arr[i]);
        arr[i] = right_max;
        right_max = new_max;
    }

    println!("{arr:?}");
}
