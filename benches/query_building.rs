use criterion::{criterion_group, criterion_main, Criterion};
use sea_query::{tests_cfg::*, *};

fn select() -> SelectStatement {
    Query::select()
        .column(Char::Character)
        .column((Font::Table, Font::Name))
        .from(Char::Table)
        .left_join(
            Font::Table,
            Expr::tbl(Char::Table, Char::FontId).equals(Font::Table, Font::Id),
        )
        .and_where(Expr::col(Char::SizeW).is_in(vec![3, 4]))
        .and_where(Expr::col(Char::Character).like("A%"))
        .to_owned()
}

fn select_and_to_string() {
    select().to_string(MysqlQueryBuilder);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("select", |b| b.iter(|| select()));
    c.bench_function("select_and_to_string", |b| {
        b.iter(|| select_and_to_string())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
