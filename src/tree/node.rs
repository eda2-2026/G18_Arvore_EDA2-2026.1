// Estratégia de ponteiros: Box<Node<K, V>> com Option como sentinela de nil.
//
// Alternativas consideradas:
//   - Rc<RefCell<Node>>: necessário para ponteiros de pai bidirecional,
//     mas adiciona overhead de contagem de referências e borrow dinâmico.
//     O algoritmo de remoção implementado aqui usa apenas descida top-down
//     e não precisa de ponteiro de pai.
//   - Raw pointers (*mut Node): elimina overhead mas requer unsafe em toda
//     a árvore. Não vale o custo de manutenção em contexto acadêmico.
//   - Box<Node>: seguro, zero-overhead de alocação extra, compatível com
//     move semantics do Rust. As rotações tomam posse dos nós via swap,
//     evitando restrições do borrow checker.

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Black,
}

pub type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    pub color: Color,
    pub left: Link<K, V>,
    pub right: Link<K, V>,
}

impl<K: Ord, V> Node<K, V> {
    pub fn new(key: K, value: V, color: Color) -> Box<Self> {
        Box::new(Node {
            key,
            value,
            color,
            left: None,
            right: None,
        })
    }
}

/// Retorna `true` se o nó é vermelho; `None` é tratado como preto (folha nil).
pub fn is_red<K, V>(node: &Link<K, V>) -> bool {
    matches!(node, Some(n) if n.color == Color::Red)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_is_black() {
        let nil: Link<i32, i32> = None;
        assert!(!is_red(&nil));
    }

    #[test]
    fn red_node_is_red() {
        let node: Link<i32, i32> = Some(Node::new(1, 10, Color::Red));
        assert!(is_red(&node));
    }

    #[test]
    fn black_node_is_not_red() {
        let node: Link<i32, i32> = Some(Node::new(1, 10, Color::Black));
        assert!(!is_red(&node));
    }

    #[test]
    fn color_implements_partial_eq_and_clone() {
        assert_eq!(Color::Red, Color::Red.clone());
        assert_eq!(Color::Black, Color::Black.clone());
        assert_ne!(Color::Red, Color::Black);
    }

    #[test]
    fn node_fields_accessible() {
        let n = Node::new("chave", 42, Color::Black);
        assert_eq!(n.key, "chave");
        assert_eq!(n.value, 42);
        assert_eq!(n.color, Color::Black);
        assert!(n.left.is_none());
        assert!(n.right.is_none());
    }
}
