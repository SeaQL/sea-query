#![cfg(feature = "postgres-range")]

use sea_query::prelude::{RangeBound::*, RangeType::*};
use sea_query::{Value, Values};
use sea_query_sqlx::{IntoArgumentsError, SqlxValues};

fn values(values: Vec<Value>) -> SqlxValues {
    SqlxValues(Values(values))
}

#[cfg(feature = "sqlx-postgres")]
fn pg_args(v: Vec<Value>) -> Result<sqlx::postgres::PgArguments, IntoArgumentsError> {
    sqlx::postgres::PgArguments::try_from(values(v))
}

#[cfg(feature = "sqlx-postgres")]
#[test]
fn postgres_scalar_ranges() {
    assert!(
        pg_args(vec![
            Int4Range(Inclusive(1), Exclusive(10)).into(),
            Int8Range(Unbounded, Inclusive(5)).into(),
            Value::Range(None),
        ])
        .is_ok()
    );

    let result = pg_args(vec![NumRange(Inclusive(0.5), Unbounded).into()]);
    #[cfg(any(feature = "with-bigdecimal", feature = "with-rust_decimal"))]
    {
        assert!(result.is_ok());
        let err = pg_args(vec![NumRange(Inclusive(f64::NAN), Unbounded).into()]);
        assert!(matches!(err, Err(IntoArgumentsError::Encode(_))));
    }
    #[cfg(not(any(feature = "with-bigdecimal", feature = "with-rust_decimal")))]
    assert!(matches!(result, Err(IntoArgumentsError::Unsupported(_))));
}

#[cfg(all(feature = "sqlx-postgres", feature = "postgres-array"))]
#[test]
fn postgres_range_arrays() {
    use sea_query::ArrayType;

    let int4 = Int4Range(Inclusive(1), Exclusive(2));
    let int8 = Int8Range(Inclusive(1), Exclusive(2));
    let array = |v: Vec<Value>| Value::Array(ArrayType::Range, Some(Box::new(v)));

    // the range type is taken from the first non-NULL element
    assert!(pg_args(vec![array(vec![Value::Range(None), int8.clone().into()])]).is_ok());
    assert!(pg_args(vec![Value::Array(ArrayType::Range, None)]).is_ok());

    for unsupported in [
        array(vec![int4.into(), int8.into()]),
        Vec::<sea_query::prelude::RangeType>::new().into(),
        array(vec![Value::Range(None)]),
    ] {
        let err = pg_args(vec![unsupported]);
        assert!(matches!(err, Err(IntoArgumentsError::Unsupported(_))));
    }
}

#[cfg(feature = "sqlx-mysql")]
#[test]
fn mysql_rejects_ranges() {
    let err = sqlx::mysql::MySqlArguments::try_from(values(vec![
        Int4Range(Inclusive(1), Exclusive(2)).into(),
    ]));
    assert!(matches!(err, Err(IntoArgumentsError::Unsupported(_))));
}

#[cfg(feature = "sqlx-sqlite")]
#[test]
fn sqlite_rejects_ranges() {
    let err = sqlx::sqlite::SqliteArguments::try_from(values(vec![
        Int4Range(Inclusive(1), Exclusive(2)).into(),
    ]));
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
    use sea_query::extension::postgres::PgBinOper;
    use sea_query::{Alias, ArrayType, Expr, ExprTrait, PostgresQueryBuilder, Query};
    use sea_query_sqlx::SqlxBinder;
    use sqlx::Connection;

    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("DATABASE_URL not set, skipping");
        return;
    };
    let mut conn = sqlx::PgConnection::connect(&url).await.unwrap();

    sqlx::query(
        "CREATE TEMPORARY TABLE range_test (
            id int PRIMARY KEY, i4 int4range, i8 int8range, n numrange, i4a int4range[], na numrange[]
        )",
    )
    .execute(&mut conn)
    .await
    .unwrap();

    let table = Alias::new("range_test");
    let (sql, values) = Query::insert()
        .into_table(table.clone())
        .columns(["id", "i4", "i8", "n", "i4a", "na"])
        .values_panic([
            1.into(),
            Int4Range(Inclusive(1), Exclusive(10)).into(),
            Int8Range(Exclusive(1), Inclusive(5)).into(),
            NumRange(Inclusive(0.1), Unbounded).into(),
            Value::Array(
                ArrayType::Range,
                Some(Box::new(vec![
                    Value::Range(None),
                    Int4Range(Inclusive(7), Inclusive(8)).into(),
                ])),
            )
            .into(),
            Value::from(vec![NumRange(Inclusive(-1234.5678), Exclusive(1e20))]).into(),
        ])
        .values_panic([
            // `(,)` is the range of all values, `[3,3)` is empty
            2.into(),
            Int4Range(Unbounded, Unbounded).into(),
            Int8Range(Inclusive(3), Exclusive(3)).into(),
            Value::Range(None).into(),
            Value::Array(ArrayType::Range, None).into(),
            Value::Array(ArrayType::Range, None).into(),
        ])
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(sqlx::AssertSqlSafe(sql.as_str()), values)
        .execute(&mut conn)
        .await
        .unwrap();

    type Text = Option<String>;
    let rows: Vec<(Text, Text, Text, Text, Text)> = sqlx::query_as(
        "SELECT i4::text, i8::text, n::text, i4a::text, na::text FROM range_test ORDER BY id",
    )
    .fetch_all(&mut conn)
    .await
    .unwrap();
    let s = |v: &str| Some(v.to_owned());
    assert_eq!(
        rows,
        [
            (
                s("[1,10)"),
                s("[2,6)"),
                s("[0.1,)"),
                s(r#"{NULL,"[7,9)"}"#),
                s(r#"{"[-1234.5678,100000000000000000000)"}"#),
            ),
            (s("(,)"), s("empty"), None, None, None),
        ]
    );

    // ranges as query parameters
    let (sql, values) = Query::select()
        .column("id")
        .from(table)
        .and_where(Expr::col("i4").binary(PgBinOper::Contains, Expr::val(5)))
        .and_where(Expr::col("i4").eq(Int4Range(Inclusive(1), Inclusive(9))))
        .build_sqlx(PostgresQueryBuilder);
    let ids: Vec<i32> = sqlx::query_scalar_with(sqlx::AssertSqlSafe(sql.as_str()), values)
        .fetch_all(&mut conn)
        .await
        .unwrap();
    assert_eq!(ids, vec![1]);
}
