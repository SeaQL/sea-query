use super::*;
use pretty_assertions::assert_eq;
use sea_query::extension::postgres::{PgFunc, json_table as pg_json_table};

#[test]
fn json_table_basic_columns() {
    let query = Query::select()
        .from_function(
            PgFunc::json_table(
                Expr::val(r#"[{"name": "John", "age": 30}, {"name": "Jane", "age": 25}]"#),
                "$[*]",
            )
            .for_ordinality("row_number")
            .column(pg_json_table::Column::new("name", "text").path("$.name"))
            .column(pg_json_table::Column::new("age", "int").path("$.age"))
            .build(),
            "people",
        )
        .column("*")
        .to_owned();

    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "*" FROM JSON_TABLE('[{"name": "John", "age": 30}, {"name": "Jane", "age": 25}]', '$[*]' COLUMNS (row_number FOR ORDINALITY, name "text" PATH '$.name', age "int" PATH '$.age')) AS "people""#
    );
}

#[test]
fn json_table_passing_and_error_handling() {
    let query = Query::select()
        .from_function(
            PgFunc::json_table(
                Expr::val(r#"{"users": [{"id": 1, "name": "John"}, {"id": 2, "name": "Jane"}]}"#),
                "$.users[*] ? (@.id > $min_id)",
            )
            .passing(0, "min_id")
            .column(
                pg_json_table::Column::new("user_id", "int")
                    .path("$.id")
                    .null_on_error(),
            )
            .column(
                pg_json_table::Column::new("user_name", "text")
                    .path("$.name")
                    .default_on_empty(Expr::val("Unknown")),
            )
            .error_on_error()
            .build(),
            "filtered_users",
        )
        .column("*")
        .to_owned();

    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "*" FROM JSON_TABLE('{"users": [{"id": 1, "name": "John"}, {"id": 2, "name": "Jane"}]}', '$.users[*] ? (@.id > $min_id)' PASSING 0 AS min_id COLUMNS (user_id "int" PATH '$.id' NULL ON ERROR, user_name "text" PATH '$.name' DEFAULT 'Unknown' ON EMPTY) ERROR ON ERROR) AS "filtered_users""#
    );
}

#[test]
fn json_table_nested_path() {
    let query = Query::select()
        .from_function(
            PgFunc::json_table(
                Expr::val(
                    r#"{"users": [{"name": "John", "phones": ["123", "456"]}, {"name": "Jane", "phones": ["789"]}]}"#,
                ),
                "$.users[*]",
            )
            .column(pg_json_table::Column::new("user_name", "text").path("$.name"))
            .nested(
                pg_json_table::NestedPath::new("$.phones[*]")
                    .column(pg_json_table::Column::new("phone", "text").path("$")),
            )
            .build(),
            "user_phones",
        )
        .column("*")
        .to_owned();

    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "*" FROM JSON_TABLE('{"users": [{"name": "John", "phones": ["123", "456"]}, {"name": "Jane", "phones": ["789"]}]}', '$.users[*]' COLUMNS (user_name "text" PATH '$.name', NESTED PATH '$.phones[*]' COLUMNS (phone "text" PATH '$'))) AS "user_phones""#
    );
}

#[test]
fn json_table_nested_path_recursion_and_siblings() {
    let query = Query::select()
        .from_function(
            PgFunc::json_table(
                Expr::cust(
                    r#"'{"favorites":[{"movies":[{"name":"One","director":"John Doe"}],"books":[{"name":"Mystery","authors":[{"name":"Brown Dan"}]}]}]}'::json"#,
                ),
                "$.favorites[*]",
            )
            .for_ordinality("user_id")
            .nested(
                pg_json_table::NestedPath::new("$.movies[*]")
                    .for_ordinality("movie_id")
                    .column(pg_json_table::Column::new("mname", "text").path("$.name"))
                    .column(pg_json_table::Column::new("director", "text")),
            )
            .nested(
                pg_json_table::NestedPath::new("$.books[*]")
                    .for_ordinality("book_id")
                    .column(pg_json_table::Column::new("bname", "text").path("$.name"))
                    .nested(
                        pg_json_table::NestedPath::new("$.authors[*]")
                            .for_ordinality("author_id")
                            .column(
                                pg_json_table::Column::new("author_name", "text").path("$.name"),
                            ),
                    ),
            )
            .build(),
            "jt",
        )
        .column("*")
        .to_owned();

    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "*" FROM JSON_TABLE('{"favorites":[{"movies":[{"name":"One","director":"John Doe"}],"books":[{"name":"Mystery","authors":[{"name":"Brown Dan"}]}]}]}'::json, '$.favorites[*]' COLUMNS (user_id FOR ORDINALITY, NESTED PATH '$.movies[*]' COLUMNS (movie_id FOR ORDINALITY, mname "text" PATH '$.name', director "text"), NESTED PATH '$.books[*]' COLUMNS (book_id FOR ORDINALITY, bname "text" PATH '$.name', NESTED PATH '$.authors[*]' COLUMNS (author_id FOR ORDINALITY, author_name "text" PATH '$.name')))) AS "jt""#
    );
}
