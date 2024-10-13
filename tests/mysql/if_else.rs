use super::*;
use pretty_assertions::assert_eq;

#[rustfmt::skip]
#[test]
fn if_without_else() {
    let query = Query::select().column(Asterisk).from(Glyph::Table).take();
    let then = SimpleExpr::SubQuery(None, Box::new(query.into_sub_query_statement()));
    let if_statement = IfElseStatement::new(
        Expr::col(Glyph::Id).eq(1),
        then,
        None
    );
    assert_eq!(
        if_statement.to_string(MysqlQueryBuilder),
        [
            "IF `id` = 1 THEN",
            "(SELECT * FROM `glyph`)",
            "END IF"
        ].join("\n")
    )
}

#[rustfmt::skip]
#[test]
fn if_with_else() {
    let query = Query::select().column(Asterisk).from(Glyph::Table).take();
    let then = SimpleExpr::SubQuery(None, Box::new(query.into_sub_query_statement()));
    let if_statement = IfElseStatement::new(
        Expr::col(Glyph::Id).eq(1),
        then,
        Some(Expr::val("23").into()),
    );
    assert_eq!(
        if_statement.to_string(MysqlQueryBuilder),
        [
            "IF `id` = 1 THEN",
            "(SELECT * FROM `glyph`)",
            "ELSE",
            "'23'",
            "END IF"
        ]
        .join("\n")
    )
}
