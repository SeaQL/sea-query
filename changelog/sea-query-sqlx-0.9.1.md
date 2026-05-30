## Release Notes: sea-query-sqlx 0.9.1

*(since sea-query-sqlx 0.9.0)*

### Fixes

* Restored SQLx binder support for PostgreSQL `pgvector` values.

`sea-query-sqlx 0.9.0` accidentally disabled the `pgvector/sqlx` integration and made `Value::Vector` binding panic in the PostgreSQL SQLx binder.

`pgvector 0.4.2` supports SQLx `0.9`, so `postgres-vector` now enables `pgvector/sqlx` again and `Value::Vector` is passed to SQLx as a bind argument.

### Migration Notes

* Use `sea-query-sqlx 0.9.1` instead of `0.9.0` if your application binds `Value::Vector` through SQLx.
* No generated SQL changes are expected.
* The temporary Jiff SQLx binder limitation remains unchanged.
