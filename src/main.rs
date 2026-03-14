pub mod heap;
pub mod lru;

use heap::*;
use lru::*;

// fn main() {
//     let mut cache = LRUCache::new(3);

//     cache.put(1, "A");
//     cache.put(2, "B");
//     cache.put(3, "C");

//     cache.print("Initial cache");

//     // cache:
//     // C B A

//     cache.get(&1);

//     // cache:
//     // A C B

//     cache.put(4, "D");
//     cache.print("Cache after adding D");

//     // cache:
//     // D A C

//     cache.get(&1);
//     cache.print("Cache after getting 1");

//     // cache:
//     // A D C

//     cache.put(5, "E");
//     cache.print("Cache after adding E");

//     // cache:
//     // E A D
// }

// fn main() {
//     let mut cache = LRUCache::new(3);

//     println!("--- PUT 1 A ---");
//     cache.put(1, "A");

//     println!("--- PUT 2 B ---");
//     cache.put(2, "B");

//     println!("--- PUT 3 C ---");
//     cache.put(3, "C");

//     cache.print_state();

//     println!("--- GET 1 ---");
//     println!("value = {:?}", cache.get(&1));

//     cache.print_state();

//     println!("--- PUT 4 D (should evict key 2) ---");
//     cache.put(4, "D");

//     cache.print_state();

//     println!("--- GET 2 (should be None) ---");
//     println!("value = {:?}", cache.get(&2));

//     println!("--- GET 3 ---");
//     println!("value = {:?}", cache.get(&3));

//     cache.print_state();
// }

fn main() {
    let mut heap: BinaryHeap<u8> = BinaryHeap::new();

    heap.push(5);
    heap.push(8);
    heap.push(3);
    heap.push(10);

    println!("Heap: {:?}", heap);

    heap.push(15);
    println!("Heap: {:?}", heap);

    heap.push(6);
    println!("Heap: {:?}", heap);

    heap.push(7);
    println!("Heap: {:?}", heap);

    heap.pop();
    println!("Heap {:?}", heap);
}
