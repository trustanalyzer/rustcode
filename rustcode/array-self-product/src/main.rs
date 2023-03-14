fn main() {
    let mut arr = vec![2, 3, 4, 5 ,6, 1];
    let n = arr.len();

    let mut left = 1;
    for i in 0..n {
        let temp = arr[i];
        arr[i] = left;
        left *= temp;
    }

    let mut right = arr[n-1];
    for i in (0..n-2).rev() {
        arr[i] *= right;
        right = arr[i];
    }

    println!("ans = {:?}", arr)
}
