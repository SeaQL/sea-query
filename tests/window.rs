#![cfg(any(
    feature = "backend-mysql",
    feature = "backend-postgres",
    feature = "backend-sqlite"
))]

use pretty_assertions::assert_eq;
#[cfg(feature = "audit")]
use sea_query::audit::AuditTrait;
use sea_query::*;

#[cfg(feature = "backend-sqlite")]
#[test]
fn ranking_and_frame_value_functions_render_with_a_window() {
    let window = WindowStatement::new().order_by("score", Order::Desc).take();
    let query = Query::select()
        .expr(Func::row_number().over("scores"))
        .expr(Func::rank().over("scores"))
        .expr(Func::dense_rank().over("scores"))
        .expr(Func::first_value(Expr::col("score")).over("scores"))
        .expr(Func::last_value(Expr::col("score")).over("scores"))
        .from("results")
        .window("scores", window)
        .to_owned();

    assert_eq!(
        query.to_string(SqliteQueryBuilder),
        concat!(
            r#"SELECT ROW_NUMBER() OVER "scores", RANK() OVER "scores", DENSE_RANK() OVER "scores", "#,
            r#"FIRST_VALUE("score") OVER "scores", LAST_VALUE("score") OVER "scores" FROM "results" "#,
            r#"WINDOW "scores" AS ( ORDER BY "score" DESC)"#,
        ),
    );
}

#[test]
fn monthly_change_uses_the_same_named_window_for_lag_and_lead() {
    let month = "year_month";
    let count = Expr::col("uid").count();
    let no_newer_month = Func::lag(Expr::col(month)).over("by_month").is_null();
    let older_count = Func::lead(count.clone()).over("by_month");
    let change = count.div(older_count).sub(1).mul(100);
    let mut window = WindowStatement::new();
    window.order_by(month, Order::Desc);

    let query = Query::select()
        .column(month)
        .expr_as(
            Expr::case(no_newer_month, Expr::null()).finally(change),
            "change_percent",
        )
        .from("activity")
        .group_by_col(month)
        .window("by_month", window)
        .order_by(month, Order::Desc)
        .to_owned();

    #[cfg(feature = "backend-mysql")]
    assert_eq!(
        query.to_string(MysqlQueryBuilder),
        concat!(
            "SELECT `year_month`, (CASE WHEN (LAG(`year_month`) OVER `by_month` IS NULL) ",
            "THEN NULL ELSE ((COUNT(`uid`) / LEAD(COUNT(`uid`)) OVER `by_month`) - 1) * 100 END) ",
            "AS `change_percent` FROM `activity` GROUP BY `year_month` ",
            "WINDOW `by_month` AS ( ORDER BY `year_month` DESC) ORDER BY `year_month` DESC",
        ),
    );
    #[cfg(any(feature = "backend-postgres", feature = "backend-sqlite"))]
    let expected = concat!(
        r#"SELECT "year_month", (CASE WHEN (LAG("year_month") OVER "by_month" IS NULL) "#,
        r#"THEN NULL ELSE ((COUNT("uid") / LEAD(COUNT("uid")) OVER "by_month") - 1) * 100 END) "#,
        r#"AS "change_percent" FROM "activity" GROUP BY "year_month" "#,
        r#"WINDOW "by_month" AS ( ORDER BY "year_month" DESC) ORDER BY "year_month" DESC"#,
    );
    #[cfg(feature = "backend-postgres")]
    assert_eq!(query.to_string(PostgresQueryBuilder), expected);
    #[cfg(feature = "backend-sqlite")]
    assert_eq!(query.to_string(SqliteQueryBuilder), expected);
}

#[cfg(feature = "backend-postgres")]
#[test]
fn window_bind_parameters_follow_sql_order() {
    let inline = WindowStatement::new()
        .order_by_expr(Expr::col("uid").add(10), Order::Asc)
        .take();
    let named = WindowStatement::new()
        .order_by_expr(Expr::col("uid").add(20), Order::Asc)
        .frame_between(FrameType::Rows, Frame::Preceding(1), Frame::Following(2))
        .take();
    let query = Query::select()
        .expr(Func::lag(Expr::col("uid").add(1)).over(inline))
        .expr(Func::lead(Expr::col("uid")).over("by_id"))
        .from("activity")
        .window("by_id", named)
        .order_by_expr(Expr::col("uid").add(30), Order::Asc)
        .to_owned();

    assert_eq!(
        query.build(PostgresQueryBuilder),
        (
            concat!(
                r#"SELECT LAG("uid" + $1) OVER (  ORDER BY "uid" + $2 ASC ), "#,
                r#"LEAD("uid") OVER "by_id" FROM "activity" "#,
                r#"WINDOW "by_id" AS ( ORDER BY "uid" + $3 ASC ROWS BETWEEN $4 PRECEDING AND $5 FOLLOWING) "#,
                r#"ORDER BY "uid" + $6 ASC"#,
            )
            .to_owned(),
            Values(vec![
                1.into(),
                10.into(),
                20.into(),
                1u32.into(),
                2u32.into(),
                30.into(),
            ]),
        ),
    );
}

#[cfg(feature = "backend-sqlite")]
#[test]
fn a_window_aggregate_can_be_an_arithmetic_operand() {
    let count = Func::count(Expr::col("uid"));
    let total = WindowFunctionCall::from(Func::sum(count.clone())).over(WindowStatement::new());
    let projection: SelectExpr = count
        .clone()
        .over(WindowStatement::partition_by("year_month"));
    let query = Query::select()
        .expr(projection)
        .expr_as(count.div(total), "activity_share")
        .from("activity")
        .group_by_col("year_month")
        .to_owned();

    assert_eq!(
        query.to_string(SqliteQueryBuilder),
        concat!(
            r#"SELECT COUNT("uid") OVER ( PARTITION BY "year_month" ), "#,
            r#"COUNT("uid") / SUM(COUNT("uid")) OVER (  ) AS "activity_share" "#,
            r#"FROM "activity" GROUP BY "year_month""#,
        ),
    );
}

#[cfg(feature = "audit")]
#[test]
fn audit_includes_tables_read_by_window_definitions() {
    let named_order = Query::select()
        .column("priority")
        .from("named_window_priorities")
        .to_owned();
    let inline_order = Query::select()
        .column("priority")
        .from("inline_window_priorities")
        .to_owned();

    let query = Query::select()
        .expr(Func::rank().over("by_priority"))
        .expr(
            Func::rank().over(
                WindowStatement::new()
                    .order_by_expr(
                        Expr::SubQuery(None, Box::new(inline_order.into())),
                        Order::Asc,
                    )
                    .take(),
            ),
        )
        .from("activity")
        .window(
            "by_priority",
            WindowStatement::new()
                .order_by_expr(
                    Expr::SubQuery(None, Box::new(named_order.into())),
                    Order::Asc,
                )
                .take(),
        )
        .to_owned();

    let mut tables = query.audit().unwrap().selected_tables();
    tables.sort_by_key(DynIden::inner);
    assert_eq!(
        tables,
        [
            "activity".into_iden(),
            "inline_window_priorities".into_iden(),
            "named_window_priorities".into_iden(),
        ],
    );
}

#[cfg(feature = "audit")]
#[test]
fn nested_window_cte_does_not_hide_an_outer_table() {
    let cte = CommonTableExpression::new()
        .table_name("activity")
        .query(Query::select().expr_as(Expr::val(99), "uid").to_owned())
        .to_owned();
    let subquery = Query::select()
        .column("uid")
        .from("activity")
        .with_cte(WithClause::new().cte(cte).to_owned())
        .to_owned();
    let window = WindowStatement::new()
        .order_by_expr(Expr::SubQuery(None, Box::new(subquery.into())), Order::Asc)
        .take();
    let query = Query::select()
        .from("activity")
        .expr(Func::rank().over("by_priority"))
        .window("by_priority", window.clone())
        .to_owned();

    assert_eq!(
        query.audit().unwrap().selected_tables(),
        ["activity".into_iden()],
    );

    let query = Query::select()
        .expr(Func::rank().over(window.clone()))
        .from("other_activity")
        .to_owned();

    assert_eq!(
        query.audit().unwrap().selected_tables(),
        ["other_activity".into_iden()],
    );

    let query = Query::select()
        .column("uid")
        .from("activity")
        .order_by_expr(Func::rank().over(window), Order::Asc)
        .to_owned();

    assert_eq!(
        query.audit().unwrap().selected_tables(),
        ["activity".into_iden()],
    );
}

#[cfg(feature = "audit")]
#[test]
fn audit_includes_tables_read_by_a_window_aggregate_filter() {
    let priorities = Query::select().column("uid").from("priorities").to_owned();
    let total = Func::sum(Expr::col("uid")).filter(Expr::col("uid").in_subquery(priorities));
    let query = Query::select()
        .expr(WindowFunctionCall::from(total).over(WindowStatement::new()))
        .from("activity")
        .to_owned();

    let mut tables = query.audit().unwrap().selected_tables();
    tables.sort_by_key(DynIden::inner);
    assert_eq!(tables, ["activity".into_iden(), "priorities".into_iden()]);
}

#[cfg(feature = "audit")]
#[test]
fn audit_includes_tables_read_through_select_clauses() {
    let subquery = |table| Query::select().column("uid").from(table).to_owned();
    let query = Query::select()
        .column(("activity", "uid"))
        .from("activity")
        .left_join(
            "details",
            Expr::col(("activity", "uid")).in_subquery(subquery("join_priorities")),
        )
        .group_by_col(("activity", "uid"))
        .add_group_by([Expr::SubQuery(
            None,
            Box::new(subquery("group_priorities").into()),
        )])
        .and_having(Expr::col(("activity", "uid")).in_subquery(subquery("having_priorities")))
        .to_owned();

    let mut tables = query.audit().unwrap().selected_tables();
    tables.sort_by_key(DynIden::inner);
    assert_eq!(
        tables,
        [
            "activity".into_iden(),
            "details".into_iden(),
            "group_priorities".into_iden(),
            "having_priorities".into_iden(),
            "join_priorities".into_iden(),
        ],
    );
}
