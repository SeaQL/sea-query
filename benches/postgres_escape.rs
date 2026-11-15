use std::hint::black_box;

use hotpath::criterion::{Criterion, criterion_group, criterion_main};
use sea_query::{Alias, Expr, PostgresQueryBuilder, Query, QueryBuilder};

fn bench(c: &mut Criterion) {
    let cases = [
        ("short_plain", "some string value".to_owned()),
        ("short_quote", "it's quoted".to_owned()),
        ("long_plain", "ordinary text ".repeat(320)),
        ("early_quote", format!("'{}", "ordinary text ".repeat(320))),
        (
            "middle_quote",
            format!(
                "{}'{}",
                "ordinary text ".repeat(160),
                "ordinary text ".repeat(160)
            ),
        ),
        ("early_quote_64k", format!("'{}", "x".repeat(65536))),
        ("late_quote", format!("{}'", "ordinary text ".repeat(320))),
        (
            "unicode_sparse",
            "文字😀é with 'quote' and newline\n".repeat(100),
        ),
        ("dense_quotes", "'".repeat(256)),
        ("dense_controls", "\n\t\r\0".repeat(64)),
        ("octal_controls", "\x01\x1f\x7f".repeat(85)),
    ];
    let mut group = c.benchmark_group("postgres_escape");
    for (name, text) in &cases {
        let mut buffer = String::with_capacity(text.len() * 4 + 3);
        group.bench_function(format!("quoted/{name}"), |b| {
            b.iter(|| {
                buffer.clear();
                PostgresQueryBuilder.write_string_quoted(black_box(text), &mut buffer);
                black_box(&buffer);
            });
        });
        let query = Query::select()
            .expr(Expr::val(text.clone()))
            .from(Alias::new("text_values"))
            .to_owned();
        group.bench_function(format!("query/{name}"), |b| {
            b.iter(|| black_box(query.to_string(PostgresQueryBuilder)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
