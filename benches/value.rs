use core::fmt;
use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use sea_query::*;

#[derive(Debug, Iden)]
pub enum Char {
    Table,
    Id,
    Character,
}

fn small_raw(value: i32) -> Result<String, fmt::Error> {
    let mut str = String::new();
    str.write_str("SELECT ")?;
    str.write_str(&Char::Id.quoted())?;
    str.write_str(" FROM ")?;
    str.write_str(&Char::Table.quoted())?;
    str.write_str(" WHERE ")?;
    for _ in black_box(0..9) {
        str.write_str(&Char::Id.quoted())?;
        str.write_str(" = ")?;
        write!(str, "{value}")?;
        str.write_str(" AND ")?;
    }
    str.write_str("1=1")?;

    Ok(str)
}

fn small_select(value: i32) -> SelectStatement {
    Query::select()
        .column(Char::Character)
        .from(Char::Table)
        .and_where(Expr::col(Char::Character).eq(value))
        .and_where(Expr::col(Char::Character).eq(value))
        .and_where(Expr::col(Char::Character).eq(value))
        .and_where(Expr::col(Char::Character).eq(value))
        .and_where(Expr::col(Char::Character).eq(value))
        .and_where(Expr::col(Char::Character).eq(value))
        .and_where(Expr::col(Char::Character).eq(value))
        .and_where(Expr::col(Char::Character).eq(value))
        .and_where(Expr::col(Char::Character).eq(value))
        .and_where(Expr::col(Char::Character).eq(value))
        .to_owned()
}

fn large_raw(value: &jiff::Zoned) -> Result<String, fmt::Error> {
    let mut str = String::new();
    str.write_str("SELECT ")?;
    str.write_str(&Char::Character.quoted())?;
    str.write_str(" FROM ")?;
    str.write_str(&Char::Table.quoted())?;
    str.write_str(" WHERE ")?;

    for _ in 0..9 {
        str.write_str(&Char::Character.quoted())?;
        str.write_str(" = '")?;
        write!(str, "{value}")?;
        str.write_str("' AND ")?;
    }

    str.write_str("1=1")?;

    Ok(str)
}

fn large_select(value: jiff::Zoned) -> SelectStatement {
    Query::select()
        .column(Char::Character)
        .from(Char::Table)
        .and_where(Expr::col(Char::Character).eq(value.clone()))
        .and_where(Expr::col(Char::Character).eq(value.clone()))
        .and_where(Expr::col(Char::Character).eq(value.clone()))
        .and_where(Expr::col(Char::Character).eq(value.clone()))
        .and_where(Expr::col(Char::Character).eq(value.clone()))
        .and_where(Expr::col(Char::Character).eq(value.clone()))
        .and_where(Expr::col(Char::Character).eq(value.clone()))
        .and_where(Expr::col(Char::Character).eq(value.clone()))
        .and_where(Expr::col(Char::Character).eq(value.clone()))
        .and_where(Expr::col(Char::Character).eq(value))
        .to_owned()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("value");
    let value = black_box(jiff::Zoned::now());

    group.bench_function("select_construction/small", |b| {
        b.iter(|| small_select(black_box(123)))
    });
    group.bench_function("select_construction/small/raw", |b| {
        b.iter(|| small_raw(black_box(123)).unwrap())
    });

    group.bench_function("select_construction/large", |b| {
        b.iter(|| large_select(value.clone()))
    });

    group.bench_function("select_construction/large/raw", |b| {
        b.iter(|| large_raw(&value).unwrap())
    });

    let select_small = black_box(small_select(black_box(123)));
    group.bench_function("select_and_build/small/mysql", |b| {
        b.iter(|| select_small.build(MysqlQueryBuilder))
    });
    group.bench_function("select_and_build/small/pg", |b| {
        b.iter(|| select_small.build(PostgresQueryBuilder))
    });
    group.bench_function("select_and_build/small/sqlite", |b| {
        b.iter(|| select_small.build(SqliteQueryBuilder))
    });
    group.bench_function("select_and_to_string/small/mysql", |b| {
        b.iter(|| select_small.to_string(MysqlQueryBuilder))
    });
    group.bench_function("select_and_to_string/small/pg", |b| {
        b.iter(|| select_small.to_string(PostgresQueryBuilder))
    });
    group.bench_function("select_and_to_string/small/sqlite", |b| {
        b.iter(|| select_small.to_string(SqliteQueryBuilder))
    });

    let select_large = black_box(large_select(value));
    group.bench_function("select_and_build/large/mysql", |b| {
        b.iter(|| select_large.build(MysqlQueryBuilder))
    });
    group.bench_function("select_and_build/large/pg", |b| {
        b.iter(|| select_large.build(PostgresQueryBuilder))
    });
    group.bench_function("select_and_build/large/sqlite", |b| {
        b.iter(|| select_large.build(SqliteQueryBuilder))
    });
    group.bench_function("select_and_to_string/large/mysql", |b| {
        b.iter(|| select_large.to_string(MysqlQueryBuilder))
    });
    group.bench_function("select_and_to_string/large/pg", |b| {
        b.iter(|| select_large.to_string(PostgresQueryBuilder))
    });
    group.bench_function("select_and_to_string/large/sqlite", |b| {
        b.iter(|| select_large.to_string(SqliteQueryBuilder))
    });

    group.finish();
}

fn config() -> Criterion {
    Criterion::default().measurement_time(std::time::Duration::new(10, 0))
}

criterion_group!(
    name = benches;
    config = config();
    targets = criterion_benchmark
);
criterion_main!(benches);
