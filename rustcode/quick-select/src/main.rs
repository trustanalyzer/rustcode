fn main() {
    let mut nums = vec![3,2,1,5,6,4];
    let mut l = 0;
    let mut r = nums.len() - 1;

    let k = nums.len() - 2;

    while quick_select(&mut nums, l, r) != k {

        let p = quick_select(&mut nums, l, r);

        if p < k { l = p+1;}  
        else if p > k { r = p-1;} 
        else { println!("{:?}",nums[p])}
    }

}

fn quick_select(nums: &mut Vec<i32>, l: usize, r: usize) -> usize {
    let pivot = nums[r];
    let mut p = l;
    for i in l..r {
        if nums[i] <= pivot {
            nums[p] = nums[i];
            p += 1;
        }
    }

    nums.swap(r,p);
    println!("{:?}", &nums);

    p
}
