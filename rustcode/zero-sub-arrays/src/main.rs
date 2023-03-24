fn main() {
    let nums = vec![1,2,0,0,4,5,0,0,0,6,0];

    let mut res = 0;
    let mut count = 0;
    for num in nums {
        if num == 0 {
            count += 1;
        } else {
            count = 0;
        }

        res += count;
    }
    
    println!("{res:?}");
}
