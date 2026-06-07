use rbtree_db::db;
use rbtree_db::persistence;

use std::io::{self, Write};

use db::Database;

#[derive(Debug)]
enum Command {
    Set(String, String),
    Get(String),
    Delete(String),
    Range(String, String),
    Keys,
    Min,
    Max,
    Save,
    Load,
    Exit,
}

/// Parses a raw input line into a `Command`, or returns a descriptive error string.
fn parse_command(input: &str) -> Result<Command, String> {
    let (cmd_str, rest) = match input.find(' ') {
        Some(i) => (&input[..i], input[i + 1..].trim()),
        None => (input, ""),
    };

    match cmd_str.to_uppercase().as_str() {
        "SET" => {
            let (key, value) = match rest.find(' ') {
                Some(i) => (&rest[..i], rest[i + 1..].trim()),
                None => return Err("usage: SET <key> <value>".to_string()),
            };
            if value.is_empty() {
                return Err("usage: SET <key> <value>".to_string());
            }
            Ok(Command::Set(key.to_string(), value.to_string()))
        }
        "GET" => {
            if rest.is_empty() {
                return Err("usage: GET <key>".to_string());
            }
            Ok(Command::Get(rest.to_string()))
        }
        "DELETE" => {
            if rest.is_empty() {
                return Err("usage: DELETE <key>".to_string());
            }
            Ok(Command::Delete(rest.to_string()))
        }
        "RANGE" => {
            let mut parts = rest.split_whitespace();
            let low = parts
                .next()
                .ok_or("usage: RANGE <key1> <key2>".to_string())?
                .to_string();
            let high = parts
                .next()
                .ok_or("usage: RANGE <key1> <key2>".to_string())?
                .to_string();
            Ok(Command::Range(low, high))
        }
        "KEYS" => Ok(Command::Keys),
        "MIN" => Ok(Command::Min),
        "MAX" => Ok(Command::Max),
        "SAVE" => Ok(Command::Save),
        "LOAD" => Ok(Command::Load),
        "EXIT" => Ok(Command::Exit),
        unknown => Err(format!("unknown command '{unknown}'")),
    }
}

fn execute(db: &mut Database, cmd: Command) -> String {
    match cmd {
        Command::Set(k, v) => db.set(k, v),
        Command::Get(k) => db.get(&k),
        Command::Delete(k) => db.delete(&k),
        Command::Range(low, high) => db.range(&low, &high),
        Command::Keys => db.keys(),
        Command::Min => db.min(),
        Command::Max => db.max(),
        Command::Save => db.save(),
        Command::Load => db.load(),
        Command::Exit => unreachable!(),
    }
}

fn main() {
    let mut db = Database::new();

    // Restaura automaticamente do último SAVE — silencioso se não houver arquivo.
    if std::path::Path::new(persistence::DUMP_PATH).exists() {
        match persistence::load() {
            Ok(pairs) if !pairs.is_empty() => {
                let count = pairs.len();
                for (k, v) in pairs {
                    db.set(k, v);
                }
                println!("Restored {count} keys from {}", persistence::DUMP_PATH);
            }
            Err(e) => eprintln!("Warning on startup: {e}"),
            _ => {}
        }
    }

    loop {
        print!("rbtree-db> ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF (Ctrl+D / Ctrl+Z)
            Ok(_) => {}
            Err(e) => {
                eprintln!("ERR: failed to read input: {e}");
                break;
            }
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match parse_command(input) {
            Ok(Command::Exit) => std::process::exit(0),
            Ok(cmd) => println!("{}", execute(&mut db, cmd)),
            Err(e) => println!("ERR: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_set_simple() {
        match parse_command("SET foo bar").unwrap() {
            Command::Set(k, v) => {
                assert_eq!(k, "foo");
                assert_eq!(v, "bar");
            }
            _ => panic!("expected Set"),
        }
    }

    #[test]
    fn parse_set_value_with_spaces() {
        match parse_command("SET nome Gabriel Zanetti").unwrap() {
            Command::Set(k, v) => {
                assert_eq!(k, "nome");
                assert_eq!(v, "Gabriel Zanetti");
            }
            _ => panic!("expected Set"),
        }
    }

    #[test]
    fn parse_set_case_insensitive() {
        assert!(matches!(parse_command("set foo bar"), Ok(Command::Set(_, _))));
    }

    #[test]
    fn parse_get() {
        assert!(matches!(
            parse_command("GET nome"),
            Ok(Command::Get(k)) if k == "nome"
        ));
    }

    #[test]
    fn parse_delete() {
        assert!(matches!(
            parse_command("DELETE key"),
            Ok(Command::Delete(k)) if k == "key"
        ));
    }

    #[test]
    fn parse_range() {
        match parse_command("RANGE ana pedro").unwrap() {
            Command::Range(low, high) => {
                assert_eq!(low, "ana");
                assert_eq!(high, "pedro");
            }
            _ => panic!("expected Range"),
        }
    }

    #[test]
    fn parse_no_arg_commands() {
        assert!(matches!(parse_command("KEYS"), Ok(Command::Keys)));
        assert!(matches!(parse_command("MIN"), Ok(Command::Min)));
        assert!(matches!(parse_command("MAX"), Ok(Command::Max)));
        assert!(matches!(parse_command("SAVE"), Ok(Command::Save)));
        assert!(matches!(parse_command("LOAD"), Ok(Command::Load)));
        assert!(matches!(parse_command("EXIT"), Ok(Command::Exit)));
    }

    #[test]
    fn parse_unknown_command_is_error() {
        let err = parse_command("FLUSHALL").unwrap_err();
        assert!(err.contains("unknown command"));
    }

    #[test]
    fn parse_set_missing_value_is_error() {
        assert!(parse_command("SET key").is_err());
    }

    #[test]
    fn parse_get_missing_key_is_error() {
        assert!(parse_command("GET").is_err());
    }

    #[test]
    fn parse_range_missing_second_key_is_error() {
        assert!(parse_command("RANGE ana").is_err());
    }

    #[test]
    fn parse_empty_input_is_error() {
        assert!(parse_command("").is_err());
    }
}
