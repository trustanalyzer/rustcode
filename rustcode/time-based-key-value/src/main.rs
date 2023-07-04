
use std::collections::HashMap;

#[derive(Debug, Default)]
struct TimeMap<'a> {
    store: HashMap<String, Vec<(&'a str, i32)>>
}

impl<'a> TimeMap<'a> {
    fn new() -> Self {
        Self {
            store: HashMap::new()
        }
    }

    fn set(&mut self, key: String, value: &'a str, timestamp: i32) -> () {
        
        self.store
        .entry(key)
        .and_modify(|vtr | vtr.push((value, timestamp)))
        .or_insert(vec![(value, timestamp)]);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        
        let mut res = String::from("");

        if let Some(values) = self.store.get(&key) { 

            //dbg!(&values);
            let (mut l, mut r) = (0usize, values.len());
            while l < r {
                
                //dbg!(l,r);
                let m = (l + r) / 2;
                if let Some((val, ts)) = values.iter().nth(m) {
                    if *ts <= timestamp {
                        res = val.to_string();
                        l = m + 1;
                    } else {
                        r = m - 1;
                    }
                }
            }

            return res;

        }

        res
    }
}

fn main() {
    let mut time_map = TimeMap::new();
    time_map.set("foo".to_string(), "bar", 1);
    time_map.set("foo".to_string(), "bar2", 4);
    
    let res = time_map.get("foo".to_owned(), 4);
    println!("{res:?}");
}
