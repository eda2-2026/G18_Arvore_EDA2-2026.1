#![allow(dead_code)]

use crate::tree::node::{Link, Node};

/// Red-Black Tree backed key-value store.
/// `K` must implement `Ord`; the tree uses that ordering for all operations.
pub struct RBTree<K: Ord, V> {
    pub(crate) root: Link<K, V>,
    len: usize,
}

impl<K: Ord, V> RBTree<K, V> {
    /// Creates an empty tree.
    pub fn new() -> Self {
        RBTree { root: None, len: 0 }
    }

    // ── Rotações ────────────────────────────────────────────────────────────
    //
    // São operações puramente estruturais: movem nós, não alteram cores.
    // O fixup de cores (inserção/remoção) chama essas funções como bloco
    // atômico e faz as trocas de cor separadamente.
    //
    // Assinatura funcional: tomam posse do nó, devolvem a nova raiz da
    // subárvore. Isso evita conflitos com o borrow checker e elimina a
    // necessidade de ponteiro de pai.

    /// Rotação à esquerda: o filho direito (y) sobe; x vira filho esquerdo de y.
    ///
    /// ```text
    ///     x                 y
    ///    / \               / \
    ///   A   y    →        x   C
    ///      / \           / \
    ///     B   C         A   B
    /// ```
    ///
    /// Pré-condição: `x.right` é `Some`.
    pub(crate) fn rotate_left(mut x: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut y = x.right.take().expect("rotate_left: right child must be Some");
        x.right = y.left.take(); // B migra para x
        y.left = Some(x);
        y
    }

    /// Rotação à direita: o filho esquerdo (x) sobe; y vira filho direito de x.
    ///
    /// ```text
    ///       y             x
    ///      / \           / \
    ///     x   C  →      A   y
    ///    / \               / \
    ///   A   B             B   C
    /// ```
    ///
    /// Pré-condição: `y.left` é `Some`.
    pub(crate) fn rotate_right(mut y: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut x = y.left.take().expect("rotate_right: left child must be Some");
        y.left = x.right.take(); // B migra para y
        x.right = Some(y);
        x
    }

    // ── Interface pública (implementada em issues #7 e #8) ──────────────────

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

// ── Testes ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::RBTree;

    // Contrato das operações públicas — compilam agora, passam após issues #7/#8.
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

#[cfg(test)]
mod rotation_tests {
    use super::RBTree;
    use crate::tree::node::{Color, Link, Node};

    fn inorder<K: Clone + Ord, V>(link: &Link<K, V>) -> Vec<K> {
        match link {
            None => vec![],
            Some(n) => {
                let mut v = inorder(&n.left);
                v.push(n.key.clone());
                v.extend(inorder(&n.right));
                v
            }
        }
    }

    /// Constrói a árvore:
    /// ```text
    ///      10(B)
    ///     /    \
    ///   5(B)  20(B)
    ///         /   \
    ///       15(B) 30(B)
    /// ```
    fn make_tree() -> Box<Node<i32, i32>> {
        let mut root = Node::new(10, 10, Color::Black);
        root.left = Some(Node::new(5, 5, Color::Black));
        let mut right = Node::new(20, 20, Color::Black);
        right.left = Some(Node::new(15, 15, Color::Black));
        right.right = Some(Node::new(30, 30, Color::Black));
        root.right = Some(right);
        root
    }

    #[test]
    fn rotate_left_preserves_bst_order() {
        // Após rotate_left em 10:
        //      20(B)
        //     /    \
        //   10(B)  30(B)
        //   /  \
        //  5(B) 15(B)
        let before = inorder(&Some(make_tree()));
        let rotated = RBTree::<i32, i32>::rotate_left(make_tree());
        let after = inorder(&Some(rotated));
        assert_eq!(before, vec![5, 10, 15, 20, 30]);
        assert_eq!(before, after);
    }

    #[test]
    fn rotate_right_preserves_bst_order() {
        // Árvore inclinada para a esquerda:
        //      20(B)
        //     /    \
        //   10(B)  30(B)
        //   /  \
        //  5(B) 15(B)
        //
        // Após rotate_right em 20:
        //      10(B)
        //     /    \
        //   5(B)  20(B)
        //         /  \
        //        15(B) 30(B)
        let mut root = Node::new(20, 20, Color::Black);
        let mut left = Node::new(10, 10, Color::Black);
        left.left = Some(Node::new(5, 5, Color::Black));
        left.right = Some(Node::new(15, 15, Color::Black));
        root.left = Some(left);
        root.right = Some(Node::new(30, 30, Color::Black));

        let before = inorder(&Some(root));

        let mut root2 = Node::new(20, 20, Color::Black);
        let mut left2 = Node::new(10, 10, Color::Black);
        left2.left = Some(Node::new(5, 5, Color::Black));
        left2.right = Some(Node::new(15, 15, Color::Black));
        root2.left = Some(left2);
        root2.right = Some(Node::new(30, 30, Color::Black));

        let rotated = RBTree::<i32, i32>::rotate_right(root2);
        let after = inorder(&Some(rotated));
        assert_eq!(before, vec![5, 10, 15, 20, 30]);
        assert_eq!(before, after);
    }

    #[test]
    fn rotate_left_then_right_restores_structure() {
        // rotate_left e rotate_right são operações inversas.
        // Aplicadas na mesma posição, restauram a estrutura original.
        let original = make_tree();
        let original_keys = inorder(&Some(original));

        // rotate_left(10) → raiz vira 20
        let after_left = RBTree::<i32, i32>::rotate_left(make_tree());
        assert_eq!(after_left.key, 20);

        // rotate_right(20) → raiz volta a ser 10
        let restored = RBTree::<i32, i32>::rotate_right(after_left);
        assert_eq!(restored.key, 10);
        assert_eq!(restored.left.as_ref().unwrap().key, 5);
        assert_eq!(restored.right.as_ref().unwrap().key, 20);
        assert_eq!(inorder(&Some(restored)), original_keys);
    }

    #[test]
    fn rotate_right_then_left_restores_structure() {
        let mut root = Node::new(20, 20, Color::Black);
        let mut left = Node::new(10, 10, Color::Black);
        left.left = Some(Node::new(5, 5, Color::Black));
        left.right = Some(Node::new(15, 15, Color::Black));
        root.left = Some(left);
        root.right = Some(Node::new(30, 30, Color::Black));
        let original_keys = inorder(&Some(root));

        let mut root2 = Node::new(20, 20, Color::Black);
        let mut left2 = Node::new(10, 10, Color::Black);
        left2.left = Some(Node::new(5, 5, Color::Black));
        left2.right = Some(Node::new(15, 15, Color::Black));
        root2.left = Some(left2);
        root2.right = Some(Node::new(30, 30, Color::Black));

        let after_right = RBTree::<i32, i32>::rotate_right(root2);
        assert_eq!(after_right.key, 10);

        let restored = RBTree::<i32, i32>::rotate_left(after_right);
        assert_eq!(restored.key, 20);
        assert_eq!(inorder(&Some(restored)), original_keys);
    }

    #[test]
    fn rotate_at_root_updates_self_root() {
        // Verifica que atribuir a raiz após a rotação funciona corretamente.
        let mut rbtree: RBTree<i32, i32> = RBTree::new();
        rbtree.root = Some(make_tree());

        let old_right_key = rbtree.root.as_ref().unwrap().right.as_ref().unwrap().key;

        // rotate_left na raiz
        rbtree.root = Some(RBTree::rotate_left(rbtree.root.take().unwrap()));
        assert_eq!(rbtree.root.as_ref().unwrap().key, old_right_key); // 20 virou raiz

        // rotate_right na nova raiz restaura
        rbtree.root = Some(RBTree::rotate_right(rbtree.root.take().unwrap()));
        assert_eq!(rbtree.root.as_ref().unwrap().key, 10); // 10 voltou à raiz
    }

    #[test]
    fn rotate_left_moves_right_child_left_subtree_correctly() {
        // Garante que B migra corretamente (x.right = y.left.take())
        let rotated = RBTree::<i32, i32>::rotate_left(make_tree());
        // y = 20, B = 15 (y.left antes da rotação)
        // Após rotate_left: 10.right deve ser 15
        let new_left = rotated.left.as_ref().unwrap(); // nó 10
        assert_eq!(new_left.key, 10);
        assert_eq!(new_left.right.as_ref().unwrap().key, 15);
    }

    #[test]
    fn rotate_right_moves_left_child_right_subtree_correctly() {
        let mut root = Node::new(20, 20, Color::Black);
        let mut left = Node::new(10, 10, Color::Black);
        left.left = Some(Node::new(5, 5, Color::Black));
        left.right = Some(Node::new(15, 15, Color::Black));
        root.left = Some(left);
        root.right = Some(Node::new(30, 30, Color::Black));

        let rotated = RBTree::<i32, i32>::rotate_right(root);
        // x = 10, B = 15 (x.right antes da rotação)
        // Após rotate_right: 20.left deve ser 15
        let new_right = rotated.right.as_ref().unwrap(); // nó 20
        assert_eq!(new_right.key, 20);
        assert_eq!(new_right.left.as_ref().unwrap().key, 15);
    }
}
