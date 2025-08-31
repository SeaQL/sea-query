pub use builder::Builder;
pub use column::ColumnBuilder;
pub use exists_column::ExistsColumnBuilder;
pub use nested::NestedPathBuilder;
pub use types::*;

use crate::*;
use std::borrow::Cow;

pub mod builder;
pub mod column;
pub mod exists_column;
pub mod nested;
pub mod types;

impl PgFunc {
    /// Create a `JSON_TABLE` function builder. Postgres only.
    ///
    /// # Examples
    ///
    /// Basic usage with simple columns:
    /// ```
    /// use sea_query::extension::postgres::*;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from_function(
    ///         PgFunc::json_table(
    ///             Expr::val(r#"[{"name": "John", "age": 30}, {"name": "Jane", "age": 25}]"#),
    ///             "$[*]"
    ///         )
    ///         .ordinality_column("row_number")
    ///         .column("name", "text").path("$.name").build_column()
    ///         .column("age", "int").path("$.age").build_column()
    ///         .build(),
    ///         "people"
    ///     )
    ///     .column("*")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "*" FROM JSON_TABLE(E'[{\"name\": \"John\", \"age\": 30}, {\"name\": \"Jane\", \"age\": 25}]', '$[*]' COLUMNS (row_number FOR ORDINALITY, name "text" PATH '$.name', age "int" PATH '$.age')) AS "people""#
    /// );
    /// ```
    ///
    /// With PASSING parameters and error handling:
    /// ```
    /// use sea_query::extension::postgres::*;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from_function(
    ///         PgFunc::json_table(
    ///             Expr::val(r#"{"users": [{"id": 1, "name": "John"}, {"id": 2, "name": "Jane"}]}"#),
    ///             "$.users[*] ? (@.id > $min_id)"
    ///         )
    ///         .passing(0, "min_id")
    ///         .column("user_id", "int").path("$.id").null_on_error().build_column()
    ///         .column("user_name", "text").path("$.name").default_on_empty(Expr::val("Unknown")).build_column()
    ///         .error_on_error()
    ///         .build(),
    ///         "filtered_users"
    ///     )
    ///     .column("*")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "*" FROM JSON_TABLE(E'{\"users\": [{\"id\": 1, \"name\": \"John\"}, {\"id\": 2, \"name\": \"Jane\"}]}', '$.users[*] ? (@.id > $min_id)' PASSING 0 AS min_id COLUMNS (user_id "int" PATH '$.id' NULL ON ERROR, user_name "text" PATH '$.name' DEFAULT 'Unknown' ON EMPTY) ERROR ON ERROR) AS "filtered_users""#
    /// );
    /// ```
    ///
    /// With NESTED PATH:
    /// ```
    /// use sea_query::extension::postgres::*;
    /// use sea_query::{tests_cfg::*, *};
    ///
    /// let query = Query::select()
    ///     .from_function(
    ///         PgFunc::json_table(
    ///             Expr::val(r#"{"users": [{"name": "John", "phones": ["123", "456"]}, {"name": "Jane", "phones": ["789"]}]}"#),
    ///             "$.users[*]"
    ///         )
    ///         .column("user_name", "text").path("$.name").build_column()
    ///         .nested("$.phones[*]")
    ///             .column("phone", "text").path("$").build_column()
    ///             .build_nested()
    ///         .build(),
    ///         "user_phones"
    ///     )
    ///     .column("*")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "*" FROM JSON_TABLE(E'{\"users\": [{\"name\": \"John\", \"phones\": [\"123\", \"456\"]}, {\"name\": \"Jane\", \"phones\": [\"789\"]}]}', '$.users[*]' COLUMNS (user_name "text" PATH '$.name', NESTED '$.phones[*]' COLUMNS (phone "text" PATH '$'))) AS "user_phones""#
    /// );
    /// ```
    pub fn json_table<T, P>(context_item: T, path_expression: P) -> Builder
    where
        T: Into<Expr>,
        P: Into<Cow<'static, str>>,
    {
        Builder {
            context_item: context_item.into(),
            path_expression: path_expression.into(),
            as_json_path_name: None,
            passing: Vec::new(),
            columns: Vec::new(),
            on_error: None,
        }
    }
}
