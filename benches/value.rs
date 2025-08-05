use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use sea_query::*;

#[derive(Debug, Iden)]
pub enum Char {
    Table,
    Id,
    Character,
}

fn vanilla() -> String {
    format!(
        "SELECT `{}` from `{}` where `character` = {}",
        "character",
        "character".to_owned(),
        "foobar"
    )
}

fn select() -> SelectStatement {
    Query::select()
        .column(Char::Character)
        .from(Char::Table)
        .and_where(Expr::col(Char::Character).eq("foobar"))
        .and_where(Expr::col(Char::Character).eq("foobar"))
        .and_where(Expr::col(Char::Character).eq("foobar"))
        .and_where(Expr::col(Char::Character).eq("foobar"))
        .and_where(Expr::col(Char::Character).eq("foobar"))
        .and_where(Expr::col(Char::Character).eq("foobar"))
        .and_where(Expr::col(Char::Character).eq("foobar"))
        .and_where(Expr::col(Char::Character).eq("foobar"))
        .and_where(Expr::col(Char::Character).eq("foobar"))
        .and_where(Expr::col(Char::Character).eq("foobar"))
        .to_owned()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("value");
    group.bench_function("vanilla", |b| b.iter(vanilla));
    group.bench_function("select", |b| b.iter(select));

    let select = black_box(select());
    group.bench_function("select_and_build::mysql", |b| {
        b.iter(|| select.build(MysqlQueryBuilder))
    });
    group.bench_function("select_and_build::pg", |b| {
        b.iter(|| select.build(PostgresQueryBuilder))
    });
    group.bench_function("select_and_build::sqlite", |b| {
        b.iter(|| select.build(SqliteQueryBuilder))
    });
    group.bench_function("select_and_to_string::mysql", |b| {
        b.iter(|| select.to_string(MysqlQueryBuilder))
    });
    group.bench_function("select_and_to_string::pg", |b| {
        b.iter(|| select.to_string(PostgresQueryBuilder))
    });
    group.bench_function("select_and_to_string::sqlite", |b| {
        b.iter(|| select.to_string(SqliteQueryBuilder))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
