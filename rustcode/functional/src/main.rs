fn main() {
    let vec: Vec<i32> = (0..=100).step_by(3).collect();
    let plus_one = |x| x+1;

    let vec_2: Vec<i32> = vec
    .into_iter()
    .map(plus_one)
    .filter(|e| *e%5 == 0)
    .collect();

    dbg!(vec_2);
}


