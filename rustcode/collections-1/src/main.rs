use std::collections::HashMap;


fn main() {
   let mut v: Vec<i32> = (-10000..9000).rev()
   .filter(|x| x%333==0)
   .collect();

   v.sort();
   println!("median is {:?}", &v[&v.len()/2]);
   
   let mut counter = HashMap::new();
   for key in &v {
       let val = counter.entry(key).or_insert(0);
        *val += 1
   }

   if let Some(max) = counter.keys().max() {
   println!("max val is for key = {:?}", max);
   }
}
