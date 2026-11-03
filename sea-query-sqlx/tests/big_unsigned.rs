use sea_query::{Value, Values};
use sea_query_sqlx::SqlxValues;

fn big_unsigned(i: u64) -> SqlxValues {
    SqlxValues(Values(vec![Value::BigUnsigned(Some(i))]))
}

#[cfg(feature = "sqlx-postgres")]
#[tokio::test]
async fn postgres_big_unsigned() {
    use sqlx::postgres::PgArguments;

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let pool = sqlx::PgPool::connect(&url).await.unwrap();

    let args = PgArguments::try_from(big_unsigned(i64::MAX as u64));
    let n: i64 = sqlx::query_scalar_with("SELECT $1", args.unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(n, i64::MAX);

    let err = PgArguments::try_from(big_unsigned(u64::MAX));
    assert!(matches!(
        err,
        Err(sea_query_sqlx::IntoArgumentsError::Encode(_))
    ));
}

#[cfg(all(feature = "sqlx-postgres", feature = "postgres-array"))]
#[tokio::test]
async fn postgres_big_unsigned_array() {
    use sqlx::postgres::PgArguments;

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let pool = sqlx::PgPool::connect(&url).await.unwrap();
    let array = |vec: Vec<u64>| SqlxValues(Values(vec![vec.into()]));

    let args = PgArguments::try_from(array(vec![1, i64::MAX as u64]));
    let v: Vec<i64> = sqlx::query_scalar_with("SELECT $1", args.unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(v, [1, i64::MAX]);

    let err = PgArguments::try_from(array(vec![1, u64::MAX]));
    assert!(matches!(
        err,
        Err(sea_query_sqlx::IntoArgumentsError::Encode(_))
    ));
}

#[cfg(feature = "sqlx-sqlite")]
#[tokio::test]
async fn sqlite_big_unsigned() {
    use sqlx::sqlite::SqliteArguments;

    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();

    let args = SqliteArguments::try_from(big_unsigned(i64::MAX as u64));
    let n: i64 = sqlx::query_scalar_with("SELECT ?", args.unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(n, i64::MAX);

    let err = SqliteArguments::try_from(big_unsigned(u64::MAX));
    assert!(matches!(
        err,
        Err(sea_query_sqlx::IntoArgumentsError::Encode(_))
    ));
}

#[cfg(feature = "sqlx-mysql")]
#[tokio::test]
async fn mysql_big_unsigned() {
    use sqlx::mysql::MySqlArguments;

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let pool = sqlx::MySqlPool::connect(&url).await.unwrap();

    let args = MySqlArguments::try_from(big_unsigned(u64::MAX));
    let n: u64 = sqlx::query_scalar_with("SELECT CAST(? AS UNSIGNED)", args.unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(n, u64::MAX);
}
