/// ======================================================
/// Binary Heap (Max Heap version)
///
/// Heap property:
/// parent >= children
///
/// Array representation:
///
/// index:   0   1   2   3   4   5
/// value:  [9,  7,  6,  3,  5,  4]
///
/// Tree:
///
///        9
///       / \
///      7   6
///     / \
///    3   5
///
/// Parent / child formula
///
/// parent = (i - 1) / 2
/// left   = 2*i + 1
/// right  = 2*i + 2
///
/// ======================================================

#[derive(Debug)]
pub struct BinaryHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> BinaryHeap<T> {
    /// Create empty heap
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// number of elements
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// ======================================
    /// Peek root (max element)
    ///
    /// root luôn ở index 0
    /// O(1)
    /// ======================================
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    /// ======================================
    /// Push element
    ///
    /// Steps:
    ///
    /// 1 push element vào cuối vec
    ///
    /// 2 heapify_up
    ///
    /// heapify_up sẽ:
    ///   compare node với parent
    ///   nếu node > parent
    ///   swap
    ///
    /// repeat cho đến khi heap property đúng
    ///
    /// ======================================
    pub fn push(&mut self, value: T) {
        // STEP 1
        // push vào cuối heap
        self.data.push(value);

        // STEP 2
        // heapify_up từ node cuối
        let index = self.len() - 1;

        self.heapify_up(index);
    }

    /// ======================================
    /// Pop root (max element)
    ///
    /// Steps:
    ///
    /// 1 swap root với element cuối
    ///
    /// 2 pop element cuối
    ///
    /// 3 heapify_down từ root
    ///
    /// ======================================
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        // STEP 1
        // swap root và element cuối
        let index = self.len() - 1;
        self.data.swap(0, index);

        // STEP 2
        // pop element cuối (max value)
        let max = self.data.pop();

        // STEP 3
        // heapify_down từ root

        if !self.data.is_empty() {
            self.heapify_down(0);
        }

        max
    }

    /// ======================================
    /// Heapify Up
    ///
    /// Used after insert
    ///
    /// Example
    ///
    /// [9,7,6,3,5,8]
    ///
    /// node index = 5
    ///
    /// compare với parent
    ///
    /// parent index = (i - 1) / 2
    ///
    /// swap nếu node > parent
    ///
    /// continue until root
    ///
    /// ======================================
    fn heapify_up(&mut self, mut index: usize) {
        // loop until root
        //
        // while index > 0
        //
        //     parent = (index - 1) / 2
        //
        //     if heap[index] <= heap[parent]
        //         break
        //
        //     swap(index, parent)
        //
        //     index = parent

        while index > 0 {
            let parent_index = (index - 1) / 2;

            if self.data[index] <= self.data[parent_index] {
                break;
            }

            self.data.swap(index, parent_index);

            index = parent_index;
        }
    }

    /// ======================================
    /// Heapify Down
    ///
    /// Used after pop
    ///
    /// Example
    ///
    /// root = 6
    ///
    /// compare với children
    ///
    /// swap với child lớn hơn
    ///
    /// repeat until leaf
    ///
    /// ======================================
    fn heapify_down(&mut self, mut index: usize) {
        // loop:
        //
        // left  = 2*i + 1
        // right = 2*i + 2
        //
        // find largest among
        //   index
        //   left
        //   right
        //
        // if largest != index
        //     swap
        //     index = largest
        //
        // else
        //     break

        loop {
            let left_index = 2 * index + 1;
            let right_index = 2 * index + 2;

            let mut largest = index;

            if left_index < self.len() && self.data[left_index] > self.data[largest] {
                largest = left_index;
            }

            if right_index < self.len() && self.data[right_index] > self.data[largest] {
                largest = right_index;
            }

            if largest != index {
                self.data.swap(largest, index);
                index = largest;
            } else {
                break;
            }
        }
    }
}
