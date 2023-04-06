fn main() {
    let mut v = vec![3,4,2,3];
    let is_possible = check_possibility(&mut v);
    dbg!(is_possible);
    dbg!(v);
}

fn check_possibility(arr: &mut Vec<i32>) -> bool {
    let mut changed = false;
    for i in 0..arr.len()-1 {
        if arr[i] <= arr[i+1] {continue;}
        if changed {return false;}

        if i==0 || arr[i-1] <= arr[i+1] {
            arr[i] = arr[i+1];
        } else {
            arr[i+1] = arr[i];
            changed = true;
        }
    }
    true
}