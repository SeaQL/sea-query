#[cfg(test)]
mod tests {
    use sea_query::{InsertStatement, PostgresQueryBuilder, Query, Value, Values};
    use sea_query_attr::enum_def;

    #[allow(dead_code)]
    #[enum_def]
    pub struct Hello {
        pub name: String,
    }

    pub fn insert_greeting_sql(name: String) -> InsertStatement {
        Query::insert()
            .into_table(HelloIden::Table)
            .columns([HelloIden::Name])
            .values_panic([name.into()])
            .clone()
    }

    #[test]
    fn enum_def_used_to_build_a_parameterized_sql_and_values_tuple() {
        let test_builder = insert_greeting_sql("test_name".to_string());
        let (sql, values) = test_builder.build(PostgresQueryBuilder); // build() returns a tuple
        assert_eq!(sql, r#"INSERT INTO "hello" ("name") VALUES ($1)"#);
        assert_eq!(
            values,
            Values(vec![Value::String(Some(Box::new("test_name".to_owned())))])
        );
    }

    #[test]
    fn enum_def_used_to_build_a_sql_string() {
        let test_builder = insert_greeting_sql("test_name".to_string());
        let sql = test_builder.to_string(PostgresQueryBuilder); // returns a sql string
        assert_eq!(sql, r#"INSERT INTO "hello" ("name") VALUES ('test_name')"#);
    }
}
