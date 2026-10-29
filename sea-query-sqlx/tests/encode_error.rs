#[cfg(all(feature = "sqlx-postgres", feature = "with-bigdecimal"))]
#[test]
fn postgres_encode_error() {
    use sea_query::prelude::BigDecimal;
    use sea_query::{Value, Values};
    use sea_query_sqlx::{SqlxValues, TryIntoArguments};
    use sqlx::postgres::Postgres;

    // A scale above i16::MAX does not fit a Postgres NUMERIC, so `Arguments::add` fails.
    let d: BigDecimal = "1e-100000".parse().unwrap();
    let values = SqlxValues(Values(vec![Value::BigDecimal(Some(Box::new(d)))]));
    let err = TryIntoArguments::<Postgres>::try_into_arguments(values);
    assert!(matches!(err, Err(sqlx::Error::Encode(_))));
}
