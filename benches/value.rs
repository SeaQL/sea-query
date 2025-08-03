use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use sea_query::*;

#[derive(Debug, Iden)]
pub enum Char {
    Table,
    Id,
    Character,
}

fn small_type(value: i32) -> SelectStatement {
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

fn large_type(value: jiff::Zoned) -> SelectStatement {
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
        b.iter(|| small_type(black_box(123)))
    });

    group.bench_function("select_construction/large", |b| {
        b.iter(|| large_type(value.clone()))
    });

    let select_small = black_box(small_type(black_box(123)));
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

    let select_large = black_box(large_type(value));
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
