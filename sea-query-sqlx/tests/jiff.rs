#[cfg(all(feature = "sqlx-sqlite", feature = "with-jiff"))]
#[tokio::test]
async fn sqlite_jiff_values_round_trip() {
    use sea_query::Values;
    use sea_query_sqlx::SqlxValues;
    use sqlx::sqlite::SqliteArguments;

    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    let date = jiff::civil::date(2024, 2, 29);
    let args = SqliteArguments::try_from(SqlxValues(Values(vec![date.into()]))).unwrap();
    let result: String = sqlx::query_scalar_with("SELECT ?", args)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(result, "2024-02-29");
}

#[cfg(all(
    feature = "sqlx-postgres",
    feature = "with-jiff",
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
