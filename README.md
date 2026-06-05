# rbtree-db

Banco de dados de chave-valor com range queries, implementado em Rust sobre uma árvore Rubro-Negra.

O diferencial em relação a uma hashtable (como o Redis usa por padrão) é o suporte a **range queries** em `O(log n + k)` — algo que hashtables não conseguem fazer, mas uma árvore balanceada faz naturalmente via percurso in-order com poda de subárvores.

## Contexto acadêmico

Projeto da disciplina **EDA2 — 2026.1** (Grupo 18). A Rubro-Negra não é detalhe de implementação — ela *habilita* as buscas por intervalo eficientes que justificam a existência do produto.

## Funcionalidades

| Comando | Descrição | Complexidade |
|---|---|---|
| `SET k v` | Insere ou atualiza um par chave-valor | O(log n) |
| `GET k` | Busca exata por chave | O(log n) |
| `DELETE k` | Remove um par | O(log n) |
| `RANGE k1 k2` | Todos os pares com chave entre k1 e k2 | O(log n + k) |
| `KEYS` | Lista todas as chaves em ordem crescente | O(n) |
| `MIN` / `MAX` | Menor e maior chave | O(log n) |
| `SAVE` | Persiste o estado em `data/dump.rdb` | O(n) |
| `LOAD` | Restaura o estado do arquivo | O(n log n) |
| `EXIT` | Encerra o processo | — |

## Por que Rubro-Negra e não hashtable?

O comando `RANGE` é o diferencial central do projeto e a justificativa para escolher uma árvore balanceada em vez de uma hashtable (estrutura padrão do Redis).

| Operação | Hashtable | Rubro-Negra |
|---|---|---|
| `GET` / `SET` / `DELETE` | O(1) amortizado | O(log n) |
| `RANGE k1 k2` | **O(n)** | **O(log n + k)** |
| `KEYS` (em ordem) | O(n log n) | O(n) |
| `MIN` / `MAX` | O(n) | O(log n) |

Em uma hashtable as chaves não têm ordem definida — encontrar todos os pares em um intervalo exige percorrer toda a coleção: **O(n)**. Na Rubro-Negra, o percurso in-order com poda de subárvores garante:

1. Desce pela árvore em **O(log n)** até o primeiro nó ≥ `k1`
2. Percorre em ordem crescente apenas os **k** nós dentro do intervalo
3. Para imediatamente ao encontrar o primeiro nó > `k2`

Custo total: **O(log n + k)** — independente do tamanho total da coleção. Para um banco com 1 milhão de chaves retornando 10 resultados, a Rubro-Negra visita ~20 nós; a hashtable visita 1 milhão.

`MIN` e `MAX` também beneficiam: a menor chave é sempre a folha mais à esquerda, acessível em **O(log n)** sem percorrer toda a estrutura.

## Como executar

**Pré-requisitos:** Rust 1.75+ instalado ([rustup.rs](https://rustup.rs))

```bash
# Clonar o repositório
git clone https://github.com/eda2-2026/G18_Arvore_EDA2-2026.1.git
cd G18_Arvore_EDA2-2026.1

# Compilar e executar
cargo run

# Rodar os testes
cargo test
```

## Exemplo de uso

```
rbtree-db> SET nome Gabriel
OK
rbtree-db> SET cidade Brasilia
OK
rbtree-db> RANGE cidade nome
cidade -> "Brasilia"
nome -> "Gabriel"
rbtree-db> MIN
cidade -> "Brasilia"
rbtree-db> SAVE
Persisted 2 keys to data/dump.rdb
rbtree-db> EXIT
```

## Estrutura do projeto

```
src/
├── main.rs           # Ponto de entrada e loop principal
├── db.rs             # Camada de banco: comandos e índice
├── persistence.rs    # Serialização para arquivo
└── tree/
    ├── mod.rs        # Declaração dos submódulos
    ├── node.rs       # Nó com cor, chave, valor, filhos
    ├── rbtree.rs     # Inserção, remoção, rotações
    └── iterator.rs   # Percurso in-order para range queries
data/
└── dump.rdb          # Arquivo de persistência (gerado pelo SAVE)
```

## Autores

- **Heitor Ricardo** ([@HeitorM50](https://github.com/HeitorM50)) — camada de banco, REPL, persistência
- **Lucas A. Zanetti** ([@Bappoz](https://github.com/Bappoz)) — implementação da árvore Rubro-Negra
