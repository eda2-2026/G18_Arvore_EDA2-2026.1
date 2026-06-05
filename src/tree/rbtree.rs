#![allow(dead_code)]

use std::marker::PhantomData;

/// Red-Black Tree backed key-value store.
/// `K` must implement `Ord`; the tree uses that ordering for all operations.
pub struct RBTree<K: Ord, V> {
    // Internal representation defined by Zanetti in issues #3/#7/#8.
    _phantom: PhantomData<(K, V)>,
}

impl<K: Ord, V> RBTree<K, V> {
    /// Creates an empty tree.
    pub fn new() -> Self {
        todo!()
    }

    /// Inserts `value` under `key`, replacing any previous value (upsert).
    pub fn insert(&mut self, _key: K, _value: V) {
        todo!()
    }

    /// Returns a reference to the value for `key`, or `None` if absent.
    pub fn get(&self, _key: &K) -> Option<&V> {
        todo!()
    }

    /// Removes `key` and returns its value, or `None` if the key did not exist.
    pub fn delete(&mut self, _key: &K) -> Option<V> {
        todo!()
    }

    /// Returns all `(key, value)` pairs with key in `[low, high]`, ascending.
    /// Complexity: O(log n + k), where k is the number of results.
    pub fn range(&self, _low: &K, _high: &K) -> Vec<(&K, &V)> {
        todo!()
    }

    /// Returns all `(key, value)` pairs in ascending key order.
    pub fn iter(&self) -> Vec<(&K, &V)> {
        todo!()
    }

    /// Returns the pair with the smallest key, or `None` if the tree is empty.
    pub fn min(&self) -> Option<(&K, &V)> {
        todo!()
    }

    /// Returns the pair with the largest key, or `None` if the tree is empty.
    pub fn max(&self) -> Option<(&K, &V)> {
        todo!()
    }

    /// Returns the number of key-value pairs stored in the tree.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Returns `true` if the tree contains no elements.
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::RBTree;

    // Contract tests: compilam agora, passam após a implementação do Zanetti.
    // #[should_panic] mantém `cargo test` verde enquanto os métodos são todo!().

    #[test]
    #[should_panic]
    fn insert_and_get_roundtrip() {
        let mut tree: RBTree<String, String> = RBTree::new();
        tree.insert("nome".to_string(), "Gabriel".to_string());
        assert_eq!(tree.get(&"nome".to_string()), Some(&"Gabriel".to_string()));
    }

    #[test]
    #[should_panic]
    fn delete_returns_removed_value() {
        let mut tree: RBTree<String, String> = RBTree::new();
        tree.insert("k".to_string(), "v".to_string());
        assert_eq!(tree.delete(&"k".to_string()), Some("v".to_string()));
        assert!(tree.get(&"k".to_string()).is_none());
    }

    #[test]
    #[should_panic]
    fn range_returns_pairs_in_order() {
        let mut tree: RBTree<String, String> = RBTree::new();
        for (k, v) in [("c", "3"), ("a", "1"), ("b", "2"), ("d", "4")] {
            tree.insert(k.to_string(), v.to_string());
        }
        let results = tree.range(&"a".to_string(), &"c".to_string());
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].0, "a");
        assert_eq!(results[2].0, "c");
    }

    #[test]
    #[should_panic]
    fn iter_preserves_sorted_order() {
        let mut tree: RBTree<i32, &str> = RBTree::new();
        for (k, v) in [(3, "c"), (1, "a"), (2, "b")] {
            tree.insert(k, v);
        }
        let keys: Vec<i32> = tree.iter().iter().map(|(k, _)| **k).collect();
        assert_eq!(keys, vec![1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn min_and_max_on_non_empty_tree() {
        let mut tree: RBTree<i32, &str> = RBTree::new();
        tree.insert(5, "e");
        tree.insert(1, "a");
        tree.insert(9, "i");
        assert_eq!(tree.min(), Some((&1, &"a")));
        assert_eq!(tree.max(), Some((&9, &"i")));
    }

    #[test]
    #[should_panic]
    fn empty_tree_invariants() {
        let tree: RBTree<i32, &str> = RBTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert!(tree.min().is_none());
        assert!(tree.max().is_none());
    }

    #[test]
    #[should_panic]
    fn len_tracks_insertions_and_deletions() {
        let mut tree: RBTree<i32, i32> = RBTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(1, 10);
        tree.insert(2, 20);
        assert_eq!(tree.len(), 2);
        tree.delete(&1);
        assert_eq!(tree.len(), 1);
    }
}
