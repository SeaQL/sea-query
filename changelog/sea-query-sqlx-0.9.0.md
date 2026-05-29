## Release Notes: sea-query-sqlx 0.9.0

*(since sea-query-sqlx 0.8.1)*

### Highlights

* Promoted `sea-query-sqlx` to the SQLx `0.9` release line.
* Kept `sea-query-sqlx 0.8.1` as the compatibility release for users staying on SQLx `0.8`.
* Raised the `sea-query-sqlx` Rust version requirement to `1.94.0`, matching SQLx `0.9`.
* Kept Jiff and pgvector value support available in SeaQuery, while temporarily disabling their SQLx binder integrations until their upstream crates support SQLx `0.9`.

### Breaking Changes

* Upgraded SQLx from `0.8` to `0.9`.

This is a breaking dependency upgrade for users of `sea-query-sqlx`. Projects that still depend on SQLx `0.8` should use:

```toml
sea-query-sqlx = "0.8.1"
```

Projects that are ready for SQLx `0.9` should use:

```toml
sea-query-sqlx = "0.9"
```

* Raised the minimum supported Rust version for `sea-query-sqlx` to `1.94.0`.

The main `sea-query` crate remains on its own Rust version requirement. This Rust version bump applies to the SQLx binder crate because SQLx `0.9` requires Rust `1.94.0`.

### Known Limitations

* `with-jiff` no longer enables `jiff-sqlx`.

`jiff-sqlx 0.1.1` still depends on SQLx `0.8`, so enabling it together with SQLx `0.9` would pull two incompatible SQLx versions into the dependency graph. `sea-query-sqlx 0.9.0` keeps SeaQuery's Jiff value support enabled, but SQLx binding support for Jiff values is temporarily unavailable.

Jiff binding support should be restored after `jiff-sqlx` supports SQLx `0.9`.

* `postgres-vector` no longer enables `pgvector/sqlx`.

`pgvector 0.4.2` still implements SQLx `0.8` traits, so enabling `pgvector/sqlx` with SQLx `0.9` causes trait mismatch errors. `sea-query-sqlx 0.9.0` keeps the `postgres-vector` feature compiling without pulling in `pgvector/sqlx`, but binding `Value::Vector` through the SQLx Postgres binder will panic until `pgvector` supports SQLx `0.9`.

### Migration Notes

Use `sea-query-sqlx 0.8.1` if your project, framework, or downstream dependency tree is still pinned to SQLx `0.8`.

Use `sea-query-sqlx 0.9.0` if your project is ready for SQLx `0.9` and Rust `1.94.0`.

SeaORM users should avoid accidentally resolving `sea-query-sqlx 0.9.0` while SeaORM is still pinned to SQLx `0.8`.

### Verification

Verified with:

```sh
cargo +1.94 check --manifest-path sea-query-sqlx/Cargo.toml --all-features
```
