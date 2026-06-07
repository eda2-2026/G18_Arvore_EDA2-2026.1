//! Benchmarks comparativos: RBTree vs BTreeMap vs HashMap
//!
//! Execute com:
//!   cargo bench
//!   cargo bench -- --save-baseline baseline   # salva linha de base
//!   cargo bench -- insert                     # filtra por nome
//!
//! Relatório HTML: target/criterion/report/index.html

use std::collections::{BTreeMap, HashMap};
use std::hint::black_box;

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use rbtree_db::tree::rbtree::RBTree;

// ── gerador determinístico (LCG) ─────────────────────────────────────────────

fn lcg_seq(n: usize) -> Vec<i32> {
    let mut v = Vec::with_capacity(n);
    let mut x: u64 = 0xdeadbeef_cafebabe;
    for _ in 0..n {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        v.push((x >> 33) as i32);
    }
    v
}

// ── INSERT ────────────────────────────────────────────────────────────────────

fn bench_insert(c: &mut Criterion) {
    let mut g = c.benchmark_group("insert_random");

    for n in [1_000, 10_000, 100_000] {
        let keys = lcg_seq(n);

        g.bench_with_input(BenchmarkId::new("RBTree", n), &keys, |b, ks| {
            b.iter(|| {
                let mut t = RBTree::new();
                for &k in ks {
                    t.insert(black_box(k), black_box(k));
                }
                t
            });
        });

        g.bench_with_input(BenchmarkId::new("BTreeMap", n), &keys, |b, ks| {
            b.iter(|| {
                let mut m: BTreeMap<i32, i32> = BTreeMap::new();
                for &k in ks {
                    m.insert(black_box(k), black_box(k));
                }
                m
            });
        });

        g.bench_with_input(BenchmarkId::new("HashMap", n), &keys, |b, ks| {
            b.iter(|| {
                let mut m: HashMap<i32, i32> = HashMap::new();
                for &k in ks {
                    m.insert(black_box(k), black_box(k));
                }
                m
            });
        });
    }

    g.finish();
}

// ── GET (hit e miss) ──────────────────────────────────────────────────────────

fn bench_get(c: &mut Criterion) {
    let n = 10_000;
    let keys = lcg_seq(n);
    let misses = lcg_seq(n).into_iter().map(|k| k.wrapping_add(1)).collect::<Vec<_>>();

    let mut rbt = RBTree::new();
    let mut btm: BTreeMap<i32, i32> = BTreeMap::new();
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for &k in &keys {
        rbt.insert(k, k);
        btm.insert(k, k);
        hm.insert(k, k);
    }

    let mut g = c.benchmark_group("get_hit");
    g.bench_function("RBTree", |b| {
        b.iter(|| keys.iter().map(|k| black_box(rbt.get(k))).count())
    });
    g.bench_function("BTreeMap", |b| {
        b.iter(|| keys.iter().map(|k| black_box(btm.get(k))).count())
    });
    g.bench_function("HashMap", |b| {
        b.iter(|| keys.iter().map(|k| black_box(hm.get(k))).count())
    });
    g.finish();

    let mut g = c.benchmark_group("get_miss");
    g.bench_function("RBTree", |b| {
        b.iter(|| misses.iter().map(|k| black_box(rbt.get(k))).count())
    });
    g.bench_function("BTreeMap", |b| {
        b.iter(|| misses.iter().map(|k| black_box(btm.get(k))).count())
    });
    g.bench_function("HashMap", |b| {
        b.iter(|| misses.iter().map(|k| black_box(hm.get(k))).count())
    });
    g.finish();
}

// ── DELETE ────────────────────────────────────────────────────────────────────

fn bench_delete(c: &mut Criterion) {
    let n = 10_000;
    let keys = lcg_seq(n);

    let mut g = c.benchmark_group("delete");

    g.bench_function("RBTree", |b| {
        b.iter_batched(
            || {
                let mut t = RBTree::new();
                for &k in &keys {
                    t.insert(k, k);
                }
                (t, keys.clone())
            },
            |(mut t, ks)| {
                for k in ks {
                    black_box(t.delete(&k));
                }
            },
            BatchSize::SmallInput,
        );
    });

    g.bench_function("BTreeMap", |b| {
        b.iter_batched(
            || {
                let mut m: BTreeMap<i32, i32> = BTreeMap::new();
                for &k in &keys {
                    m.insert(k, k);
                }
                (m, keys.clone())
            },
            |(mut m, ks)| {
                for k in ks {
                    black_box(m.remove(&k));
                }
            },
            BatchSize::SmallInput,
        );
    });

    g.bench_function("HashMap", |b| {
        b.iter_batched(
            || {
                let mut m: HashMap<i32, i32> = HashMap::new();
                for &k in &keys {
                    m.insert(k, k);
                }
                (m, keys.clone())
            },
            |(mut m, ks)| {
                for k in ks {
                    black_box(m.remove(&k));
                }
            },
            BatchSize::SmallInput,
        );
    });

    g.finish();
}

// ── RANGE QUERY ───────────────────────────────────────────────────────────────
//
// Vantagem da árvore: O(log n + k) com poda de subárvores.
// HashMap não tem range nativo — simula varredura completa O(n).

fn bench_range(c: &mut Criterion) {
    let n = 100_000i32;
    // chaves densas 0..n para controlar o range facilmente
    let keys: Vec<i32> = (0..n).collect();

    let mut rbt = RBTree::new();
    let mut btm: BTreeMap<i32, i32> = BTreeMap::new();
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for &k in &keys {
        rbt.insert(k, k);
        btm.insert(k, k);
        hm.insert(k, k);
    }

    // k ≈ 1 % de n (1 000 resultados)
    let lo = n / 2;
    let hi = lo + (n / 100);

    let mut g = c.benchmark_group("range_1pct");

    g.bench_function("RBTree", |b| {
        b.iter(|| rbt.range(&black_box(lo), &black_box(hi)).count())
    });

    g.bench_function("BTreeMap", |b| {
        b.iter(|| {
            btm.range(black_box(lo)..=black_box(hi)).count()
        })
    });

    // HashMap: sem range nativo — filtra toda a coleção
    g.bench_function("HashMap_full_scan", |b| {
        b.iter(|| {
            hm.iter()
                .filter(|&(&k, _)| k >= black_box(lo) && k <= black_box(hi))
                .count()
        })
    });

    g.finish();
}

// ── MIN / MAX ─────────────────────────────────────────────────────────────────
//
// RBTree e BTreeMap: O(log n) / O(log n).
// HashMap: O(n) — precisa varrer tudo.

fn bench_min_max(c: &mut Criterion) {
    let n = 100_000;
    let keys = lcg_seq(n);

    let mut rbt = RBTree::new();
    let mut btm: BTreeMap<i32, i32> = BTreeMap::new();
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for &k in &keys {
        rbt.insert(k, k);
        btm.insert(k, k);
        hm.insert(k, k);
    }

    let mut g = c.benchmark_group("min");
    g.bench_function("RBTree", |b| b.iter(|| black_box(rbt.min())));
    g.bench_function("BTreeMap", |b| b.iter(|| black_box(btm.iter().next())));
    g.bench_function("HashMap_full_scan", |b| {
        b.iter(|| black_box(hm.iter().min_by_key(|&(&k, _)| k)))
    });
    g.finish();

    let mut g = c.benchmark_group("max");
    g.bench_function("RBTree", |b| b.iter(|| black_box(rbt.max())));
    g.bench_function("BTreeMap", |b| b.iter(|| black_box(btm.iter().next_back())));
    g.bench_function("HashMap_full_scan", |b| {
        b.iter(|| black_box(hm.iter().max_by_key(|&(&k, _)| k)))
    });
    g.finish();
}

// ── ITER ORDENADO ─────────────────────────────────────────────────────────────
//
// RBTree e BTreeMap entregam ordem garantida.
// HashMap precisa coletar + ordenar: O(n log n) extra.

fn bench_iter(c: &mut Criterion) {
    let n = 100_000;
    let keys = lcg_seq(n);

    let mut rbt = RBTree::new();
    let mut btm: BTreeMap<i32, i32> = BTreeMap::new();
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for &k in &keys {
        rbt.insert(k, k);
        btm.insert(k, k);
        hm.insert(k, k);
    }

    let mut g = c.benchmark_group("iter_sorted");
    g.bench_function("RBTree", |b| {
        b.iter(|| rbt.iter().map(|(k, _)| black_box(k)).count())
    });
    g.bench_function("BTreeMap", |b| {
        b.iter(|| btm.iter().map(|(k, _)| black_box(k)).count())
    });
    // HashMap: sem ordem — simula o custo de coletar e ordenar
    g.bench_function("HashMap_collect_sort", |b| {
        b.iter(|| {
            let mut v: Vec<i32> = hm.keys().copied().collect();
            v.sort_unstable();
            black_box(v.len())
        })
    });
    g.finish();
}

// ── registro ──────────────────────────────────────────────────────────────────

criterion_group!(
    benches,
    bench_insert,
    bench_get,
    bench_delete,
    bench_range,
    bench_min_max,
    bench_iter,
);
criterion_main!(benches);
