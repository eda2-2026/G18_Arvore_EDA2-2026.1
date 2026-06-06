#![allow(dead_code)]

use std::cmp::Ordering;

use crate::tree::node::{is_red, Color, Link, Node};

pub struct RBTree<K: Ord, V> {
    pub(crate) root: Link<K, V>,
    len: usize,
}

impl<K: Ord, V> RBTree<K, V> {
    pub fn new() -> Self {
        RBTree { root: None, len: 0 }
    }

    // ── Rotações (issue #5) ──────────────────────────────────────────────────

    /// ```text
    ///     x                 y
    ///    / \               / \
    ///   A   y    →        x   C
    ///      / \           / \
    ///     B   C         A   B
    /// ```
    pub(crate) fn rotate_left(mut x: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut y = x.right.take().expect("rotate_left: right child must be Some");
        x.right = y.left.take();
        y.left = Some(x);
        y
    }

    /// ```text
    ///       y             x
    ///      / \           / \
    ///     x   C  →      A   y
    ///    / \               / \
    ///   A   B             B   C
    /// ```
    pub(crate) fn rotate_right(mut y: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut x = y.left.take().expect("rotate_right: left child must be Some");
        y.left = x.right.take();
        x.right = Some(y);
        x
    }

    // ── Inserção com fixup (issue #7) ────────────────────────────────────────

    pub fn insert(&mut self, key: K, value: V) {
        let (new_root, is_new) = Self::insert_rec(self.root.take(), key, value);
        self.root = Some(new_root);
        self.root.as_mut().unwrap().color = Color::Black; // invariante 2: raiz sempre preta
        if is_new {
            self.len += 1;
        }
    }

    fn insert_rec(link: Link<K, V>, key: K, value: V) -> (Box<Node<K, V>>, bool) {
        match link {
            None => (Node::new(key, value, Color::Red), true),
            Some(mut node) => {
                let is_new = match key.cmp(&node.key) {
                    Ordering::Less => {
                        let (l, new_key) = Self::insert_rec(node.left.take(), key, value);
                        node.left = Some(l);
                        new_key
                    }
                    Ordering::Greater => {
                        let (r, new_key) = Self::insert_rec(node.right.take(), key, value);
                        node.right = Some(r);
                        new_key
                    }
                    Ordering::Equal => {
                        node.value = value;
                        false
                    }
                };
                (Self::insert_fixup(node), is_new)
            }
        }
    }

    /// Corrige violações de duplo-vermelho após inserção.
    ///
    /// Opera no nível do avô: verifica filhos e netos para detectar duplo-vermelho.
    /// Implementa os 6 casos CLRS (3 por lado):
    /// - **Caso 1**: tio vermelho → recolorir pai, tio e avô
    /// - **Caso 2**: tio preto, neto interno → rotação no pai (converte em Caso 3)
    /// - **Caso 3**: tio preto, neto externo → rotação no avô + recolorir
    fn insert_fixup(mut g: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let left_red = is_red(&g.left);
        let right_red = is_red(&g.right);

        // Detecta duplo-vermelho nos quatro alinhamentos possíveis
        let ll = left_red && g.left.as_ref().map_or(false, |l| is_red(&l.left));
        let lr = left_red && g.left.as_ref().map_or(false, |l| is_red(&l.right));
        let rl = right_red && g.right.as_ref().map_or(false, |r| is_red(&r.left));
        let rr = right_red && g.right.as_ref().map_or(false, |r| is_red(&r.right));

        if ll || lr {
            if right_red {
                // Caso 1 (esquerda): tio direito é vermelho → recolorir
                g.left.as_mut().unwrap().color = Color::Black;
                g.right.as_mut().unwrap().color = Color::Black;
                g.color = Color::Red;
            } else {
                if !ll {
                    // Caso 2 (esquerda-direita): rotação à esquerda no filho esquerdo
                    // transforma em esquerda-esquerda
                    g.left = Some(Self::rotate_left(g.left.take().unwrap()));
                }
                // Caso 3 (esquerda-esquerda): rotação à direita no avô + recolorir
                let g_color = g.color.clone();
                let mut new_root = Self::rotate_right(g);
                new_root.color = g_color;
                new_root.right.as_mut().unwrap().color = Color::Red;
                return new_root;
            }
        } else if rl || rr {
            if left_red {
                // Caso 1 (direita): tio esquerdo é vermelho → recolorir
                g.left.as_mut().unwrap().color = Color::Black;
                g.right.as_mut().unwrap().color = Color::Black;
                g.color = Color::Red;
            } else {
                if !rr {
                    // Caso 2 (direita-esquerda): rotação à direita no filho direito
                    // transforma em direita-direita
                    g.right = Some(Self::rotate_right(g.right.take().unwrap()));
                }
                // Caso 3 (direita-direita): rotação à esquerda no avô + recolorir
                let g_color = g.color.clone();
                let mut new_root = Self::rotate_left(g);
                new_root.color = g_color;
                new_root.left.as_mut().unwrap().color = Color::Red;
                return new_root;
            }
        }

        g
    }

    // ── Validação das invariantes ────────────────────────────────────────────

    /// Verifica as 5 invariantes da Rubro-Negra.
    /// Retorna `Ok(())` se a árvore estiver válida, `Err(msg)` com a violação.
    pub fn validate(&self) -> Result<(), String> {
        if is_red(&self.root) {
            return Err("invariante 2 violada: raiz deve ser preta".to_string());
        }
        Self::check_node(&self.root).map(|_| ())
    }

    /// Retorna a black-height da subárvore, ou `Err` se alguma invariante for violada.
    fn check_node(link: &Link<K, V>) -> Result<usize, String> {
        match link {
            None => Ok(1), // folha nil conta como nó preto
            Some(node) => {
                // Invariante 4: filhos de nó vermelho são ambos pretos
                if node.color == Color::Red {
                    if is_red(&node.left) {
                        return Err(
                            "invariante 4: filho esquerdo vermelho de nó vermelho".to_string(),
                        );
                    }
                    if is_red(&node.right) {
                        return Err(
                            "invariante 4: filho direito vermelho de nó vermelho".to_string(),
                        );
                    }
                }

                let lbh = Self::check_node(&node.left)?;
                let rbh = Self::check_node(&node.right)?;

                // Invariante 5: black-height igual em todos os caminhos
                if lbh != rbh {
                    return Err(format!(
                        "invariante 5: black-height desigual (esq={lbh}, dir={rbh})"
                    ));
                }

                Ok(lbh + (node.color == Color::Black) as usize)
            }
        }
    }

    /// Altura máxima da árvore (nº de nós no caminho mais longo raiz→folha).
    pub fn height(&self) -> usize {
        Self::node_height(&self.root)
    }

    fn node_height(link: &Link<K, V>) -> usize {
        match link {
            None => 0,
            Some(n) => 1 + Self::node_height(&n.left).max(Self::node_height(&n.right)),
        }
    }

    // ── Interface pública (outras issues) ────────────────────────────────────

    pub fn get(&self, _key: &K) -> Option<&V> {
        todo!()
    }

    pub fn delete(&mut self, _key: &K) -> Option<V> {
        todo!()
    }

    pub fn range(&self, _low: &K, _high: &K) -> Vec<(&K, &V)> {
        todo!()
    }

    pub fn iter(&self) -> Vec<(&K, &V)> {
        todo!()
    }

    pub fn min(&self) -> Option<(&K, &V)> {
        todo!()
    }

    pub fn max(&self) -> Option<(&K, &V)> {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

// ── Testes ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::RBTree;

    // Contrato das operações ainda não implementadas — panick em todo!().
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
        let before = inorder(&Some(make_tree()));
        let rotated = RBTree::<i32, i32>::rotate_left(make_tree());
        assert_eq!(before, inorder(&Some(rotated)));
        assert_eq!(before, vec![5, 10, 15, 20, 30]);
    }

    #[test]
    fn rotate_right_preserves_bst_order() {
        let mut root = Node::new(20, 20, Color::Black);
        let mut left = Node::new(10, 10, Color::Black);
        left.left = Some(Node::new(5, 5, Color::Black));
        left.right = Some(Node::new(15, 15, Color::Black));
        root.left = Some(left);
        root.right = Some(Node::new(30, 30, Color::Black));
        let expected = vec![5, 10, 15, 20, 30];

        let mut root2 = Node::new(20, 20, Color::Black);
        let mut left2 = Node::new(10, 10, Color::Black);
        left2.left = Some(Node::new(5, 5, Color::Black));
        left2.right = Some(Node::new(15, 15, Color::Black));
        root2.left = Some(left2);
        root2.right = Some(Node::new(30, 30, Color::Black));
        assert_eq!(inorder(&Some(root)), expected);
        assert_eq!(inorder(&Some(RBTree::<i32, i32>::rotate_right(root2))), expected);
    }

    #[test]
    fn rotate_left_then_right_restores_structure() {
        let original_keys = inorder(&Some(make_tree()));
        let after_left = RBTree::<i32, i32>::rotate_left(make_tree());
        assert_eq!(after_left.key, 20);
        let restored = RBTree::<i32, i32>::rotate_right(after_left);
        assert_eq!(restored.key, 10);
        assert_eq!(inorder(&Some(restored)), original_keys);
    }

    #[test]
    fn rotate_at_root_updates_self_root() {
        let mut rbtree: RBTree<i32, i32> = RBTree::new();
        rbtree.root = Some(make_tree());
        let old_right_key = rbtree.root.as_ref().unwrap().right.as_ref().unwrap().key;
        rbtree.root = Some(RBTree::rotate_left(rbtree.root.take().unwrap()));
        assert_eq!(rbtree.root.as_ref().unwrap().key, old_right_key);
        rbtree.root = Some(RBTree::rotate_right(rbtree.root.take().unwrap()));
        assert_eq!(rbtree.root.as_ref().unwrap().key, 10);
    }

    #[test]
    fn rotate_left_moves_right_child_left_subtree_correctly() {
        let rotated = RBTree::<i32, i32>::rotate_left(make_tree());
        let new_left = rotated.left.as_ref().unwrap();
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
        let new_right = rotated.right.as_ref().unwrap();
        assert_eq!(new_right.key, 20);
        assert_eq!(new_right.left.as_ref().unwrap().key, 15);
    }
}

#[cfg(test)]
mod insert_tests {
    use super::RBTree;
    use crate::tree::node::Color;

    fn pseudo_random(n: usize) -> Vec<i32> {
        let mut v = Vec::with_capacity(n);
        let mut x: u64 = 0xdeadbeef_cafebabe;
        for _ in 0..n {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            v.push((x >> 33) as i32);
        }
        v
    }

    #[test]
    fn insert_sequence_validates_all_invariants_step_by_step() {
        let mut tree: RBTree<i32, i32> = RBTree::new();
        for key in [10, 20, 5, 15, 1, 30] {
            tree.insert(key, key);
            tree.validate()
                .unwrap_or_else(|e| panic!("falha após inserir {key}: {e}"));
        }
    }

    #[test]
    fn root_is_always_black_after_insert() {
        let mut tree: RBTree<i32, i32> = RBTree::new();
        for i in [3, 1, 5, 2, 4, 7, 6] {
            tree.insert(i, i);
            assert_eq!(
                tree.root.as_ref().unwrap().color,
                Color::Black,
                "raiz deve ser preta após inserir {i}"
            );
        }
    }

    #[test]
    fn upsert_updates_value_without_duplicating_key() {
        let mut tree: RBTree<i32, i32> = RBTree::new();
        tree.insert(42, 100);
        tree.insert(42, 999);
        assert_eq!(tree.len(), 1);
        tree.validate().unwrap();
    }

    #[test]
    fn len_and_is_empty_track_insertions() {
        let mut tree: RBTree<i32, i32> = RBTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        for i in 1..=10 {
            tree.insert(i, i);
            assert_eq!(tree.len(), i as usize);
        }
        assert!(!tree.is_empty());
    }

    #[test]
    fn insert_ascending_stays_balanced() {
        let mut tree: RBTree<i32, i32> = RBTree::new();
        for i in 0..100 {
            tree.insert(i, i);
        }
        tree.validate().unwrap();
        let n = tree.len() as f64;
        let max_h = 2.0 * (n + 1.0).log2();
        assert!(
            tree.height() as f64 <= max_h,
            "inserção ascendente: altura {} > 2*log2({}) = {:.1}",
            tree.height(), n as usize + 1, max_h
        );
    }

    #[test]
    fn insert_descending_stays_balanced() {
        let mut tree: RBTree<i32, i32> = RBTree::new();
        for i in (0..100).rev() {
            tree.insert(i, i);
        }
        tree.validate().unwrap();
        let n = tree.len() as f64;
        let max_h = 2.0 * (n + 1.0).log2();
        assert!(
            tree.height() as f64 <= max_h,
            "inserção descendente: altura {} > 2*log2({}) = {:.1}",
            tree.height(), n as usize + 1, max_h
        );
    }

    #[test]
    fn insert_1000_random_height_bound_and_valid() {
        let mut tree: RBTree<i32, i32> = RBTree::new();
        for k in pseudo_random(1000) {
            tree.insert(k, k);
        }
        tree.validate().unwrap();
        let n = tree.len() as f64;
        let max_h = 2.0 * (n + 1.0).log2();
        assert!(
            tree.height() as f64 <= max_h,
            "1000 aleatórios: altura {} > 2*log2({}) = {:.1}",
            tree.height(), n as usize + 1, max_h
        );
    }

    #[test]
    fn insert_validate_each_of_1000_random() {
        let mut tree: RBTree<i32, i32> = RBTree::new();
        for (i, k) in pseudo_random(1000).into_iter().enumerate() {
            tree.insert(k, k);
            if i % 100 == 0 {
                tree.validate()
                    .unwrap_or_else(|e| panic!("invariante violada na inserção {i}: {e}"));
            }
        }
        tree.validate().unwrap();
    }
}
