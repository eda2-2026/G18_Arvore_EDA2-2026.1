use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub const DUMP_PATH: &str = "data/dump.rdb";

/// Persiste os pares em `DUMP_PATH` como script de comandos SET.
pub fn save(pairs: &[(String, String)]) -> String {
    save_to(DUMP_PATH, pairs)
}

/// Lê `DUMP_PATH` e retorna os pares. `Ok(vec![])` se o arquivo não existe.
pub fn load() -> Result<Vec<(String, String)>, String> {
    load_from(DUMP_PATH)
}

// --- helpers com path parametrizado (reutilizados nos testes) ---

fn save_to(path: &str, pairs: &[(String, String)]) -> String {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(e) = fs::create_dir_all(parent) {
                return format!("ERR: could not create directory: {e}");
            }
        }
    }
    let content: String = pairs
        .iter()
        .map(|(k, v)| format!("SET {k} {v}\n"))
        .collect();
    match fs::write(path, content) {
        Ok(_) => format!("Persisted {} keys to {path}", pairs.len()),
        Err(e) => format!("ERR: could not write to {path}: {e}"),
    }
}

fn load_from(path: &str) -> Result<Vec<(String, String)>, String> {
    if !Path::new(path).exists() {
        return Ok(vec![]);
    }
    let file = fs::File::open(path)
        .map_err(|e| format!("ERR: could not open {path}: {e}"))?;
    let mut pairs = Vec::new();
    for (i, line) in BufReader::new(file).lines().enumerate() {
        let line = line.map_err(|e| format!("ERR: read error at line {}: {e}", i + 1))?;
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }
        // Formato esperado: SET <key> <value>
        // splitn(3) preserva espaços no valor: "SET nome Gabriel Zanetti"
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() != 3 || parts[0] != "SET" {
            return Err(format!("ERR: malformed line {} in {path}", i + 1));
        }
        pairs.push((parts[1].to_string(), parts[2].to_string()));
    }
    Ok(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_load_roundtrip() {
        let path = "data/test_roundtrip.rdb";
        let _ = fs::remove_file(path);
        let pairs = vec![
            ("cidade".to_string(), "Brasilia".to_string()),
            ("nome".to_string(), "Gabriel".to_string()),
        ];
        let msg = save_to(path, &pairs);
        assert!(msg.contains("Persisted 2"), "unexpected: {msg}");
        let loaded = load_from(path).unwrap();
        assert_eq!(loaded, pairs);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn load_nonexistent_file_returns_empty_ok() {
        let result = load_from("data/definitely_does_not_exist_xyz.rdb").unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn save_empty_db_creates_empty_file() {
        let path = "data/test_empty_save.rdb";
        let _ = fs::remove_file(path);
        let msg = save_to(path, &[]);
        assert!(msg.contains("Persisted 0"), "unexpected: {msg}");
        let loaded = load_from(path).unwrap();
        assert!(loaded.is_empty());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn value_with_spaces_survives_roundtrip() {
        let path = "data/test_spaces.rdb";
        let _ = fs::remove_file(path);
        let pairs = vec![("nome".to_string(), "Gabriel Zanetti".to_string())];
        save_to(path, &pairs);
        let loaded = load_from(path).unwrap();
        assert_eq!(loaded[0].1, "Gabriel Zanetti");
        let _ = fs::remove_file(path);
    }

    #[test]
    fn file_is_readable_set_script() {
        let path = "data/test_fmt.rdb";
        let _ = fs::remove_file(path);
        let pairs = vec![("chave".to_string(), "valor".to_string())];
        save_to(path, &pairs);
        let content = fs::read_to_string(path).unwrap();
        assert_eq!(content.trim(), "SET chave valor");
        let _ = fs::remove_file(path);
    }

    #[test]
    fn n_pairs_roundtrip() {
        // Critério: salvar N pares e verificar que todos sobrevivem ao ciclo save→load.
        let path = "data/test_n_pairs.rdb";
        let _ = fs::remove_file(path);
        let pairs: Vec<(String, String)> = (0..20)
            .map(|i| (format!("key{i:02}"), format!("val{i}")))
            .collect();
        save_to(path, &pairs);
        let loaded = load_from(path).unwrap();
        assert_eq!(loaded.len(), 20);
        for (i, (k, v)) in loaded.iter().enumerate() {
            assert_eq!(k, &format!("key{i:02}"));
            assert_eq!(v, &format!("val{i}"));
        }
        let _ = fs::remove_file(path);
    }
}
