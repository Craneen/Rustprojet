# Slab Allocator en Rust (`no_std`)

## Objectif
Implémenter un Slab Allocator simple en Rust sans la bibliothèque standard (`no_std`).

## Structure du Projet
- `src/allocator.rs` : Code de l’allocateur.  
- `src/lib.rs` : Configuration en `no_std`.  
- `tests/allocator_test.rs` : Tests unitaires.  
- `REPORT.md` : Rapport détaillant les choix techniques.  
- `REPORT_TEST.md` : Résultats détaillés des tests.

## Compilation

```bash
cargo build

