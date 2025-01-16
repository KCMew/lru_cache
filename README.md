# Cache LRU en Rust

Une implémentation robuste et flexible d'un cache LRU (Least Recently Used) en Rust, avec support de persistance.

## Fonctionnalités

- 🚀 Cache LRU en mémoire avec capacité configurable
- 💾 Version persistante
- 🎯 Simulateur d'API pour les tests

# Arborescence du projet

```
lru/
├── benches/
│   ├── bench_cache.rs
├── src/
│   ├── lib.rs
│   ├── cache.rs
│   ├── persistent_cache.rs
│   ├── api_service.rs
├── tests/
│   ├── tests.rs
│   ├── persistent_tests.rs

```

# Lancement des tests

```bash
cargo test
cargo test -- --nocapture
cargo test --test persistent_tests -- --nocapture
```

# Lancement de la documentation

```bash
cargo doc --open
```

# Lancement du benchmarking

```bash
cargo bench
```

