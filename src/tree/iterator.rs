use crate::tree::node::{Link, Node};

pub struct InOrderIterator<'a, K: Ord, V> {
    stack: Vec<&'a Node<K, V>>,
    low: Option<&'a K>,
    high: Option<&'a K>,
}

impl<'a, K: Ord, V> InOrderIterator<'a, K, V> {
    pub fn new(root: &'a Link<K, V>, low: Option<&'a K>, high: Option<&'a K>) -> Self {
        let mut iter = Self { stack: Vec::new(), low, high };
        iter.push_left_spine(root.as_deref());
        iter
    }

    // Descemos pela espinha esquerda empurrando nós ao stack, com poda:
    // - key < low  → pula este nó e subárvore esquerda, desce pela direita
    // - key > high → pula este nó e subárvore direita, desce pela esquerda
    fn push_left_spine(&mut self, mut node: Option<&'a Node<K, V>>) {
        while let Some(n) = node {
            if self.low.is_some_and(|l| &n.key < l) {
                node = n.right.as_deref();
            } else if self.high.is_some_and(|h| &n.key > h) {
                node = n.left.as_deref();
            } else {
                self.stack.push(n);
                node = n.left.as_deref();
            }
        }
    }
}

impl<'a, K: Ord, V> Iterator for InOrderIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|node| {
            self.push_left_spine(node.right.as_deref());
            (&node.key, &node.value)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::InOrderIterator;
    use crate::tree::rbtree::RBTree;

    fn build_tree(keys: &[i32]) -> RBTree<i32, i32> {
        let mut t = RBTree::new();
        for &k in keys {
            t.insert(k, k * 10);
        }
        t
    }

    #[test]
    fn empty_tree_yields_nothing() {
        let t: RBTree<i32, i32> = RBTree::new();
        assert_eq!(t.iter().count(), 0);
    }

    #[test]
    fn iter_returns_all_pairs_in_order() {
        let t = build_tree(&[5, 3, 8, 1, 4, 7, 9, 2, 6, 10]);
        let keys: Vec<i32> = t.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn range_returns_pairs_in_order_with_bounds() {
        let t = build_tree(&[5, 3, 8, 1, 4, 7, 9, 2, 6, 10]);
        let pairs: Vec<_> = t.range(&3, &7).collect();
        let keys: Vec<i32> = pairs.iter().map(|(k, _)| **k).collect();
        assert_eq!(keys, vec![3, 4, 5, 6, 7]);
    }

    #[test]
    fn range_inclusive_boundaries() {
        let t = build_tree(&[1, 2, 3, 4, 5]);
        let keys: Vec<_> = t.range(&1, &5).map(|(k, _)| *k).collect();
        assert_eq!(keys, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn range_empty_when_no_keys_in_bounds() {
        let t = build_tree(&[1, 2, 3, 10, 11]);
        let keys: Vec<_> = t.range(&5, &9).map(|(k, _)| *k).collect();
        assert!(keys.is_empty());
    }

    #[test]
    fn range_pruning_skips_out_of_bound_subtrees() {
        let t = build_tree(&(1..=20).collect::<Vec<_>>());
        let mut visit_count = 0;
        let keys: Vec<_> = InOrderIterator::new(&t.root, Some(&8), Some(&12))
            .inspect(|_| visit_count += 1)
            .map(|(k, _)| *k)
            .collect();
        assert_eq!(keys, vec![8, 9, 10, 11, 12]);
        // com poda, visitamos apenas os k=5 nós do range
        assert_eq!(visit_count, 5);
    }

    #[test]
    fn iter_15_nodes_all_in_order() {
        let t = build_tree(&(1..=15).collect::<Vec<_>>());
        let keys: Vec<i32> = t.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, (1..=15).collect::<Vec<_>>());
    }
}
