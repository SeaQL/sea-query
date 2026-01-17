use super::*;
use pretty_assertions::assert_eq;

#[test]
fn explain_sqlite_select() {
    assert_eq!(
        ExplainStatement::new()
            .statement(
                Query::select()
                    .column(Char::Character)
                    .from(Char::Table)
                    .to_owned(),
            )
            .to_string(SqliteQueryBuilder),
        r#"EXPLAIN SELECT "character" FROM "character""#
    );
}

#[test]
fn explain_sqlite_query_plan() {
    assert_eq!(
        ExplainStatement::new()
            .query_plan()
            .statement(
                Query::select()
                    .column(Char::Character)
                    .from(Char::Table)
                    .to_owned(),
            )
            .to_string(SqliteQueryBuilder),
        r#"EXPLAIN QUERY PLAN SELECT "character" FROM "character""#
    );
}
