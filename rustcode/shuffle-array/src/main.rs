fn main() {
    let mut arr = vec![2u32, 5, 1, 3, 4, 7];
    shuffle_array(&mut arr, 3);
    println!("{arr:?}");
}

fn shuffle_array(arr: &mut Vec<u32>, n: usize) -> () {

    for i in 0..n {
        arr[i] = arr[i] << 10;
        arr[i] |= arr[i+n];
    }

    for i in (0..n).rev() {
        let y = arr[i] & (2_u32.pow(10) - 1);
        let x = arr[i] >> 10;

        arr[2*i+1] = y;
        arr[2*i] = x;
        // dbg!(i , &arr);
    }
}