

use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct Node {
    val: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Self {val: val, prev: None, next: None}
    }
}

#[derive(Debug, Clone)]
struct LRUCache {
    capacity: usize,
    key_node_map: HashMap<u32, Rc<RefCell<Node>>>,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity, 
            key_node_map: HashMap::new(),
            head: None, 
            tail: None
        }
    }

    fn get(&mut self, key: u32) -> Option<i32> {
        
        if !self.key_node_map.contains_key(&key) {
            return None;
        }
        
        let new_head = self.key_node_map.get(&key).unwrap();
        self.tail = new_head.borrow_mut().prev;
        new_head.borrow_mut().prev = None;
        self.tail.as_mut().unwrap().borrow_mut().next = None;
        
        
        new_head.borrow_mut().next = self.head;
        self.head.unwrap().borrow_mut().prev = Some(*new_head);
        self.head = Some(*new_head);

        Some(new_head.borrow().val)
    }

    fn put(&mut self, key: u32, val: i32) -> () {
        
        if self.capacity == self.key_node_map.len() {
            let tail = self.tail.unwrap();
            self.tail = tail.borrow().prev;
            self.tail.unwrap().borrow_mut().next = None;
        }

        let new_node = RefCell::new(Node::new(val));
        self.key_node_map.insert(key, Rc::new(new_node));
        self.head.unwrap().borrow_mut().prev = Some(Rc::clone(&new_node));
        self.head = Some(Rc::clone(new_node));

    }


}

fn main() {
    
    let mut lru_cache = LRUCache::new(4);
    
    let res = lru_cache.get(0);
    
    println!("{res:?}");
}
