use std::{char, collections::HashMap};

/// ======================================================
/// Trie (Prefix Tree)
///
/// Trie là cây lưu string.
/// Mỗi node = 1 ký tự.
/// Đường từ root -> node = prefix.
///
/// Example words:
///
/// cat
/// car
/// cap
///
/// Tree:
///
/// root
///  └─ c
///      └─ a
///          ├─ t
///          ├─ r
///          └─ p
///
/// ======================================================

#[derive(Debug)]
pub struct TrieNode {
    /// children của node
    /// key = ký tự tiếp theo
    /// value = node tiếp theo
    children: HashMap<char, TrieNode>,

    /// đánh dấu node này là kết thúc của word
    is_word: bool,
}

impl TrieNode {
    /// create node mới
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    /// root node
    root: TrieNode,
}

impl Trie {
    /// ======================================
    /// Create empty trie
    /// ======================================
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    /// ======================================
    /// Insert word
    ///
    /// Example:
    /// insert("cat")
    ///
    /// root -> c -> a -> t
    ///
    /// Steps:
    ///
    /// 1 start từ root
    ///
    /// 2 iterate từng character trong word
    ///
    /// 3 nếu character chưa tồn tại trong children
    ///     create node mới
    ///
    /// 4 move xuống node đó
    ///
    /// 5 sau khi hết word
    ///     mark node.is_word = true
    ///
    /// ======================================
    pub fn insert(&mut self, word: &str) {
        let mut curent_node = &mut self.root;

        for char in word.chars() {
            curent_node = curent_node.children.entry(char).or_insert(TrieNode::new());
        }

        curent_node.is_word = true;
    }

    /// ======================================
    /// Search full word
    ///
    /// Example:
    ///
    /// search("cat")
    ///
    /// Steps:
    ///
    /// 1 start từ root
    ///
    /// 2 iterate từng character
    ///
    /// 3 nếu character không tồn tại
    ///     return false
    ///
    /// 4 move xuống node tiếp theo
    ///
    /// 5 sau khi hết word
    ///     check is_word
    ///
    /// ======================================
    pub fn search(&self, word: &str) -> bool {
        // STEP 1
        // current node = root
        let mut current_node = &self.root;

        // STEP 2
        // iterate characters
        for char in word.chars() {
            // STEP 3
            // nếu character không tồn tại -> return false

            // STEP 4
            // move node

            current_node = match current_node.children.get(&char) {
                Some(node) => node,
                None => return false,
            };
        }

        current_node.is_word

        // STEP 5
        // check node.is_word
    }

    /// ======================================
    /// Check prefix
    ///
    /// Example:
    ///
    /// starts_with("ca")
    ///
    /// cat
    /// car
    /// cap
    ///
    /// Steps:
    ///
    /// 1 start từ root
    ///
    /// 2 iterate prefix
    ///
    /// 3 nếu character không tồn tại
    ///     return false
    ///
    /// 4 move xuống node
    ///
    /// 5 nếu đi hết prefix
    ///     return true
    ///
    /// ======================================
    pub fn starts_with(&self, prefix: &str) -> bool {
        // STEP 1
        // current node = root
        let mut current_node = &self.root;

        // STEP 2
        // iterate characters
        for char in prefix.chars() {
            // STEP 3
            // nếu character không tồn tại -> return false

            // STEP 4
            // move node

            current_node = match current_node.children.get(&char) {
                Some(node) => node,
                None => return false,
            };
        }

        // STEP 3
        // nếu char không tồn tại -> false

        // STEP 4
        // move node

        // STEP 5
        // return true
        true
    }

    pub fn print_tree(&self) {
        println!("Trie:");
        Self::print_node(&self.root, 0);
    }

    fn print_node(node: &TrieNode, depth: usize) {
        for (ch, child) in &node.children {
            let indent = "  ".repeat(depth);

            if child.is_word {
                println!("{}{}*", indent, ch);
            } else {
                println!("{}{}", indent, ch);
            }

            Self::print_node(child, depth + 1);
        }
    }
}
