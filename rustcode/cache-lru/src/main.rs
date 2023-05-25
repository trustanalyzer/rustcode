use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct LRUCache {
    capacity: usize,
    hash_map: HashMap<i32, char>,
    linked_list: VecDeque<i32>,
}


impl LRUCache {

    fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity,
            hash_map: HashMap::new(),
            linked_list: VecDeque::new(),
        }
    }

    fn get(&mut self, key: i32) -> Option<char> {
        match self.hash_map.get(&key) {
            None => {return None;},
            Some(c) => {
                for (idx, chr_key) in self.linked_list.iter_mut().enumerate() {
                    if *chr_key == key {
                        self.linked_list.remove(idx);
                        break;
                    }
                }
                self.linked_list.push_front(key);
                return Some(*c);
            }
        }
    }

    fn put(&mut self, key: i32, val: char) -> () {
        
        if self.hash_map.len() == self.capacity {
            
            let lru_key = self.linked_list.pop_back().unwrap();
            self.hash_map.remove(&lru_key);
        }
        

        self.linked_list.push_front(key);
        self.hash_map.insert(key, val);


    }
}

fn main() {

    let mut lru_cache = LRUCache::new(3);
    dbg!(&lru_cache);

    let val = lru_cache.get(3);
    dbg!(val);

    for (i,c) in [(1,'a'), (2,'b'), (3,'c'), (5,'e')] {
        lru_cache.put(i,c);
    }

    let val = lru_cache.get(3);
    dbg!(val);

    dbg!(&lru_cache);
}
