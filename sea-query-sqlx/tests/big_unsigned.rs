use sea_query::{Value, Values};
use sea_query_sqlx::{SqlxValues, TryIntoArguments};

fn big_unsigned(i: u64) -> SqlxValues {
    SqlxValues(Values(vec![Value::BigUnsigned(Some(i))]))
}

#[cfg(feature = "sqlx-postgres")]
#[tokio::test]
async fn postgres_big_unsigned() {
    use sqlx::postgres::Postgres;

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let pool = sqlx::PgPool::connect(&url).await.unwrap();

    let args = TryIntoArguments::<Postgres>::try_into_arguments(big_unsigned(i64::MAX as u64));
    let n: i64 = sqlx::query_scalar_with("SELECT $1", args.unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(n, i64::MAX);

    let err = TryIntoArguments::<Postgres>::try_into_arguments(big_unsigned(u64::MAX));
    assert!(matches!(err, Err(sqlx::Error::Encode(_))));
}

#[cfg(all(feature = "sqlx-postgres", feature = "postgres-array"))]
#[tokio::test]
async fn postgres_big_unsigned_array() {
    use sea_query::ArrayType;
    use sqlx::postgres::Postgres;

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let pool = sqlx::PgPool::connect(&url).await.unwrap();
    let array = |vec: Vec<u64>| {
        let vec = vec
            .into_iter()
            .map(|i| Value::BigUnsigned(Some(i)))
            .collect();
        SqlxValues(Values(vec![Value::Array(
            ArrayType::BigUnsigned,
            Some(Box::new(vec)),
        )]))
    };

    let args = TryIntoArguments::<Postgres>::try_into_arguments(array(vec![1, i64::MAX as u64]));
    let v: Vec<i64> = sqlx::query_scalar_with("SELECT $1", args.unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(v, [1, i64::MAX]);

    let err = TryIntoArguments::<Postgres>::try_into_arguments(array(vec![1, u64::MAX]));
    assert!(matches!(err, Err(sqlx::Error::Encode(_))));
}

#[cfg(feature = "sqlx-sqlite")]
#[tokio::test]
async fn sqlite_big_unsigned() {
    use sqlx::sqlite::Sqlite;

    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();

    let args = TryIntoArguments::<Sqlite>::try_into_arguments(big_unsigned(i64::MAX as u64));
    let n: i64 = sqlx::query_scalar_with("SELECT ?", args.unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(n, i64::MAX);

    let err = TryIntoArguments::<Sqlite>::try_into_arguments(big_unsigned(u64::MAX));
    assert!(matches!(err, Err(sqlx::Error::Encode(_))));
}

#[cfg(feature = "sqlx-mysql")]
#[tokio::test]
async fn mysql_big_unsigned() {
    use sqlx::mysql::MySql;

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let pool = sqlx::MySqlPool::connect(&url).await.unwrap();

    let args = TryIntoArguments::<MySql>::try_into_arguments(big_unsigned(u64::MAX));
    let n: u64 = sqlx::query_scalar_with("SELECT CAST(? AS UNSIGNED)", args.unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(n, u64::MAX);
}
