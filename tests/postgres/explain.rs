use super::*;
use pretty_assertions::assert_eq;

#[test]
fn explain_postgres_select_with_options() {
    assert_eq!(
        ExplainStatement::new()
            .analyze()
            .verbose(false)
            .costs(true)
            .settings(false)
            .generic_plan(true)
            .buffers(true)
            .serialize_text()
            .wal(true)
            .timing(false)
            .summary(true)
            .memory(true)
            .format(ExplainFormat::Json)
            .statement(
                Query::select()
                    .column(Char::Character)
                    .from(Char::Table)
                    .to_owned(),
            )
            .to_string(PostgresQueryBuilder),
        r#"EXPLAIN (ANALYZE, VERBOSE 0, COSTS, SETTINGS 0, GENERIC_PLAN, BUFFERS, SERIALIZE TEXT, WAL, TIMING 0, SUMMARY, MEMORY, FORMAT JSON) SELECT "character" FROM "character""#
    );
}

#[test]
fn explain_postgres_serialize_text() {
    assert_eq!(
        ExplainStatement::new()
            .serialize_text()
            .statement(
                Query::select()
                    .column(Char::Character)
                    .from(Char::Table)
                    .to_owned(),
            )
            .to_string(PostgresQueryBuilder),
        r#"EXPLAIN (SERIALIZE TEXT) SELECT "character" FROM "character""#
    );
}

#[test]
fn explain_postgres_serialize_binary() {
    assert_eq!(
        ExplainStatement::new()
            .serialize_binary()
            .statement(
                Query::select()
                    .column(Char::Character)
                    .from(Char::Table)
                    .to_owned(),
            )
            .to_string(PostgresQueryBuilder),
        r#"EXPLAIN (SERIALIZE BINARY) SELECT "character" FROM "character""#
    );
}

#[test]
fn explain_postgres_serialize_none() {
    assert_eq!(
        ExplainStatement::new()
            .serialize_none()
            .statement(
                Query::select()
                    .column(Char::Character)
                    .from(Char::Table)
                    .to_owned(),
            )
            .to_string(PostgresQueryBuilder),
        r#"EXPLAIN (SERIALIZE NONE) SELECT "character" FROM "character""#
    );
}
