#![cfg(all(
    feature = "postgres-range",
    feature = "postgres-array",
    feature = "with-rust_decimal"
))]

//! Round trip against a real database, run with `DATABASE_URL` set.

use postgres::{Client, NoTls};
use sea_query::prelude::{RangeBound::*, RangeType::*};
use sea_query::{Alias, ArrayType, PostgresQueryBuilder, Query, Value};
use sea_query_postgres::PostgresBinder;

#[test]
fn postgres_range_round_trip() {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("DATABASE_URL not set, skipping");
        return;
    };
    let mut client = Client::connect(&url, NoTls).unwrap();

    client
        .batch_execute(
            "CREATE TEMPORARY TABLE range_test (
                 id int PRIMARY KEY, i4 int4range, i8 int8range, n numrange, na numrange[]
             )",
        )
        .unwrap();

    let columns = ["id", "i4", "i8", "n", "na"];
    let (sql, values) = Query::insert()
        .into_table(Alias::new("range_test"))
        .columns(columns)
        .values_panic([
            1.into(),
            Int4Range(Inclusive(1), Exclusive(10)).into(),
            Int8Range(Exclusive(1), Inclusive(5)).into(),
            NumRange(Inclusive(0.1), Unbounded).into(),
            Value::from(vec![NumRange(Inclusive(-1234.5678), Exclusive(1e20))]).into(),
        ])
        .values_panic([
            // `(,)` is the range of all values, `[3,3)` is empty
            2.into(),
            Int4Range(Unbounded, Unbounded).into(),
            Int8Range(Inclusive(3), Exclusive(3)).into(),
            Value::Range(None).into(),
            Value::Array(ArrayType::Range, None).into(),
        ])
        .build_postgres(PostgresQueryBuilder);
    client.execute(&sql, &values.as_params()).unwrap();

    // the parameter types from `as_types()`
    let (sql, values) = Query::insert()
        .into_table(Alias::new("range_test"))
        .columns(columns)
        .values_panic([
            3.into(),
            Int4Range(Inclusive(1), Exclusive(2)).into(),
            Value::Range(None).into(),
            NumRange(Inclusive(2.5), Inclusive(3.5)).into(),
            Vec::<sea_query::prelude::RangeType>::new().into(),
        ])
        .build_postgres(PostgresQueryBuilder);
    let statement = client.prepare_typed(&sql, &values.as_types()).unwrap();
    client.execute(&statement, &values.as_params()).unwrap();

    let rows: Vec<[Option<String>; 4]> = client
        .query(
            "SELECT i4::text, i8::text, n::text, na::text FROM range_test ORDER BY id",
            &[],
        )
        .unwrap()
        .iter()
        .map(|r| [r.get(0), r.get(1), r.get(2), r.get(3)])
        .collect();
    let s = |v: &str| Some(v.to_owned());
    assert_eq!(
        rows,
        [
            [
                s("[1,10)"),
                s("[2,6)"),
                s("[0.1,)"),
                s(r#"{"[-1234.5678,100000000000000000000)"}"#),
            ],
            [s("(,)"), s("empty"), None, None],
            [s("[1,2)"), None, s("[2.5,3.5]"), s("{}")],
        ]
    );

    // mixed range types in one array fail to encode
    let (sql, values) = Query::insert()
        .into_table(Alias::new("range_test"))
        .columns(["id", "na"])
        .values_panic([
            4.into(),
            Value::Array(
                ArrayType::Range,
                Some(Box::new(vec![
                    NumRange(Inclusive(1.0), Exclusive(2.0)).into(),
                    Int8Range(Inclusive(1), Exclusive(2)).into(),
                ])),
            )
            .into(),
        ])
        .build_postgres(PostgresQueryBuilder);
    assert!(client.execute(&sql, &values.as_params()).is_err());
}
