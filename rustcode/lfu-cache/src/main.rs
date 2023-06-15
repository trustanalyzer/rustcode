use std::collections::{LinkedList, HashMap};
use std::cmp::max;

#[derive(Default, Debug, Clone)]
struct LFUCache {
    capacity: usize,
    lfu_count: usize,
    val_map: HashMap<u32, u32>,
    count_map: HashMap<u32, usize>,
    list_map: HashMap<usize, LinkedList<u32>>,
}

impl LFUCache {
    fn new(capacity: usize) -> Self {
        Self {capacity: capacity,..Default::default()}
    }

    fn get(&mut self, key: u32) -> Option<u32> {
        
        match self.val_map.get(&key) {
            None => {return None;},
            Some(val) => {
                let key_ct = *self.count_map.get(&key).unwrap_or(&0);
                self.count_map.entry(key).and_modify(|ct| *ct = key_ct+1).or_default();
                self.list_map.entry(key_ct+1)
                .and_modify(|lst| lst.push_front(key))
                .or_insert(LinkedList::new());
                self.lfu_count = max(self.lfu_count, key_ct + 1);
                return Some(*val)
            }
        }
    }

    fn put(&mut self, key: u32, val: u32) -> () {

        if self.val_map.len() == self.capacity {
            
            if let Some(lst) = self.list_map.get_mut(&self.lfu_count) {
                lst.pop_back();
            }
            
        }; 

        self.val_map.insert(key, val);
        self.get(key);

    }
}

fn main() {
    let mut lfu_cache = LFUCache::new(2);
    println!("{lfu_cache:?}");

    lfu_cache.put(1,1);
    lfu_cache.put(2,2);
    lfu_cache.get(1);
    dbg!(&lfu_cache);

    lfu_cache.put(3,3);
    lfu_cache.get(2);
    dbg!(&lfu_cache);
    
    lfu_cache.put(4,4);
    dbg!(&lfu_cache);
}
