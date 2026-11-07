#![cfg(all(feature = "postgres-range", feature = "postgres-array"))]

//! Round trip against a real database, run with `DATABASE_URL` set.

use postgres::{Client, NoTls};
use sea_query::prelude::{RangeBound, RangeType};
use sea_query::{Alias, ArrayType, PostgresQueryBuilder, Query, Value};
use sea_query_postgres::PostgresBinder;

fn connect() -> Option<Client> {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("DATABASE_URL not set, skipping");
        return None;
    };
    Some(Client::connect(&url, NoTls).unwrap())
}

fn int4(start: RangeBound<i32>, end: RangeBound<i32>) -> RangeType {
    RangeType::Int4Range(start, end)
}

fn int8(start: RangeBound<i64>, end: RangeBound<i64>) -> RangeType {
    RangeType::Int8Range(start, end)
}

fn num(start: RangeBound<f64>, end: RangeBound<f64>) -> RangeType {
    RangeType::NumRange(start, end)
}

type Row = (
    i32,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
);

const SELECT: &str = "SELECT id, i4::text, i8::text, n::text, i4a::text, i8a::text, na::text \
                      FROM sea_query_pg_range ORDER BY id";

fn fetch(client: &mut Client) -> Vec<Row> {
    client
        .query(SELECT, &[])
        .unwrap()
        .iter()
        .map(|r| {
            (
                r.get(0),
                r.get(1),
                r.get(2),
                r.get(3),
                r.get(4),
                r.get(5),
                r.get(6),
            )
        })
        .collect()
}

fn s(v: &str) -> Option<String> {
    Some(v.to_owned())
}

#[test]
fn postgres_range_round_trip() {
    use RangeBound::*;

    let Some(mut client) = connect() else {
        return;
    };

    client
        .batch_execute(
            "DROP TABLE IF EXISTS sea_query_pg_range;
             CREATE TABLE sea_query_pg_range (
                 id int PRIMARY KEY,
                 i4 int4range, i8 int8range, n numrange,
                 i4a int4range[], i8a int8range[], na numrange[]
             )",
        )
        .unwrap();

    let table = Alias::new("sea_query_pg_range");
    let columns = ["id", "i4", "i8", "n", "i4a", "i8a", "na"];

    let row = |id: i32, ranges: [Value; 3], arrays: [Value; 3]| -> [sea_query::Expr; 7] {
        let [i4, i8, n] = ranges;
        let [i4a, i8a, na] = arrays;
        [
            id.into(),
            i4.into(),
            i8.into(),
            n.into(),
            i4a.into(),
            i8a.into(),
            na.into(),
        ]
    };

    let (sql, values) = Query::insert()
        .into_table(table.clone())
        .columns(columns)
        .values_panic(row(
            1,
            [
                int4(Inclusive(1), Exclusive(10)).into(),
                int8(Exclusive(1), Inclusive(5)).into(),
                num(Inclusive(0.1), Unbounded).into(),
            ],
            [
                vec![
                    int4(Inclusive(1), Exclusive(2)),
                    int4(Unbounded, Inclusive(3)),
                ]
                .into(),
                vec![int8(Inclusive(-5), Exclusive(5))].into(),
                vec![
                    num(Inclusive(-1234.5678), Exclusive(1e20)),
                    num(Exclusive(0.00012), Inclusive(1.5)),
                ]
                .into(),
            ],
        ))
        .values_panic(row(
            2,
            [
                // `(,)` is the range of all values, not an empty range
                int4(Unbounded, Unbounded).into(),
                int8(Unbounded, Unbounded).into(),
                num(Unbounded, Unbounded).into(),
            ],
            [
                // empty ranges are sent as they are and normalized by PostgreSQL
                vec![int4(Exclusive(1), Exclusive(2))].into(),
                vec![int8(Inclusive(3), Exclusive(3))].into(),
                vec![num(Inclusive(0.5), Exclusive(0.5))].into(),
            ],
        ))
        .values_panic(row(
            3,
            [Value::Range(None), Value::Range(None), Value::Range(None)],
            [
                Vec::<RangeType>::new().into(),
                Value::Array(ArrayType::Range, Some(Box::new(vec![Value::Range(None)]))),
                Value::Array(ArrayType::Range, None),
            ],
        ))
        .build_postgres(PostgresQueryBuilder);

    client.execute(&sql, &values.as_params()).unwrap();

    assert_eq!(
        fetch(&mut client),
        vec![
            (
                1,
                s("[1,10)"),
                // int8range is canonicalized to the [) form
                s("[2,6)"),
                s("[0.1,)"),
                s(r#"{"[1,2)","(,4)"}"#),
                s(r#"{"[-5,5)"}"#),
                s(r#"{"[-1234.5678,100000000000000000000)","(0.00012,1.5]"}"#),
            ),
            (
                2,
                s("(,)"),
                s("(,)"),
                s("(,)"),
                s("{empty}"),
                s("{empty}"),
                s("{empty}"),
            ),
            (3, None, None, None, s("{}"), s("{NULL}"), None),
        ]
    );
    assert!(int4(Exclusive(1), Exclusive(2)).empty());
    assert!(!int4(Unbounded, Unbounded).empty());

    // the same scalar ranges rendered as SQL literals give the same values
    client
        .batch_execute("DELETE FROM sea_query_pg_range WHERE id IN (1, 2)")
        .unwrap();
    let sql = Query::insert()
        .into_table(table.clone())
        .columns(["id", "i4", "i8", "n"])
        .values_panic([
            1.into(),
            int4(Inclusive(1), Exclusive(10)).into(),
            int8(Exclusive(1), Inclusive(5)).into(),
            num(Inclusive(0.1), Unbounded).into(),
        ])
        .values_panic([
            2.into(),
            int4(Unbounded, Unbounded).into(),
            int8(Unbounded, Unbounded).into(),
            num(Unbounded, Unbounded).into(),
        ])
        .to_string(PostgresQueryBuilder);
    client.batch_execute(&sql).unwrap();

    let scalars: Vec<_> = fetch(&mut client)
        .into_iter()
        .map(|(id, i4, i8, n, ..)| (id, i4, i8, n))
        .collect();
    assert_eq!(
        scalars,
        vec![
            (1, s("[1,10)"), s("[2,6)"), s("[0.1,)")),
            (2, s("(,)"), s("(,)"), s("(,)")),
            (3, None, None, None),
        ]
    );

    // `as_types` works with `prepare_typed`
    let (sql, values) = Query::insert()
        .into_table(table)
        .columns(columns)
        .values_panic(row(
            4,
            [
                int4(Inclusive(1), Exclusive(2)).into(),
                Value::Range(None),
                num(Inclusive(2.5), Inclusive(3.5)).into(),
            ],
            [
                vec![int4(Inclusive(1), Exclusive(2))].into(),
                Vec::<RangeType>::new().into(),
                vec![num(Inclusive(2.5), Inclusive(3.5))].into(),
            ],
        ))
        .build_postgres(PostgresQueryBuilder);
    let statement = client.prepare_typed(&sql, &values.as_types()).unwrap();
    client.execute(&statement, &values.as_params()).unwrap();

    assert_eq!(
        fetch(&mut client).pop().unwrap(),
        (
            4,
            s("[1,2)"),
            None,
            s("[2.5,3.5]"),
            s(r#"{"[1,2)"}"#),
            s("{}"),
            s(r#"{"[2.5,3.5]"}"#),
        )
    );

    // a range of the wrong type, or mixed range types in one array, fail to encode
    let (sql, values) = Query::insert()
        .into_table(Alias::new("sea_query_pg_range"))
        .columns(["id", "i4a"])
        .values_panic([
            5.into(),
            Value::Array(
                ArrayType::Range,
                Some(Box::new(vec![
                    int4(Inclusive(1), Exclusive(2)).into(),
                    int8(Inclusive(1), Exclusive(2)).into(),
                ])),
            )
            .into(),
        ])
        .build_postgres(PostgresQueryBuilder);
    assert!(client.execute(&sql, &values.as_params()).is_err());

    client
        .batch_execute("DROP TABLE sea_query_pg_range")
        .unwrap();
}
