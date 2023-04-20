fn main() {
    let points = vec![(1,1), (2,2), (3,3), (4,3), (5,3), (6,3), (2,5), (2,1)];
    use std::collections::HashMap;

    let mut slope_intercept_map = HashMap::new();
    for i in 0..points.len() {
        for j in i+1..points.len() {

            if &points[j] == &points[i] {

                continue;

            } else if &points[j].0 == &points[i].0 {

                slope_intercept_map
                .entry(("inf".to_string(), *&points[j].1))
                .and_modify(|v| *v += 1)
                .or_insert(2);

            } else {

                let mut slope = (&points[j].1 - &points[i].1) as f64 / (&points[j].0 - &points[i].0) as f64;
                if slope < 0f64 {slope *= -1f64;}
                let intercept = *&points[i].1;

                // if slope == 0.0 && intercept == 1 { dbg!(i, j);}
                
                slope_intercept_map
                .entry((slope.to_string(), intercept))
                .and_modify(|v|  *v += 1)
                .or_insert(2);
            }
        }
    }

    dbg!(slope_intercept_map.values().sum::<i32>());
}
