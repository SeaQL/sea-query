use crate::*;
use std::borrow::Cow;

mod builder;
pub use builder::Builder;
mod column;
pub use column::Column;
mod exists_column;
pub use exists_column::ExistsColumn;
mod nested;
pub use nested::NestedPath;
pub(super) mod types;
pub use types::*;

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
    ///         .for_ordinality("row_number")
    ///         .column(json_table::Column::new("name", "text").path("$.name"))
    ///         .column(json_table::Column::new("age", "int").path("$.age"))
    ///         .build(),
    ///         "people"
    ///     )
    ///     .column("*")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "*" FROM JSON_TABLE('[{"name": "John", "age": 30}, {"name": "Jane", "age": 25}]', '$[*]' COLUMNS (row_number FOR ORDINALITY, name "text" PATH '$.name', age "int" PATH '$.age')) AS "people""#
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
    ///         .column(json_table::Column::new("user_id", "int").path("$.id").null_on_error())
    ///         .column(json_table::Column::new("user_name", "text").path("$.name").default_on_empty(Expr::val("Unknown")))
    ///         .error_on_error()
    ///         .build(),
    ///         "filtered_users"
    ///     )
    ///     .column("*")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "*" FROM JSON_TABLE('{"users": [{"id": 1, "name": "John"}, {"id": 2, "name": "Jane"}]}', '$.users[*] ? (@.id > $min_id)' PASSING 0 AS min_id COLUMNS (user_id "int" PATH '$.id' NULL ON ERROR, user_name "text" PATH '$.name' DEFAULT 'Unknown' ON EMPTY) ERROR ON ERROR) AS "filtered_users""#
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
    ///         .column(json_table::Column::new("user_name", "text").path("$.name"))
    ///         .nested(
    ///             json_table::NestedPath::new("$.phones[*]")
    ///                 .column(json_table::Column::new("phone", "text").path("$")),
    ///         )
    ///         .build(),
    ///         "user_phones"
    ///     )
    ///     .column("*")
    ///     .to_owned();
    ///
    /// assert_eq!(
    ///     query.to_string(PostgresQueryBuilder),
    ///     r#"SELECT "*" FROM JSON_TABLE('{"users": [{"name": "John", "phones": ["123", "456"]}, {"name": "Jane", "phones": ["789"]}]}', '$.users[*]' COLUMNS (user_name "text" PATH '$.name', NESTED PATH '$.phones[*]' COLUMNS (phone "text" PATH '$'))) AS "user_phones""#
    /// );
    /// ```
    pub fn json_table(
        context_item: impl Into<Expr>,
        path_expression: impl Into<Cow<'static, str>>,
    ) -> Builder {
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
