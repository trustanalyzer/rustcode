use std::cmp::max;

fn main() {
    let elements = vec![2,1,4,1,2,1,3];
    let k = 2;

    let res = max_frequency_subsequence(elements, k);
    println!("{res:?}");
}

fn max_frequency_subsequence(mut scores: Vec<i32>, k: i32) -> usize {

    scores.sort();
    let mut total = 0;
    let mut l = 0;
    let mut max_freq = 0usize;

    for r in 0..scores.len() {

        total += scores[r];
        while scores[r]*(r as i32-l+1) > total + k {
            total -= scores[l as usize];
            l += 1;
        }

        max_freq = max(max_freq, r-l as usize +1);

    }

    max_freq

}
