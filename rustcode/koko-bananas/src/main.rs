use std::cmp::min;

fn main() {
    let piles = vec![3,6,7,11];
    let h = 8;

    let res = min_hours(&piles, h);
    println!("{res:?}");

}

fn min_hours(piles: &Vec<i32>, h: i32) -> i32 {

    let (mut l, mut r, mut res) = (1, *piles.iter().max().unwrap(), *piles.iter().max().unwrap());
    while l <= r {
        
        let m = (l+r)/2;
        let hrs = piles.iter().map(|e| (*e as f64)/(m as f64)).sum::<f64>();
        //dbg!(m, hrs);
        
        if hrs <= h as f64 {
            res = min(res, hrs as i32);
            r = m - 1;
        } else {
            l = m + 1;
        }

    }

    res

}
