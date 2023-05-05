use std::collections::{HashMap, LinkedList};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    key: i32,
    val: i32,
}

#[derive(Debug)]
struct LRUCache<'a> {
    capacity: usize,
    hash_map: HashMap<&'a Node, &'a Node>,
    linked_list: LinkedList<&'a Node>,
}

impl<'a> LRUCache<'a> {
    fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity,
            hash_map: HashMap::new(),
            linked_list: LinkedList::new(),
        }
    }

    fn get(&self, node: &Node) -> Option<&Node> {
        self.hash_map.get(node).copied()
    }

    fn put(&mut self, node: &'a Node) -> () {
        if self.linked_list.len() == self.capacity {
            if let Some(deleted_node) = self.linked_list.pop_back() {
                self.hash_map.remove(deleted_node);
            }
        }
        self.hash_map.insert(node, node);
        self.linked_list.push_front(node);
    }
}



fn main() {
    let mut lru_cache = LRUCache::new(2);
    let node_1 = Node{key: 0,val: 0};
    lru_cache.put(&node_1);

    dbg!(&lru_cache);

    let node_2 = Node{key:1, val:-1};
    let res = lru_cache.get(&node_2);

    dbg!(res);

}
