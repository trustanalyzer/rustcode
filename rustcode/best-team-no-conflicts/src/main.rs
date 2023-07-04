use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let scores = vec![1,3,15,10,5];
    let ages = vec![1,2,3,4,5];
    let res = max_score(scores, ages);
    println!("{res:?}");
}

fn max_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {

    let mut zip_tuples = scores.into_iter()
    .zip(ages.into_iter())
    .into_iter()
    .collect::<Vec<_>>();
    
    zip_tuples.sort();
    //println!("{zip_tuples:?}");

    let mut table = HashMap::new();
    for i in 0..zip_tuples.len() {

        table.insert(i, zip_tuples[i].0); 
        for j in 0..i {
            if zip_tuples[j].1 <= zip_tuples[i].1 {
                let curr_score = *table.get(&i).unwrap();
                let other_score = zip_tuples[i].0 + *table.get(&j).unwrap();

                table.insert(i, max(curr_score, other_score));

            }
        }
    }

    *table.values().to_owned().max().unwrap()
}


