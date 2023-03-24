fn main() {
    let nums = vec![5,3,2,1,4];
    let merged = merge_sort(&nums);

    println!("{merged:?}");
}

fn merge_sort(arr: &Vec<i32>) -> Vec<i32> {    
    if arr.len() < 2 {
        return arr.to_vec();
    }

    let size = arr.len() / 2;
    let left = merge_sort(&arr[..size].to_vec());
    let right = merge_sort(&arr[size..].to_vec());
    let merged = merge(&left, &right);

    merged
}

fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    
    let mut merged = Vec::new();
    let mut i = 0usize;
    let mut j = 0usize;

    while i < left.len() && j < right.len() {

        if left[i] <= right[j] {

            merged.push(left[i]);
            i+=1;

        }  else {

            merged.push(right[j]);
            j+=1;

        };
    };

    while i < left.len() {
        merged.push(left[i]);
        i+=1;
    }

    while j < right.len() {
        merged.push(right[j]);
        j+=1;
    }

    merged

}