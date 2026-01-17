use super::*;
use pretty_assertions::assert_eq;

#[test]
fn explain_mysql_select_with_format() {
    assert_eq!(
        ExplainStatement::new()
            .format(ExplainFormat::Json)
            .statement(
                Query::select()
                    .column(Char::Character)
                    .from(Char::Table)
                    .to_owned(),
            )
            .to_string(MysqlQueryBuilder),
        "EXPLAIN FORMAT = JSON SELECT `character` FROM `character`"
    );
}

#[test]
fn explain_mysql_analyze_tree() {
    assert_eq!(
        ExplainStatement::new()
            .analyze()
            .format(ExplainFormat::Tree)
            .statement(
                Query::select()
                    .column(Char::Character)
                    .from(Char::Table)
                    .to_owned(),
            )
            .to_string(MysqlQueryBuilder),
        "EXPLAIN ANALYZE FORMAT = TREE SELECT `character` FROM `character`"
    );
}

#[test]
fn explain_mysql_describe_table_column() {
    assert_eq!(
        ExplainStatement::table_with_column(Char::Table, Char::SizeW).to_string(MysqlQueryBuilder),
        "EXPLAIN `character` `size_w`"
    );
}

#[test]
fn explain_mysql_for_connection() {
    assert_eq!(
        ExplainStatement::new()
            .for_connection(123)
            .to_string(MysqlQueryBuilder),
        "EXPLAIN FOR CONNECTION 123"
    );
    assert_eq!(
        ExplainStatement::new()
            .format(ExplainFormat::Json)
            .into_variable("@foo")
            .for_connection(123)
            .to_string(MysqlQueryBuilder),
        "EXPLAIN FORMAT = JSON INTO @foo FOR CONNECTION 123"
    );
}

#[test]
fn explain_mysql_for_schema_database() {
    assert_eq!(
        ExplainStatement::new()
            .for_schema("s1")
            .statement(
                Query::select()
                    .column(Char::Character)
                    .from(Char::Table)
                    .to_owned(),
            )
            .to_string(MysqlQueryBuilder),
        "EXPLAIN FOR SCHEMA `s1` SELECT `character` FROM `character`"
    );
    assert_eq!(
        ExplainStatement::new()
            .for_database("db1")
            .statement(
                Query::select()
                    .column(Char::Character)
                    .from(Char::Table)
                    .to_owned(),
            )
            .to_string(MysqlQueryBuilder),
        "EXPLAIN FOR DATABASE `db1` SELECT `character` FROM `character`"
    );
}

#[test]
fn explain_mysql_into_variable() {
    assert_eq!(
        ExplainStatement::new()
            .format(ExplainFormat::Json)
            .into_variable("@plan")
            .statement(
                Query::select()
                    .column(Char::Character)
                    .from(Char::Table)
                    .to_owned(),
            )
            .to_string(MysqlQueryBuilder),
        "EXPLAIN FORMAT = JSON INTO @plan SELECT `character` FROM `character`"
    );
}
