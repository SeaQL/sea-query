use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use sea_query::*;

#[derive(Iden)]
enum T {
    Table,
    Id,
}

fn select_stmt() -> SelectStatement {
    let mut q = Query::select();
    for i in 0..20 {
        q.column((T::Table, Alias::new(format!("col_{i}"))));
    }
    q.from(T::Table)
        .inner_join(
            Alias::new("users"),
            Expr::col((T::Table, T::Id)).equals((Alias::new("users"), T::Id)),
        )
        .left_join(
            Alias::new("orgs"),
            Expr::col((Alias::new("users"), Alias::new("org_id")))
                .equals((Alias::new("orgs"), T::Id)),
        )
        .and_where(Expr::col((T::Table, Alias::new("col_1"))).eq("some text value"))
        .and_where(Expr::col((T::Table, Alias::new("col_2"))).gt(42))
        .and_where(Expr::col((T::Table, Alias::new("col_3"))).is_in(["a", "b", "c", "d"]))
        .and_where(Expr::col((T::Table, Alias::new("col_4"))).like("%needle%"))
        .order_by((T::Table, Alias::new("col_5")), Order::Desc)
        .limit(50)
        .offset(100);
    q
}

fn select_cond_stmt() -> SelectStatement {
    let mut q = Query::select();
    q.columns([T::Id]).from(T::Table).cond_where(
        Condition::all()
            .add(Expr::col(T::Id).eq(1))
            .add(Expr::col(Alias::new("a")).eq("some text value"))
            .add(Expr::col(Alias::new("b")).like("%needle in a haystack%"))
            .add(
                Condition::any()
                    .add(Expr::col(Alias::new("c")).is_in(["a", "b", "c", "d"]))
                    .add(Expr::col(Alias::new("d")).gt(42))
                    .add(Expr::col(Alias::new("e")).eq("another text value")),
            )
            .add(Expr::col(Alias::new("f")).eq("yet another text value")),
    );
    q
}

fn insert_stmt() -> InsertStatement {
    let mut q = Query::insert();
    q.into_table(T::Table)
        .columns((0..10).map(|i| Alias::new(format!("c{i}"))));
    for r in 0..100 {
        q.values_panic([
            r.into(),
            "some string value".into(),
            "another one with 'quote'".into(),
            1.5f64.into(),
            true.into(),
            (r as i64).into(),
            "x".into(),
            r.into(),
            "yy".into(),
            7u32.into(),
        ]);
    }
    q
}

fn insert_text_stmt() -> InsertStatement {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor. "
        .repeat(6);
    let mut q = Query::insert();
    q.into_table(T::Table)
        .columns([Alias::new("a"), Alias::new("b")]);
    for _ in 0..100 {
        q.values_panic([text.as_str().into(), "it's quoted".into()]);
    }
    q
}

fn bench(c: &mut Criterion) {
    let s = select_stmt();
    let i = insert_stmt();
    let it = insert_text_stmt();
    c.bench_function("insert_text_to_string_pg", |b| {
        b.iter(|| black_box(it.to_string(PostgresQueryBuilder)))
    });
    c.bench_function("insert_text_to_string_sqlite", |b| {
        b.iter(|| black_box(it.to_string(SqliteQueryBuilder)))
    });
    let sc = select_cond_stmt();
    c.bench_function("select_cond_to_string_pg", |b| {
        b.iter(|| black_box(sc.to_string(PostgresQueryBuilder)))
    });
    c.bench_function("select_cond_build_pg", |b| {
        b.iter(|| black_box(sc.build(PostgresQueryBuilder)))
    });
    c.bench_function("select_to_string_pg", |b| {
        b.iter(|| black_box(s.to_string(PostgresQueryBuilder)))
    });
    c.bench_function("select_build_pg", |b| {
        b.iter(|| black_box(s.build(PostgresQueryBuilder)))
    });
    c.bench_function("select_to_string_mysql", |b| {
        b.iter(|| black_box(s.to_string(MysqlQueryBuilder)))
    });
    c.bench_function("insert_to_string_pg", |b| {
        b.iter(|| black_box(i.to_string(PostgresQueryBuilder)))
    });
    c.bench_function("insert_build_pg", |b| {
        b.iter(|| black_box(i.build(PostgresQueryBuilder)))
    });
    c.bench_function("insert_build_sqlite", |b| {
        b.iter(|| black_box(i.build(SqliteQueryBuilder)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
