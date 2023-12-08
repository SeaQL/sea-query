use sea_query::{tests_cfg::Glyph, Cond, Expr, MysqlQueryBuilder, Query};

#[test]
fn test_more_parentheses() {
    let query = Query::select()
        .column(Glyph::Image)
        .from(Glyph::Table)
        .cond_where(Cond::all())
        .cond_where(Expr::val(1).eq(1))
        .cond_where(Expr::val(2).eq(2))
        .cond_where(Cond::any().add(Expr::val(3).eq(3)).add(Expr::val(4).eq(4)))
        .to_owned();

    assert_eq!(
        query.to_string(MysqlQueryBuilder),
        "SELECT `image` FROM `glyph` WHERE 1 = 1 AND 2 = 2 AND (3 = 3 OR 4 = 4)"
    );

    sea_query::options::set_prefer_more_parentheses(true);

    assert_eq!(
        query.to_string(MysqlQueryBuilder),
        "SELECT `image` FROM `glyph` WHERE (1 = 1) AND (2 = 2) AND ((3 = 3) OR (4 = 4))"
    );
}
