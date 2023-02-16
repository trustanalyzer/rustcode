use std::env;

fn binary_search(nums: &[i32], x: i32) -> bool {

    let mut l: usize = 0;
    let mut r: usize = nums.len() - 1;

    while l <= r {
        let m: usize = (l + r) / 2;

        if &nums[m] == &x {
            println!{"index m == {:?}", m}
            return true;
        }

        if &nums[m] < &x {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    false
}


fn main() {
    let mut arr = [3, -1, 5, 0, 99, 97];
    arr.sort();
    //let res = binary_search(&arr, 98);
    //println!("{:?}", res);

    assert!(binary_search(&arr, 0));
}
