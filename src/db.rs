use std::collections::BTreeMap;

// Para trocar pelo RBTree real, altere apenas esta linha (e os corpos dos
// métodos abaixo que chamam a API do BTreeMap) — toda a lógica fica em db.rs.
type Store = BTreeMap<String, String>;

pub struct Database {
    store: Store,
}

impl Database {
    pub fn new() -> Self {
        Database {
            store: BTreeMap::new(),
        }
    }

    /// Insere ou atualiza o par chave-valor. Retorna "OK".
    pub fn set(&mut self, key: String, value: String) -> String {
        self.store.insert(key, value);
        "OK".to_string()
    }

    /// Retorna o valor associado à chave, ou "(nil)" se ausente.
    pub fn get(&self, key: &str) -> String {
        match self.store.get(key) {
            Some(v) => format!("\"{v}\""),
            None => "(nil)".to_string(),
        }
    }

    /// Remove a chave e retorna "OK", ou "(nil)" se ela não existia.
    pub fn delete(&mut self, key: &str) -> String {
        match self.store.remove(key) {
            Some(_) => "OK".to_string(),
            None => "(nil)".to_string(),
        }
    }

    /// Retorna todos os pares com chave em [low, high], um por linha, em ordem crescente.
    /// Complexidade com RBTree: O(log n + k). Com BTreeMap: O(log n + k) também.
    pub fn range(&self, low: &str, high: &str) -> String {
        if low > high {
            return "(empty)".to_string();
        }
        let results: Vec<String> = self
            .store
            .range(low.to_owned()..=high.to_owned())
            .map(|(k, v)| format!("{k} -> \"{v}\""))
            .collect();
        if results.is_empty() {
            "(empty)".to_string()
        } else {
            results.join("\n")
        }
    }

    /// Lista todas as chaves em ordem crescente, uma por linha.
    pub fn keys(&self) -> String {
        if self.store.is_empty() {
            return "(empty)".to_string();
        }
        self.store
            .keys()
            .map(|k| k.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Retorna o par com a menor chave, ou "(nil)" se o banco estiver vazio.
    pub fn min(&self) -> String {
        match self.store.iter().next() {
            Some((k, v)) => format!("{k} -> \"{v}\""),
            None => "(nil)".to_string(),
        }
    }

    /// Retorna o par com a maior chave, ou "(nil)" se o banco estiver vazio.
    pub fn max(&self) -> String {
        match self.store.iter().next_back() {
            Some((k, v)) => format!("{k} -> \"{v}\""),
            None => "(nil)".to_string(),
        }
    }

    /// Retorna um iterador sobre todos os pares — usado pela camada de persistência (issue #12).
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.store.iter()
    }

    /// Implementado na issue #12 (persistence).
    pub fn save(&self) -> String {
        "(not implemented)".to_string()
    }

    /// Implementado na issue #12 (persistence).
    pub fn load(&mut self) -> String {
        "(not implemented)".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn populated() -> Database {
        let mut db = Database::new();
        // Inserção fora de ordem intencional — verifica que BTreeMap/RBTree ordena.
        db.set("carlos".to_string(), "3".to_string());
        db.set("ana".to_string(), "1".to_string());
        db.set("bruno".to_string(), "2".to_string());
        db
    }

    #[test]
    fn set_and_get_roundtrip() {
        let mut db = Database::new();
        assert_eq!(db.set("nome".to_string(), "Gabriel".to_string()), "OK");
        assert_eq!(db.get("nome"), "\"Gabriel\"");
    }

    #[test]
    fn get_missing_key_returns_nil() {
        assert_eq!(Database::new().get("nada"), "(nil)");
    }

    #[test]
    fn set_upsert_replaces_value() {
        let mut db = Database::new();
        db.set("k".to_string(), "v1".to_string());
        db.set("k".to_string(), "v2".to_string());
        assert_eq!(db.get("k"), "\"v2\"");
    }

    #[test]
    fn delete_existing_key() {
        let mut db = Database::new();
        db.set("k".to_string(), "v".to_string());
        assert_eq!(db.delete("k"), "OK");
        assert_eq!(db.get("k"), "(nil)");
    }

    #[test]
    fn delete_missing_key_returns_nil() {
        assert_eq!(Database::new().delete("nada"), "(nil)");
    }

    #[test]
    fn range_returns_pairs_in_order() {
        let db = populated();
        let result = db.range("ana", "bruno");
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].starts_with("ana"));
        assert!(lines[1].starts_with("bruno"));
    }

    #[test]
    fn range_excludes_keys_outside_bounds() {
        let db = populated();
        let result = db.range("ana", "bruno");
        assert!(!result.contains("carlos"));
    }

    #[test]
    fn range_empty_returns_empty_marker() {
        assert_eq!(populated().range("x", "z"), "(empty)");
    }

    #[test]
    fn range_inverted_bounds_returns_empty() {
        assert_eq!(populated().range("z", "a"), "(empty)");
    }

    #[test]
    fn keys_returns_sorted_order() {
        let result = populated().keys();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines, vec!["ana", "bruno", "carlos"]);
    }

    #[test]
    fn keys_empty_db() {
        assert_eq!(Database::new().keys(), "(empty)");
    }

    #[test]
    fn min_returns_smallest_key() {
        assert!(populated().min().starts_with("ana"));
    }

    #[test]
    fn max_returns_largest_key() {
        assert!(populated().max().starts_with("carlos"));
    }

    #[test]
    fn min_max_empty_db() {
        assert_eq!(Database::new().min(), "(nil)");
        assert_eq!(Database::new().max(), "(nil)");
    }

    #[test]
    fn iter_yields_all_pairs_in_order() {
        let db = populated();
        let pairs: Vec<(&String, &String)> = db.iter().collect();
        assert_eq!(pairs.len(), 3);
        assert_eq!(pairs[0].0, "ana");
        assert_eq!(pairs[1].0, "bruno");
        assert_eq!(pairs[2].0, "carlos");
    }
}
