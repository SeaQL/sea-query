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

    let ok = PgArguments::try_from(values(vec![Value::Array(
        ArrayType::Range,
        Some(Box::new(vec![
            int4(RangeBound::Inclusive(1), RangeBound::Exclusive(2)),
            Value::Range(None),
            int4(RangeBound::Inclusive(5), RangeBound::Exclusive(9)),
        ])),
    )]));
    assert!(ok.is_ok());

    let err = PgArguments::try_from(values(vec![Value::Array(
        ArrayType::Range,
        Some(Box::new(vec![
            int4(RangeBound::Inclusive(1), RangeBound::Exclusive(2)),
            int8(RangeBound::Inclusive(1), RangeBound::Exclusive(2)),
        ])),
    )]));
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
    any(feature = "with-bigdecimal", feature = "with-rust_decimal")
))]
#[tokio::test]
async fn postgres_range_round_trip() {
    use sea_query::{Alias, Expr, ExprTrait, PostgresQueryBuilder, Query};
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
        "CREATE TABLE sea_query_range (id int PRIMARY KEY, i4 int4range, i8 int8range, n numrange)",
    )
    .execute(&mut conn)
    .await
    .unwrap();

    let table = Alias::new("sea_query_range");

    let (sql, values) = Query::insert()
        .into_table(table.clone())
        .columns(["id", "i4", "i8", "n"])
        .values_panic([
            1.into(),
            int4(RangeBound::Inclusive(1), RangeBound::Exclusive(10)).into(),
            int8(RangeBound::Exclusive(1), RangeBound::Inclusive(5)).into(),
            num(RangeBound::Inclusive(0.1), RangeBound::Unbounded).into(),
        ])
        .values_panic([
            2.into(),
            Value::Range(None).into(),
            Value::Range(None).into(),
            Value::Range(None).into(),
        ])
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(sqlx::AssertSqlSafe(sql.as_str()), values)
        .execute(&mut conn)
        .await
        .unwrap();

    type Row = (i32, Option<String>, Option<String>, Option<String>);

    let rows: Vec<Row> =
        sqlx::query_as("SELECT id, i4::text, i8::text, n::text FROM sea_query_range ORDER BY id")
            .fetch_all(&mut conn)
            .await
            .unwrap();

    assert_eq!(
        rows,
        vec![
            (
                1,
                Some("[1,10)".to_owned()),
                // int8range is canonicalized to the [) form
                Some("[2,6)".to_owned()),
                Some("[0.1,)".to_owned()),
            ),
            (2, None, None, None),
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
        .and_where(Expr::col("i4").eq(int4(RangeBound::Inclusive(1), RangeBound::Inclusive(9))))
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
