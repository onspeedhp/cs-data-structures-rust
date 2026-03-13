use std::collections::HashMap;

/// =============================
/// NODE
/// =============================
///
/// node trong doubly linked list
///
#[derive(Debug)]
struct Node<K, V> {
    // key cần lưu để khi evict tail
    // ta biết phải remove key nào khỏi hashmap
    key: K,

    value: V,

    prev: Option<usize>,
    next: Option<usize>,
}

/// =============================
/// DOUBLY LINKED LIST
/// =============================

#[derive(Debug)]
struct DoublyLinkedList<K, V> {
    // storage node
    nodes: Vec<Node<K, V>>,

    // most recently used
    head: Option<usize>,

    // least recently used
    tail: Option<usize>,
}

impl<K, V> DoublyLinkedList<K, V> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            head: None,
            tail: None,
        }
    }

    /// =============================
    /// PUSH FRONT
    /// =============================
    ///
    /// insert node vào head
    ///
    /// steps:
    /// 1 push node vào vec
    /// 2 lấy index
    /// 3 nếu list rỗng
    ///      head = index
    ///      tail = index
    ///
    /// 4 nếu list không rỗng
    ///      old_head = head
    ///      node.next = old_head
    ///      old_head.prev = index
    ///
    /// 5 update head
    ///
    pub fn push_front(&mut self, node: Node<K, V>) -> usize {
        self.nodes.push(node);
        let index = self.nodes.len() - 1;

        if let Some(old_head) = self.head {
            self.nodes[index].next = Some(old_head);
            self.nodes[old_head].prev = Some(index);
        } else {
            self.tail = Some(index);
        }

        self.head = Some(index);

        index
    }

    /// =============================
    /// REMOVE NODE
    /// =============================
    ///
    /// remove node khỏi list
    ///
    /// steps:
    ///
    /// prev <-> node <-> next
    ///
    /// becomes
    ///
    /// prev <-> next
    ///
    /// update the node's prev and next to None
    pub fn remove(&mut self, index: usize) {
        let prev = self.nodes[index].prev;
        let next = self.nodes[index].next;

        if let Some(prev) = prev {
            self.nodes[prev].next = next
        } else {
            self.head = next;
        }

        if let Some(next) = next {
            self.nodes[next].prev = prev;
        } else {
            self.tail = prev;
        };

        self.nodes[index].prev = None;
        self.nodes[index].next = None;
    }

    /// =============================
    /// MOVE NODE TO FRONT
    /// =============================
    ///
    /// steps:
    ///
    /// remove(node)
    /// push_front(node)
    ///
    pub fn move_to_front(&mut self, index: usize) {
        if Some(index) == self.head {
            return;
        }

        self.remove(index);

        if let Some(old_head) = self.head {
            self.nodes[index].next = Some(old_head);
            self.nodes[old_head].prev = Some(index);
        }

        self.head = Some(index);
        if self.tail.is_none() {
            self.tail = Some(index);
        }
    }

    /// =============================
    /// POP TAIL
    /// =============================
    ///
    /// remove least recently used
    ///
    pub fn pop_tail(&mut self) -> Option<usize> {
        let tail = self.tail?;
        self.remove(tail);
        Some(tail)
    }
}

/// =============================
/// LRU CACHE
/// =============================

pub struct LRUCache<K, V> {
    capacity: usize,

    // key -> node index
    map: HashMap<K, usize>,

    list: DoublyLinkedList<K, V>,
}

impl<K: Eq + std::hash::Hash + Clone, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            list: DoublyLinkedList::new(),
        }
    }

    /// =============================
    /// GET
    /// =============================
    ///
    /// steps
    ///
    /// 1 lookup hashmap
    ///
    /// 2 nếu không có -> return None
    ///
    /// 3 lấy node index
    ///
    /// 4 move node lên head
    ///
    /// 5 return value
    ///
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let index = *self.map.get(key)?;
        self.list.move_to_front(index);
        Some(&self.list.nodes[index].value)
    }

    /// =============================
    /// PUT
    /// =============================
    ///
    /// steps
    ///
    /// CASE 1 key exists
    ///
    /// update value
    /// move_to_front
    ///
    ///
    /// CASE 2 key new
    ///
    /// create node
    /// push_front
    /// insert hashmap
    ///
    ///
    /// CASE 3 capacity exceeded
    ///
    /// pop_tail
    /// remove key khỏi hashmap
    ///
    pub fn put(&mut self, key: K, value: V) {
        if let Some(&index) = self.map.get(&key) {
            self.list.nodes[index].value = value;
            self.list.move_to_front(index);
            return;
        }

        let node = Node {
            key: key.clone(),
            value,
            prev: None,
            next: None,
        };

        let index = self.list.push_front(node);
        self.map.insert(key, index);

        if self.map.len() > self.capacity {
            if let Some(tail_index) = self.list.pop_tail() {
                let old_key = self.list.nodes[tail_index].key.clone();
                self.map.remove(&old_key);
            }
        }
    }
}

impl<K: std::fmt::Debug + Eq + std::hash::Hash + Clone, V: std::fmt::Debug> LRUCache<K, V> {
    pub fn print_state(&self) {
        println!("---- CACHE STATE ----");

        let mut current = self.list.head;

        while let Some(i) = current {
            let node = &self.list.nodes[i];

            println!(
                "key={:?} value={:?} prev={:?} next={:?}",
                node.key, node.value, node.prev, node.next
            );

            current = node.next;
        }

        println!("head = {:?}", self.list.head);
        println!("tail = {:?}", self.list.tail);

        println!("---------------------");
    }
}
