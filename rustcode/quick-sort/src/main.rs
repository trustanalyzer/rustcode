
fn main() {
    let mut arr = vec![8,22,2,5,4,20,10,12];

    let size = arr.len();
    quick_sort(&mut arr, 0, size-1);

    println!("{:?}", arr);
}

fn partition(arr: &mut Vec<i32>, l: usize, r: usize) -> usize {
    
    let mut p_index = l;
    for i in l..r {
        if arr[i] <= arr[r] {
            arr.swap(i, p_index);
            p_index += 1;
        }
    }
    arr.swap(r, p_index);
    //println!("{:?}", arr);
    p_index
}

fn quick_sort(arr: &mut Vec<i32>, l: usize, r: usize) {
    if l < r {
        let pi = partition(arr, l, r);
        //println!("{:?}", pi);
        quick_sort(arr, l, pi-1);
        quick_sort(arr, pi+1, r);
    };
}
