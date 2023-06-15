use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let s = "abciiidef".to_owned();

    let res = max_vowel_substring(&s, 3);
    dbg!(res);

}

fn max_vowel_substring(s: &String, k: usize) -> u32 {

    let mut vowel_map = HashMap::new();
    for c in 'a'..='z' {
        match c {
            'a' | 'e' |'i' |'o' | 'u' => {vowel_map.insert(c,1);},
            _ => {vowel_map.insert(c, 0);},
        }
    }

    let (mut l, mut ct, mut res) = (0, 0, 0);
    for r in 0..s.len() {
    
        ct += vowel_map[&s.chars().nth(r).unwrap()];
        if r - l + 1 > k {
            ct -= vowel_map[&s.chars().nth(l).unwrap()];
            l += 1;
        }
        res = max(res, ct);
    }

    res
}
