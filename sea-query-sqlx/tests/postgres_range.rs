#![cfg(feature = "postgres-range")]

use sea_query::prelude::{RangeBound, RangeType};
use sea_query::{Value, Values};
use sea_query_sqlx::{IntoArgumentsError, SqlxValues};

fn values(values: Vec<Value>) -> SqlxValues {
    SqlxValues(Values(values))
}

fn int4(start: RangeBound<i32>, end: RangeBound<i32>) -> Value {
    RangeType::Int4Range(start, end).into()
}

fn int8(start: RangeBound<i64>, end: RangeBound<i64>) -> Value {
    RangeType::Int8Range(start, end).into()
}

fn num(start: RangeBound<f64>, end: RangeBound<f64>) -> Value {
    RangeType::NumRange(start, end).into()
}

#[cfg(feature = "sqlx-postgres")]
#[test]
fn postgres_encodes_ranges() {
    use sqlx::postgres::PgArguments;

    let args = PgArguments::try_from(values(vec![
        int4(RangeBound::Inclusive(1), RangeBound::Exclusive(10)),
        int8(RangeBound::Unbounded, RangeBound::Inclusive(5)),
        Value::Range(None),
    ]));

    assert!(args.is_ok());
}

#[cfg(all(
    feature = "sqlx-postgres",
    not(any(feature = "with-bigdecimal", feature = "with-rust_decimal"))
))]
#[test]
fn postgres_numrange_needs_decimal_feature() {
    use sqlx::postgres::PgArguments;

    let err = PgArguments::try_from(values(vec![num(
        RangeBound::Inclusive(0.5),
        RangeBound::Unbounded,
    )]));

    assert!(matches!(err, Err(IntoArgumentsError::Unsupported(_))));
}

#[cfg(all(
    feature = "sqlx-postgres",
    any(feature = "with-bigdecimal", feature = "with-rust_decimal")
))]
#[test]
fn postgres_numrange_rejects_non_finite_bounds() {
    use sqlx::postgres::PgArguments;

    let err = PgArguments::try_from(values(vec![num(
        RangeBound::Inclusive(f64::NAN),
        RangeBound::Unbounded,
    )]));

    assert!(matches!(err, Err(IntoArgumentsError::Encode(_))));
}

#[cfg(all(feature = "sqlx-postgres", feature = "postgres-array"))]
#[test]
fn postgres_range_array_must_be_homogeneous() {
    use sea_query::ArrayType;
    use sqlx::postgres::PgArguments;

    // through the array API, `Vec<RangeType>` -> `Value::Array`
    let int4_array: Value = vec![
        RangeType::Int4Range(RangeBound::Inclusive(1), RangeBound::Exclusive(2)),
        RangeType::Int4Range(RangeBound::Inclusive(5), RangeBound::Exclusive(9)),
    ]
    .into();
    let int8_array: Value = vec![RangeType::Int8Range(
        RangeBound::Unbounded,
        RangeBound::Inclusive(3),
    )]
    .into();
    assert!(matches!(
        int4_array,
        Value::Array(ArrayType::Range, Some(_))
    ));
    assert!(PgArguments::try_from(values(vec![int4_array, int8_array])).is_ok());

    // NULL elements are skipped when inferring the range type
    let ok = PgArguments::try_from(values(vec![Value::Array(
        ArrayType::Range,
        Some(Box::new(vec![
            Value::Range(None),
            int8(RangeBound::Inclusive(1), RangeBound::Exclusive(2)),
        ])),
    )]));
    assert!(ok.is_ok());

    // a NULL array is sent untyped
    let ok = PgArguments::try_from(values(vec![Value::Array(ArrayType::Range, None)]));
    assert!(ok.is_ok());

    let err = PgArguments::try_from(values(vec![Value::Array(
        ArrayType::Range,
        Some(Box::new(vec![
            int4(RangeBound::Inclusive(1), RangeBound::Exclusive(2)),
            int8(RangeBound::Inclusive(1), RangeBound::Exclusive(2)),
        ])),
    )]));
    assert!(matches!(err, Err(IntoArgumentsError::Unsupported(_))));

    // empty and all-NULL arrays have no range type
    let err = PgArguments::try_from(values(vec![Vec::<RangeType>::new().into()]));
    assert!(matches!(err, Err(IntoArgumentsError::Unsupported(_))));
    let err = PgArguments::try_from(values(vec![Value::Array(
        ArrayType::Range,
        Some(Box::new(vec![Value::Range(None)])),
    )]));
    assert!(matches!(err, Err(IntoArgumentsError::Unsupported(_))));
}

#[cfg(all(
    feature = "sqlx-postgres",
    feature = "postgres-array",
    any(feature = "with-bigdecimal", feature = "with-rust_decimal")
))]
#[test]
fn postgres_encodes_numrange_arrays() {
    use sqlx::postgres::PgArguments;

    let array: Value = vec![
        RangeType::NumRange(RangeBound::Inclusive(0.5), RangeBound::Unbounded),
        RangeType::NumRange(RangeBound::Exclusive(-1.25), RangeBound::Inclusive(3.0)),
    ]
    .into();
    assert!(PgArguments::try_from(values(vec![array])).is_ok());

    let err = PgArguments::try_from(values(vec![
        vec![RangeType::NumRange(
            RangeBound::Inclusive(f64::INFINITY),
            RangeBound::Unbounded,
        )]
        .into(),
    ]));
    assert!(matches!(err, Err(IntoArgumentsError::Encode(_))));
}

#[cfg(all(
    feature = "sqlx-postgres",
    feature = "postgres-array",
    not(any(feature = "with-bigdecimal", feature = "with-rust_decimal"))
))]
#[test]
fn postgres_numrange_array_needs_decimal_feature() {
    use sqlx::postgres::PgArguments;

    let err = PgArguments::try_from(values(vec![
        vec![RangeType::NumRange(
            RangeBound::Inclusive(0.5),
            RangeBound::Unbounded,
        )]
        .into(),
    ]));
    assert!(matches!(err, Err(IntoArgumentsError::Unsupported(_))));
}

#[cfg(feature = "sqlx-mysql")]
#[test]
fn mysql_rejects_ranges() {
    let err = sqlx::mysql::MySqlArguments::try_from(values(vec![int4(
        RangeBound::Inclusive(1),
        RangeBound::Exclusive(2),
    )]));

    assert!(matches!(err, Err(IntoArgumentsError::Unsupported(_))));
}

#[cfg(feature = "sqlx-sqlite")]
#[test]
fn sqlite_rejects_ranges() {
    let err = sqlx::sqlite::SqliteArguments::try_from(values(vec![int4(
        RangeBound::Inclusive(1),
        RangeBound::Exclusive(2),
    )]));

    assert!(matches!(err, Err(IntoArgumentsError::Unsupported(_))));
}

/// Round trip against a real database, run with `DATABASE_URL` set.
#[cfg(all(
    feature = "sqlx-postgres",
    feature = "postgres-array",
    any(feature = "with-bigdecimal", feature = "with-rust_decimal")
))]
#[tokio::test]
async fn postgres_range_round_trip() {
    use RangeBound::*;
    use sea_query::{Alias, ArrayType, Expr, ExprTrait, PostgresQueryBuilder, Query};
    use sea_query_sqlx::SqlxBinder;

    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("DATABASE_URL not set, skipping");
        return;
    };

    use sqlx::Connection;

    let mut conn = sqlx::PgConnection::connect(&url).await.unwrap();

    sqlx::query("DROP TABLE IF EXISTS sea_query_range")
        .execute(&mut conn)
        .await
        .unwrap();
    sqlx::query(
        "CREATE TABLE sea_query_range (
            id int PRIMARY KEY,
            i4 int4range, i8 int8range, n numrange,
            i4a int4range[], i8a int8range[], na numrange[]
        )",
    )
    .execute(&mut conn)
    .await
    .unwrap();

    let table = Alias::new("sea_query_range");
    let columns = ["id", "i4", "i8", "n", "i4a", "i8a", "na"];

    let (sql, values) = Query::insert()
        .into_table(table.clone())
        .columns(columns)
        .values_panic([
            1.into(),
            int4(Inclusive(1), Exclusive(10)).into(),
            int8(Exclusive(1), Inclusive(5)).into(),
            num(Inclusive(0.1), Unbounded).into(),
            Value::from(vec![
                RangeType::Int4Range(Inclusive(1), Exclusive(2)),
                RangeType::Int4Range(Unbounded, Inclusive(3)),
            ])
            .into(),
            Value::from(vec![RangeType::Int8Range(Inclusive(-5), Exclusive(5))]).into(),
            Value::from(vec![
                RangeType::NumRange(Inclusive(-1234.5678), Exclusive(1e20)),
                RangeType::NumRange(Exclusive(0.00012), Inclusive(1.5)),
            ])
            .into(),
        ])
        .values_panic([
            2.into(),
            // `(,)` is the range of all values, not an empty range
            int4(Unbounded, Unbounded).into(),
            int8(Unbounded, Unbounded).into(),
            num(Unbounded, Unbounded).into(),
            // empty ranges are sent as they are and normalized by PostgreSQL
            Value::from(vec![RangeType::Int4Range(Exclusive(1), Exclusive(2))]).into(),
            Value::from(vec![RangeType::Int8Range(Inclusive(3), Exclusive(3))]).into(),
            Value::from(vec![RangeType::NumRange(Inclusive(0.5), Exclusive(0.5))]).into(),
        ])
        .values_panic([
            3.into(),
            Value::Range(None).into(),
            Value::Range(None).into(),
            Value::Range(None).into(),
            Value::Array(
                ArrayType::Range,
                Some(Box::new(vec![
                    Value::Range(None),
                    int4(Inclusive(7), Inclusive(8)),
                ])),
            )
            .into(),
            Value::Array(ArrayType::Range, None).into(),
            Value::Array(ArrayType::Range, None).into(),
        ])
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(sqlx::AssertSqlSafe(sql.as_str()), values)
        .execute(&mut conn)
        .await
        .unwrap();

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
                          FROM sea_query_range ORDER BY id";
    let s = |v: &str| Some(v.to_owned());

    let rows: Vec<Row> = sqlx::query_as(SELECT).fetch_all(&mut conn).await.unwrap();

    assert_eq!(
        rows,
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
            (3, None, None, None, s(r#"{NULL,"[7,9)"}"#), None, None),
        ]
    );

    // the same scalar ranges rendered as SQL literals give the same values
    sqlx::query("DELETE FROM sea_query_range WHERE id IN (1, 2)")
        .execute(&mut conn)
        .await
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
    sqlx::query(sqlx::AssertSqlSafe(sql.as_str()))
        .execute(&mut conn)
        .await
        .unwrap();

    let rows: Vec<Row> = sqlx::query_as(SELECT).fetch_all(&mut conn).await.unwrap();
    let scalars: Vec<_> = rows
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

    // ranges can be used as query parameters, too
    let (sql, values) = Query::select()
        .column("id")
        .from(table)
        .and_where(Expr::col("i4").binary(
            sea_query::extension::postgres::PgBinOper::Contains,
            Expr::val(5),
        ))
        .and_where(Expr::col("i4").eq(int4(Inclusive(1), Inclusive(9))))
        .build_sqlx(PostgresQueryBuilder);

    let ids: Vec<i32> = sqlx::query_scalar_with(sqlx::AssertSqlSafe(sql.as_str()), values)
        .fetch_all(&mut conn)
        .await
        .unwrap();
    assert_eq!(ids, vec![1]);

    sqlx::query("DROP TABLE sea_query_range")
        .execute(&mut conn)
        .await
        .unwrap();
}
