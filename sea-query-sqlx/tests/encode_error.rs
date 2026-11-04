#[cfg(all(feature = "sqlx-postgres", feature = "with-bigdecimal"))]
#[test]
fn postgres_encode_error() {
    use sea_query::prelude::BigDecimal;
    use sea_query::{Value, Values};
    use sea_query_sqlx::SqlxValues;
    use sqlx::postgres::PgArguments;

    // A scale above i16::MAX does not fit a Postgres NUMERIC, so `Arguments::add` fails.
    let d: BigDecimal = "1e-100000".parse().unwrap();
    let values = SqlxValues(Values(vec![Value::BigDecimal(Some(Box::new(d)))]));
    let err = PgArguments::try_from(values);
    assert!(matches!(
        err,
        Err(sea_query_sqlx::IntoArgumentsError::Encode(_))
    ));
}
