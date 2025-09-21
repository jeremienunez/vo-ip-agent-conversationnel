# Scripts & Automatisation

Ce dossier contient les scripts d'automatisation :

- `bootstrap.sh` : installe la toolchain Rust (version définie dans `rust-toolchain.toml`) ainsi que `rustfmt`, `clippy`, `miri` et `cargo-tarpaulin`.
- `ci/` : scripts invoqués par la CI (lint, tests, build, scan) – à créer.
- `ops/` : tâches opérationnelles (backups, rotation logs, collecte métriques) – à compléter.

Privilégier des scripts idempotents et documentés (usage, paramètres). Ajouter des tests lorsque pertinent.
