#[cfg(all(
    feature = "sqlx-sqlite",
    any(feature = "with-jiff-0_2", feature = "with-jiff-1")
))]
#[tokio::test]
async fn sqlite_binds_jiff_dates() {
    use sea_query::{Value, Values};
    use sea_query_sqlx::SqlxValues;
    use sqlx::sqlite::SqliteArguments;

    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    let values: Vec<(Value, &str)> = vec![
        #[cfg(feature = "with-jiff-0_2")]
        (
            sea_query::prelude::jiff::civil::date(2024, 2, 29).into(),
            "2024-02-29",
        ),
        #[cfg(feature = "with-jiff-1")]
        (
            sea_query::prelude::jiff_1::civil::date(2023, 3, 1).into(),
            "2023-03-01",
        ),
    ];
    for (value, expected) in values {
        let args = SqliteArguments::try_from(SqlxValues(Values(vec![value]))).unwrap();
        let result: String = sqlx::query_scalar_with("SELECT ?", args)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(result, expected);
    }
}

#[cfg(all(
    feature = "sqlx-postgres",
    feature = "with-jiff-0_2",
    feature = "postgres-array"
))]
#[test]
fn postgres_jiff_arrays_reject_invalid_elements() {
    use sea_query::{ArrayType, Value, Values};
    use sea_query_sqlx::{IntoArgumentsError, SqlxValues};
    use sqlx::postgres::PgArguments;

    let value = Value::Array(
        ArrayType::JiffDate,
        Some(Box::new(vec![Value::Int(Some(1))])),
    );
    let result = PgArguments::try_from(SqlxValues(Values(vec![value])));
    assert!(matches!(
        result,
        Err(IntoArgumentsError::InvalidArray { .. })
    ));
}

#[cfg(all(feature = "sqlx-mysql", feature = "with-jiff"))]
#[test]
fn mysql_rejects_jiff_arguments() {
    use sea_query::{Value, Values, prelude::jiff};
    use sea_query_sqlx::{IntoArgumentsError, SqlxValues};
    use sqlx::mysql::MySqlArguments;

    let date = jiff::civil::date(2024, 2, 29);
    let values = [
        date.into(),
        date.at(1, 2, 3, 0).time().into(),
        date.at(1, 2, 3, 0).into(),
        jiff::Timestamp::UNIX_EPOCH.into(),
        Value::JiffDate(None),
        Value::JiffTime(None),
        Value::JiffDateTime(None),
        Value::JiffTimestamp(None),
    ];
    for value in values {
        let result = MySqlArguments::try_from(SqlxValues(Values(vec![value])));
        assert!(matches!(result, Err(IntoArgumentsError::Unsupported(_))));
    }
}
