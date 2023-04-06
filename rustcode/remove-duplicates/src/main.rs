fn main() {
    let mut nums = vec![1,1,1,2,2,2,3,4,4,5,5,5];
    let ans = remove_duplicates(&mut nums);
    dbg!(ans);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
    let mut l = 0usize;
    let mut r = 0usize;
    let n = nums.len();

    while r < n {

        let mut count = 1;
        while r + 1 < n && nums[r] == nums[r+1] {
            r += 1;
            count += 1;
        }

        use std::cmp::min;
        for _ in 0..min(2, count) {
            nums[l] = nums[r];
            l += 1;
        }

        r += 1;
    }

    l
}