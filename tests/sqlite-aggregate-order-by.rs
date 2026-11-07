use sea_query::{Expr, Func, Order, Query, SqliteQueryBuilder, tests_cfg::Char};

#[test]
fn test_group_concat_order_by() {
    let query = Query::select()
        .expr(
            Func::group_concat(Expr::col(Char::Character), ", ")
                .order_by(Expr::col(Char::Id), Order::Desc),
        )
        .from(Char::Table)
        .to_owned();

    assert_eq!(
        query.to_string(SqliteQueryBuilder),
        r#"SELECT GROUP_CONCAT("character", ', ' ORDER BY "id" DESC) FROM "character""#
    );
}
