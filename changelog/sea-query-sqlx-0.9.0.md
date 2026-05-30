## Release Notes: sea-query-sqlx 0.9.0

*(since sea-query-sqlx 0.8.1)*

### Highlights

* Promoted `sea-query-sqlx` to the SQLx `0.9` release line.
* Kept `sea-query-sqlx 0.8.1` as the compatibility release for users staying on SQLx `0.8`.
* Raised the `sea-query-sqlx` Rust version requirement to `1.94.0`, matching SQLx `0.9`.
* Kept Jiff value support available in SeaQuery, while temporarily disabling SQLx binder integration until upstream `jiff-sqlx` supports SQLx `0.9`.
* Known issue: `postgres-vector` compiled, but `Value::Vector` binding through the SQLx Postgres binder panicked in this release. This was corrected in `sea-query-sqlx 0.9.1`.

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

This was an accidental regression in `sea-query-sqlx 0.9.0`. `pgvector 0.4.2` supports SQLx `0.9`, and `sea-query-sqlx 0.9.1` restores SQLx binding support for `Value::Vector`.

### Migration Notes

Use `sea-query-sqlx 0.8.1` if your project, framework, or downstream dependency tree is still pinned to SQLx `0.8`.

Use `sea-query-sqlx 0.9.0` if your project is ready for SQLx `0.9` and Rust `1.94.0`.

SeaORM users should avoid accidentally resolving `sea-query-sqlx 0.9.0` while SeaORM is still pinned to SQLx `0.8`.

### Verification

Verified with:

```sh
cargo +1.94 check --manifest-path sea-query-sqlx/Cargo.toml --all-features
```
