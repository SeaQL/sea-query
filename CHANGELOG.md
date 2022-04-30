# Changelog

All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## 0.25.0 - Pending

### New Features

* CASE WHEN statement support https://github.com/SeaQL/sea-query/pull/304
* Add support for Ip(4,6)Network and MacAddress https://github.com/SeaQL/sea-query/pull/309
* [sea-query-attr] macro for deriving `Iden` enum from struct https://github.com/SeaQL/sea-query/pull/300
* Add ability to alter foreign keys https://github.com/SeaQL/sea-query/pull/299
* Select `DISTINCT ON` https://github.com/SeaQL/sea-query/pull/313

### Enhancements

* Insert Default https://github.com/SeaQL/sea-query/pull/266

## 0.24.4 - 2022-04-26

* update sea-query-driver

## 0.24.3 - 2022-04-26

### Bug fixes

* Fix MySQL index create statement https://github.com/SeaQL/sea-query/pull/308

### Enhancements

* Add length check on condition array https://github.com/SeaQL/sea-query/pull/307

## 0.24.2 - 2022-04-18

### Bug fixes

* Fixed https://github.com/SeaQL/sea-query/issues/303 driver breakage in 0.24.0

Notes: 0.24.0 & 0.24.1 were yanked

## 0.24.1 - 2022-04-15

### Enhancements

* #295 Add parameter for SQLx path to proc-macro https://github.com/SeaQL/sea-query/pull/297

### Bug fixes

* CTE optional columns https://github.com/SeaQL/sea-query/pull/301

## 0.24.0 - 2022-04-05

### New Features

* Add `LOWER` and `UPPER` func https://github.com/SeaQL/sea-query/pull/276
* Insert `ON CONFLICT` support https://github.com/SeaQL/sea-query/pull/279
* #174 Add support for `WINDOWS` statement https://github.com/SeaQL/sea-query/pull/271
* #142 full support lock in select https://github.com/SeaQL/sea-query/pull/289
* #269 add support for postgres `ANY`, `SOME`, `ALL` https://github.com/SeaQL/sea-query/pull/283

### Enhancements

* Add support for multiple `ALTER` operations https://github.com/SeaQL/sea-query/pull/277
* #229 add column if not exists https://github.com/SeaQL/sea-query/pull/278
* #255 Add support to CommonTableExpression columns method https://github.com/SeaQL/sea-query/pull/284
* #280 Rewrite drivers using proc-macro https://github.com/SeaQL/sea-query/pull/292

### Bug fixes

* #285 Fix timestamp_with_time_zone_len https://github.com/SeaQL/sea-query/pull/286

### Breaking changes

* The enum variants for `LockType` were renamed: `Exclusive` -> `Update` and `Shared` -> `Share`
* As part of #283, the drivers are split to the `sea-query-driver` crate
    1. Remove methods `Value::is_json` and `Value::as_ref_json` when feature: **with-json** is disabled
    2. Remove methods `Value::is_time_*` and `Value::as_ref_time_*` when feature: **with-time** is disabled
    3. Remove methods `Value::is_chrono_*` and `Value::as_ref_chrono*` when feature: **with-chrono** is disabled
    4. Remove methods `Value::is_decimal`, `Value::as_ref_decimal` and `Value::decimal_to_f64` when feature: **with-rust_decimal** is disabled
    5. Remove methods `Value::is_big_decimal`, `Value::as_ref_big_decimal` and `Value::big_decimal_to_f64` when feature: **with-bigdecimal** is disabled
    6. Remove methods `Value::is_uuid` and `Value::as_ref_uuid` when feature: **with-uuid** is disabled
    7. Remove methods `Value::is_array` and `Value::as_ref_array` when feature: **postgres-array** is disabled

## 0.23.0 - 2022-03-15

### New Features

* Supports `time` in addition to `chrono` https://github.com/SeaQL/sea-query/pull/267

### Enhancements

* Allow for trailing commas in any and all macros https://github.com/SeaQL/sea-query/pull/270

### Bug fixes

* Fix UNIQUE table index expression syntax for sqlite https://github.com/SeaQL/sea-query/pull/227

### Breaking changes

In order to co-exist with the `time` crate, `Date`, `Time`, `DateTime` etc are renamed to `ChronoDate`, `ChronoTime`, `ChronoDateTime`. In addition, new variants `TimeDate`, `TimeTime`, `TimeDateTime` and so on are introduced to `Value`.

## 0.22.0 - 2022-02-26

### New Features

* Support multiple tables in the select from by @Sytten in https://github.com/SeaQL/sea-query/pull/261
* Add support for replace insert by @Sytten in https://github.com/SeaQL/sea-query/pull/262
* Add `ColumnType` unsigned integer types by @billy1624 in https://github.com/SeaQL/sea-query/pull/211

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.21.0...0.22.0

## 0.21.0 - 2022-02-01

### Breaking Changes

* Use double quotes for quoting identifiers for SQLite by @SpyrosRoum in https://github.com/SeaQL/sea-query/pull/221

### New Features

* Implement `RETURNING` for SQLite by @SpyrosRoum in https://github.com/SeaQL/sea-query/pull/194
* Support 'NULLS LAST' and 'NULLS FIRST' by @qyihua in https://github.com/SeaQL/sea-query/pull/210
* [join-lateral]  by @rex-remind101 in https://github.com/SeaQL/sea-query/pull/224
* Insert from select by @05storm26 in https://github.com/SeaQL/sea-query/pull/238
* Add Expr::asterisk() and Expr::tbl_asterisk(table: DynIden) methods - Fix #217 by @RomainMazB in https://github.com/SeaQL/sea-query/pull/219

### Enhancements

* Implement ToTokens for IntervalField by @autarch in https://github.com/SeaQL/sea-query/pull/195
* Implemented 'Array' type for Postgres. by @kev0960 in https://github.com/SeaQL/sea-query/pull/205
* Add `Value::DateTimeLocal` by @billy1624 in https://github.com/SeaQL/sea-query/pull/249
* Add `ColumnRef::SchemaTableColumn` by @billy1624 in https://github.com/SeaQL/sea-query/pull/206
* Datetime utc by @tyt2y3 in https://github.com/SeaQL/sea-query/pull/241
* Support the use of chrono::DateTime<Utc> using the type alias DateTim… by @charleschege in https://github.com/SeaQL/sea-query/pull/222

### Bug Fixes

* Fix PostgreSQL `ColumnType::TinyInteger` mapping by @billy1624 in https://github.com/SeaQL/sea-query/pull/207
* PR without clippy warmings in file changed tab by @billy1624 in https://github.com/SeaQL/sea-query/pull/212

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.20.0...0.21.0

## 0.20.0 - 2021-12-11

### Merged PRs

* Add `TableRef::DatabaseSchemaTable` by @billy1624 in https://github.com/SeaQL/sea-query/pull/193

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.19.4...0.20.0

## 0.19.4 - 2021-12-11

### Merged PRs

* Binding `DateTime<FixedOffset>` for SQLx MySQL & SQLite by @billy1624 in https://github.com/SeaQL/sea-query/pull/197

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.19.2...0.19.4

## 0.19.2 - 2021-12-04

### Merged PRs

* Impl `ValueTuple` Up to Six by @billy1624 in https://github.com/SeaQL/sea-query/pull/200
* Basic Benchmark by @tyt2y3 in https://github.com/SeaQL/sea-query/pull/192

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.19.1...0.19.2

## 0.19.1 - 2021-11-25

### Merged PRs
* `driver/postgres` handle non-exhaustive `Value` by @billy1624 in https://github.com/SeaQL/sea-query/pull/191

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.19.0...0.19.1

## 0.19.0 - 2021-11-19

### Merged PRs
* `TableCreateStatement` and `TableDropStatement` takes any `IntoTableRef` table name. by @josh-codes in https://github.com/SeaQL/sea-query/pull/186
* Add `ColumnType::Enum` by @billy1624 in https://github.com/SeaQL/sea-query/pull/188
* Update to Rust Edition 2021 by @billy1624 in https://github.com/SeaQL/sea-query/pull/189

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.18.2...0.19.0

## 0.18.2 - 2021-11-04

### Merged PRs
* Rename "where" keywords in `SelectStatement` to suppress IDEA warnings by @baoyachi in https://github.com/SeaQL/sea-query/pull/166
* Add binary method to expr by @Progdrasil in https://github.com/SeaQL/sea-query/pull/173
* Cast expression as custom type by @billy1624 in https://github.com/SeaQL/sea-query/pull/170
* Support tuple expression by @shuoli84 in https://github.com/SeaQL/sea-query/pull/178

**Full Changelog**: https://github.com/SeaQL/sea-query/compare/0.18.1...0.18.2

## 0.18.1 - 2021-10-26

+ [[#169]] Add support for Postgres interval type
+ [[#171]] Fix bug in `Condition::add` where Condition negation is ignored

[#169]: https://github.com/SeaQL/sea-query/pull/169
[#171]: https://github.com/SeaQL/sea-query/pull/171

## 0.18.0 - 2021-10-15

+ [[#159]] Add `ValueType::column_type`
+ [[#160]] Add `FromValueTuple` trait

[#159]: https://github.com/SeaQL/sea-query/pull/159
[#160]: https://github.com/SeaQL/sea-query/pull/160

## 0.17.3 - 2021-10-26

+ [[#171]] Fix bug in `Condition::add` where Condition negation is ignored

[#171]: https://github.com/SeaQL/sea-query/pull/171

## 0.17.2 - 2021-10-15

+ [[#164]] Revert "Fix SQLite `chrono::NaiveTime` binding"

[#164]: https://github.com/SeaQL/sea-query/pull/164

## 0.17.1 - 2021-10-12 (yanked)

## 0.17.0 - 2021-10-06

+ [[#157]] Fix binding nullable custom types on db drivers

The `as_ref_*` methods on `Value` are changed:

```rust
pub fn as_ref_json(&self) -> &Json;
```

Is now

```rust
pub fn as_ref_json(&self) -> Option<&Json>;
```

[#157]: https://github.com/SeaQL/sea-query/pull/157

## 0.16.6 - 2021-10-26

+ [[#171]] Fix bug in `Condition::add` where Condition negation is ignored

[#171]: https://github.com/SeaQL/sea-query/pull/171

## 0.16.5 - 2021-09-30

+ [[#145]] Add Condition::not
+ [[#149]] Fix SQLite `chrono::NaiveTime` binding

[#145]: https://github.com/SeaQL/sea-query/pull/145
[#149]: https://github.com/SeaQL/sea-query/pull/149

## 0.16.4 - 2021-09-26

+ Fix table drop options for SQLite
+ Add `IndexCreateStatement::is_unique_key()`

## 0.16.3 - 2021-09-17

+ [[#131]] `CAST AS` expression
+ [[#131]] `InsertStatement` accepts `SimpleExpr`
+ [[#137]] SQLx Postgres driver bind `DateTime<FixedOffset>`

[#131]: https://github.com/SeaQL/sea-query/issues/131
[#137]: https://github.com/SeaQL/sea-query/pull/137

## 0.16.2 - 2021-09-15

+ [[#120]] Support `RETURNING` for `DeleteStatement`
+ [[#128]] Support `UNION` clause for `SelectStatement`

[#120]: https://github.com/SeaQL/sea-query/issues/120
[#128]: https://github.com/SeaQL/sea-query/pull/128

## 0.16.1 - 2021-09-10

+ [[#129]] MySql `ColumnType::Binary(None)` maps to "blob"

[#129]: https://github.com/SeaQL/sea-query/pull/129

## 0.16.0 - 2021-09-02

+ [[#112]] Introduce `Nullable` trait to permit custom `Option<T>`
+ [[#113]] `ValueType` trait should have a non-panic-ing method
+ [[#114]] `ValueType` revamp

    1. Remove `ValueTypeDefault`
    1. Change `type_name` to return `String`

+ [[#115]] Postgres concatenate operator (`||`)
+ [[#117]] Lock support (`FOR SHARE`, `FOR UPDATE`) for SELECT statement

[#112]: https://github.com/SeaQL/sea-query/pull/112
[#113]: https://github.com/SeaQL/sea-query/pull/113
[#114]: https://github.com/SeaQL/sea-query/pull/114
[#115]: https://github.com/SeaQL/sea-query/pull/115
[#117]: https://github.com/SeaQL/sea-query/pull/117

## 0.15.0 - 2021-08-21

+ [[#107]] Revamp `Value` to typed null value
+ Added `BigDecimal` support

The `Value::Null` variant is removed. You have to use a specific variant with a `None`.

Before:

```rust
Query::insert()
    .values_panic(vec![
        Value::Null,
        2.1345.into(),
    ])
```

After:

```rust
Query::insert()
    .values_panic(vec![
        Value::String(None),
        2.1345.into(),
    ])
```

Since we cannot handle the generic `Null` value on JSON, we removed the `json` method on `InsertStatement` and `UpdateStatement`. The following NO LONGER WORKS:

```rust
let query = Query::insert()
    .into_table(Glyph::Table)
    .json(json!({
        "aspect": 2.1345,
        "image": "24B",
    }));
```

```rust
let query = Query::update()
    .table(Glyph::Table)
    .json(json!({
        "aspect": 2.1345,
        "image": "235m",
    }));
```

In addition, if you constructed `Value` manually before (instead of using `into()` which is unaffected), you have to wrap them in an `Option`:

Before:

```rust
let (sql, values) = query.build(PostgresQueryBuilder);
assert_eq!(
	values,
	Values(vec![Value::String(Box::new("A".to_owned())), Value::Int(1), Value::Int(2), Value::Int(3)]))
);
```

After:

```rust
let (sql, values) = query.build(PostgresQueryBuilder);
assert_eq!(
	values,
	Values(vec![Value::String(Some(Box::new("A".to_owned()))), Value::Int(Some(1)), Value::Int(Some(2)), Value::Int(Some(3))]))
);
```

[#107]: https://github.com/SeaQL/sea-query/pull/107

## 0.14.1 - 2021-08-15

+ [[#87]] Fix inconsistent Ownership of self in Builder APIs
+ [[#105]] Use Arc for SeaRc with feature flag thread-safe

[#87]: https://github.com/SeaQL/sea-query/pull/87
[#105]: https://github.com/SeaQL/sea-query/pull/105

## 0.12.12 - 2021-08-14

+ [[#98]] Support Postgres full text search

[#98]: https://github.com/SeaQL/sea-query/pull/98

## 0.12.11 - 2021-08-13

+ Support SeaORM

## 0.12.10 - 2021-08-11

+ [[#89]] flattening iden enums in derive macro

[#89]: https://github.com/SeaQL/sea-query/pull/87

## 0.12.9 - 2021-08-08

+ [[#77]] Postgres `binary` type
+ [[#81]] example for CockroachDB
+ [[#84]] Fix Postgres constraint keywords
+ [[#75]] `DateTimeWithTimeZone` value type and `TimestampWithTimeZone` column type

[#77]: https://github.com/SeaQL/sea-query/pull/77
[#81]: https://github.com/SeaQL/sea-query/pull/81
[#84]: https://github.com/SeaQL/sea-query/pull/84
[#75]: https://github.com/SeaQL/sea-query/pull/75

## 0.12.8 - 2021-07-24

+ Fix Postgres `datetime` column type mapping
+ `Uuid` in schema builder

## 0.12.7 - 2021-07-13

+ `cust_with_values` allow escape `?` using `??`

## 0.12.6 - 2021-07-07

+ Fixed build error for `sqlx-sqlite`

## 0.12.5 - 2021-07-07

+ Support `Decimal` from rust_decimal

## 0.12.4 - 2021-06-23

+ Added `returning` for update statement

## 0.12.3 - 2021-06-19

+ Added `type_name` for ValueType
+ `Values` derive `Clone`
+ Added `Update::col_expr`
+ Type def Rc as `SeaRc`
+ getters for schema statements

## 0.12.2 - 2021-06-04

+ Fixed `and_where_option`
+ Added `Condition::add_option`

## 0.12.1 - 2021-06-03

+ Added `not_in_subquery`

## 0.12.0 - 2021-05-31

+ Unify `cond_where` and `and_where`. Note: will panic if calling `or_where` after `and_where`.

## 0.11.1 - 2021-05-22

+ Updated Readme

## 0.11.0 - 2021-05-19

+ Added APIs to support ORM
+ Backend and internal refactoring
+ Introduced `QueryStatementBuilder` and `SchemaStatementBuilder` traits
+ Introduced `ConditionalStatement` and `OrderedStatement` traits
+ Introduced any/all style conditions for `cond_where` and `cond_having`

## 0.10.6 - 2021-05-04

+ Postgres `ALTER TYPE` statements for `ENUM`

## 0.10.5 - 2021-05-02

+ Updated documentation

## 0.10.4 - 2021-05-02

+ `returning()` expression for Postgres insert statements
+ Remove redundant index name in foreign key expression of MySQL

## 0.10.3 - 2021-04-30

+ custom `Error` type
+ Empty value list for IN
+ Index prefix and `IndexOrder`

## 0.10.2 - 2021-04-27

+ Foreign key API `from` and `to`
+ Fix foreign key bug in `ON UPDATE`

## 0.10.1 - 2021-04-25

+ Added `index_type()` (`FullText` and `Hash`)
+ Added `unique()` to `Index`
+ Support composite primary key

## 0.10.0 - 2021-04-23

+ Use `IntoIterator` trait instead of `Vec` on most APIs
+ UUID support in `Value`
+ Rusqlite support

## 0.9.6 - 2021-04-18

+ Rename `create_if_not_exists` to `if_not_exists`
+ Remove `partition_option` from `TableAlterStatement`
+ Added `ColumnDef::extra()`

## 0.9.5 - 2021-04-17

+ Added `SchemaStatement`

## 0.9.4 - 2021-04-13

+ Fixed `DateTime` quoting bugs

## 0.9.3 - 2021-04-08

+ Update sea-query-derive to 0.1.2

## 0.9.2 - 2021-04-05

+ derive supporting enum tuple variant and custom method

## 0.9.1 - 2021-03-30

+ updated docs

## 0.9.0 - 2021-03-29

+ Introduced `IntoColumnRef` trait to consolidate `column` and `table.column`
+ Introduced `IntoTableRef` trait to consolidate `table` and `schema.table`
+ Introduced `IntoIden` trait to remove `*_dyn` methods

## 0.8.5 - 2021-03-29

+ added `into_simple_expr()`

## 0.8.4 - 2021-03-24

+ Fixing `IS NULL`

## 0.8.3 - 2021-03-23

+ derive `Debug` on most structs

## 0.8.2 - 2021-03-23

+ Added `unescape_string`

## 0.8.1 - 2021-03-23

+ Improve documentation

## 0.8.0 - 2021-03-14

+ `json` support behind features
+ backend as features (`backend-mysql`, `backend-postgres`, `backend-sqlite`)
+ added `from_schema()`, `from_schema_as()`

## 0.7.0 - 2021-03-06

+ Revamp `Value`
+ `build()` API change
+ `postgres` driver support
+ `json` and `chrono` support

## 0.6.1 - 2021-03-05

+ Added `join_as`
+ Deprecated `expr_alias`, `from_alias`

## 0.6.0 - 2021-02-20

+ Custom expression with parameters `Expr::cust_with_values()`
+ Custom function call `Func::cust()`
+ Postgres enum `Type::create().as_enum()`

## 0.5.0 - 2021-02-09

+ derive macro `#[derive(Iden)]`

## 0.4.0 - 2021-02-02

+ Added JSON binary column type `ColumnDef::json_binary()`
+ Custom column type `ColumnDef::custom()`

## 0.3.0 - 2020-12-29



## 0.2.0 - 2020-12-26



## 0.1.0 - 2020-12-16

Publish to crate.io
