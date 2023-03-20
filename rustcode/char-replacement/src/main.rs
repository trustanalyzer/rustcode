use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let word = "ABABBA".to_owned();
    let ans = character_replacement(&word, 2);
    println!("{:?}", ans);

    let eff = efficient_replacement(&word, 2);
    println!("{:?}", eff);

}

fn character_replacement(word: &String, k: usize) -> usize {
    let mut counter: HashMap<char, usize> = HashMap::new();
    let mut res = 0;

    let mut l = 0;

    for (r,char) in word.chars().enumerate() {
        counter
        .entry(char)
        .and_modify(|e| {*e +=1})
        .or_insert(1);

        while (r - l + 1) - counter.get(&word.chars().nth(l).unwrap()).unwrap() > k {
            counter
            .entry(word.chars().nth(l).unwrap())
            .and_modify(|e| {*e -= 1});

            res = max(res, r - l +1);
            l += 1;
        }
    }

    res


}

fn efficient_replacement(word: &String, k: usize) -> usize {
    let mut counter: HashMap<char, usize> = HashMap::new();
    let mut res = 0;

    let mut l = 0;
    let mut maxf = 0;

    for (r,char) in word.chars().enumerate() {
        counter
        .entry(char)
        .and_modify(|e| {*e +=1})
        .or_insert(1);

        //println!("{:?}", &char);
        maxf = max(maxf, *counter.get(&char).unwrap());

        while (r - l + 1) - maxf > k {
            counter
            .entry(word.chars().nth(l).unwrap())
            .and_modify(|e| {*e -= 1});

            res = max(res, r - l +1);
            l += 1;
        }
    }

    res


}