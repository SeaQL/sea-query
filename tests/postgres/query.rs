use super::*;
use pretty_assertions::assert_eq;
use sea_query::{audit::AuditTrait, extension::postgres::PgBinOper};

#[test]
fn select_1() {
    assert_eq!(
        Query::select()
            .columns([Char::Character, Char::SizeW, Char::SizeH])
            .from(Char::Table)
            .limit(10)
            .offset(100)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character", "size_w", "size_h" FROM "character" LIMIT 10 OFFSET 100"#
    );
}

#[test]
fn select_2() {
    assert_eq!(
        Query::select()
            .columns([Char::Character, Char::SizeW, Char::SizeH])
            .from(Char::Table)
            .and_where(Expr::col(Char::SizeW).eq(3))
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 3"#
    );
}

#[test]
fn select_3() {
    let query = Query::select()
        .columns([Char::Character, Char::SizeW, Char::SizeH])
        .from(Char::Table)
        .and_where(Expr::col(Char::SizeW).eq(3))
        .and_where(Expr::col(Char::SizeH).eq(4))
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 3 AND "size_h" = 4"#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Char::Table.into_iden()]
    );
}

#[test]
fn select_4() {
    let query = Query::select()
        .columns([Glyph::Aspect])
        .from_subquery(
            Query::select()
                .columns([Glyph::Image, Glyph::Aspect])
                .from(Glyph::Table)
                .take(),
            "subglyph",
        )
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "aspect" FROM (SELECT "image", "aspect" FROM "glyph") AS "subglyph""#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
fn select_5() {
    let query = Query::select()
        .column((Glyph::Table, Glyph::Image))
        .from(Glyph::Table)
        .and_where(Expr::col((Glyph::Table, Glyph::Aspect)).is_in([3, 4]))
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "glyph"."image" FROM "glyph" WHERE "glyph"."aspect" IN (3, 4)"#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
fn select_6() {
    let query = Query::select()
        .columns([Glyph::Aspect])
        .exprs([Expr::col(Glyph::Image).max()])
        .from(Glyph::Table)
        .group_by_columns([Glyph::Aspect])
        .and_having(Expr::col(Glyph::Aspect).gt(2))
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "aspect", MAX("image") FROM "glyph" GROUP BY "aspect" HAVING "aspect" > 2"#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
fn select_7() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::col(Glyph::Aspect).if_null(0).gt(2))
            .to_string(PostgresQueryBuilder),
        r#"SELECT "aspect" FROM "glyph" WHERE COALESCE("aspect", 0) > 2"#
    );
}

#[test]
fn select_8() {
    let query = Query::select()
        .columns([Char::Character])
        .from(Char::Table)
        .left_join(
            Font::Table,
            Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
        )
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id""#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Char::Table.into_iden(), Font::Table.into_iden()]
    );
}

#[test]
fn select_9() {
    let query = Query::select()
        .columns([Char::Character])
        .from(Char::Table)
        .left_join(
            Font::Table,
            Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
        )
        .inner_join(
            Glyph::Table,
            Expr::col((Char::Table, Char::Character)).equals((Glyph::Table, Glyph::Image)),
        )
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" INNER JOIN "glyph" ON "character"."character" = "glyph"."image""#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [
            Char::Table.into_iden(),
            Font::Table.into_iden(),
            Glyph::Table.into_iden(),
        ]
    );
}

#[test]
fn select_10() {
    let query = Query::select()
        .columns([Char::Character])
        .from(Char::Table)
        .left_join(
            Font::Table,
            Expr::col((Char::Table, Char::FontId))
                .equals((Font::Table, Font::Id))
                .and(Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id))),
        )
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" LEFT JOIN "font" ON "character"."font_id" = "font"."id" AND "character"."font_id" = "font"."id""#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Char::Table.into_iden(), Font::Table.into_iden()]
    );
}

#[test]
fn select_11() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::col(Glyph::Aspect).if_null(0).gt(2))
            .order_by(Glyph::Image, Order::Desc)
            .order_by((Glyph::Table, Glyph::Aspect), Order::Asc)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "aspect" FROM "glyph" WHERE COALESCE("aspect", 0) > 2 ORDER BY "image" DESC, "glyph"."aspect" ASC"#
    );
}

#[test]
fn select_12() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::col(Glyph::Aspect).if_null(0).gt(2))
            .order_by_columns([(Glyph::Id, Order::Asc), (Glyph::Aspect, Order::Desc)])
            .to_string(PostgresQueryBuilder),
        r#"SELECT "aspect" FROM "glyph" WHERE COALESCE("aspect", 0) > 2 ORDER BY "id" ASC, "aspect" DESC"#
    );
}

#[test]
fn select_13() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::col(Glyph::Aspect).if_null(0).gt(2))
            .order_by_columns([
                ((Glyph::Table, Glyph::Id), Order::Asc),
                ((Glyph::Table, Glyph::Aspect), Order::Desc),
            ])
            .to_string(PostgresQueryBuilder),
        r#"SELECT "aspect" FROM "glyph" WHERE COALESCE("aspect", 0) > 2 ORDER BY "glyph"."id" ASC, "glyph"."aspect" DESC"#
    );
}

#[test]
fn select_14() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Id, Glyph::Aspect])
            .expr(Expr::col(Glyph::Image).max())
            .from(Glyph::Table)
            .group_by_columns([(Glyph::Table, Glyph::Id), (Glyph::Table, Glyph::Aspect)])
            .and_having(Expr::col(Glyph::Aspect).gt(2))
            .to_string(PostgresQueryBuilder),
        r#"SELECT "id", "aspect", MAX("image") FROM "glyph" GROUP BY "glyph"."id", "glyph"."aspect" HAVING "aspect" > 2"#
    );
}

#[test]
fn select_15() {
    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .and_where(Expr::col(Char::FontId).is_null())
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "font_id" IS NULL"#
    );
}

#[test]
fn select_16() {
    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .and_where(Expr::col(Char::FontId).is_null())
            .and_where(Expr::col(Char::Character).is_not_null())
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "font_id" IS NULL AND "character" IS NOT NULL"#
    );
}

#[test]
fn select_17() {
    assert_eq!(
        Query::select()
            .columns([(Glyph::Table, Glyph::Image)])
            .from(Glyph::Table)
            .and_where(Expr::col((Glyph::Table, Glyph::Aspect)).between(3, 5))
            .to_string(PostgresQueryBuilder),
        r#"SELECT "glyph"."image" FROM "glyph" WHERE "glyph"."aspect" BETWEEN 3 AND 5"#
    );
}

#[test]
fn select_18() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::col(Glyph::Aspect).between(3, 5))
            .and_where(Expr::col(Glyph::Aspect).not_between(8, 10))
            .to_string(PostgresQueryBuilder),
        r#"SELECT "aspect" FROM "glyph" WHERE ("aspect" BETWEEN 3 AND 5) AND ("aspect" NOT BETWEEN 8 AND 10)"#
    );
}

#[test]
fn select_19() {
    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .and_where(Expr::col(Char::Character).eq("A"))
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "character" = 'A'"#
    );
}

#[test]
fn select_20() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(Expr::col(Char::Character).like("A"))
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "character" LIKE 'A'"#
    );
}

#[test]
fn select_21() {
    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .cond_where(any![
                Expr::col(Char::Character).like("A%"),
                Expr::col(Char::Character).like("%B"),
                Expr::col(Char::Character).like("%C%"),
            ])
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "character" LIKE 'A%' OR "character" LIKE '%B' OR "character" LIKE '%C%'"#
    );
}

#[test]
fn select_22() {
    let query = Query::select()
        .column(Char::Character)
        .from(Char::Table)
        .cond_where(
            Cond::all()
                .add(
                    Cond::any().add(Expr::col(Char::Character).like("C")).add(
                        Expr::col(Char::Character)
                            .like("D")
                            .and(Expr::col(Char::Character).like("E")),
                    ),
                )
                .add(
                    Expr::col(Char::Character)
                        .like("F")
                        .or(Expr::col(Char::Character).like("G")),
                ),
        )
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE ("character" LIKE 'C' OR ("character" LIKE 'D' AND "character" LIKE 'E')) AND ("character" LIKE 'F' OR "character" LIKE 'G')"#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Char::Table.into_iden()]
    );
}

#[test]
fn select_23() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where_option(None)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character""#
    );
}

#[test]
fn select_24() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .conditions(
                true,
                |x| {
                    x.and_where(Expr::col(Char::FontId).eq(5));
                },
                |_| ()
            )
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "font_id" = 5"#
    );
}

#[test]
fn select_25() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(
                Expr::col(Char::SizeW)
                    .mul(2)
                    .eq(Expr::col(Char::SizeH).div(2))
            )
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "size_w" * 2 = "size_h" / 2"#
    );
}

#[test]
fn select_26() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(
                Expr::col(Char::SizeW)
                    .add(1)
                    .mul(2)
                    .eq(Expr::col(Char::SizeH).div(2).sub(1))
            )
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE ("size_w" + 1) * 2 = ("size_h" / 2) - 1"#
    );
}

#[test]
fn select_27() {
    assert_eq!(
        Query::select()
            .columns([Char::Character, Char::SizeW, Char::SizeH])
            .from(Char::Table)
            .and_where(Expr::col(Char::SizeW).eq(3))
            .and_where(Expr::col(Char::SizeH).eq(4))
            .and_where(Expr::col(Char::SizeH).eq(5))
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 3 AND "size_h" = 4 AND "size_h" = 5"#
    );
}

#[test]
fn select_28() {
    assert_eq!(
        Query::select()
            .columns([Char::Character, Char::SizeW, Char::SizeH])
            .from(Char::Table)
            .cond_where(any![
                Expr::col(Char::SizeW).eq(3),
                Expr::col(Char::SizeH).eq(4),
                Expr::col(Char::SizeH).eq(5),
            ])
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE "size_w" = 3 OR "size_h" = 4 OR "size_h" = 5"#
    );
}

#[test]
fn select_30() {
    assert_eq!(
        Query::select()
            .columns([Char::Character, Char::SizeW, Char::SizeH])
            .from(Char::Table)
            .and_where(
                Expr::col(Char::SizeW)
                    .mul(2)
                    .add(Expr::col(Char::SizeH).div(3))
                    .eq(4)
            )
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character", "size_w", "size_h" FROM "character" WHERE ("size_w" * 2) + ("size_h" / 3) = 4"#
    );
}

#[test]
fn select_31() {
    let query = Query::select()
        .expr((1..10_i32).fold(Expr::value(0), |expr, i| expr.add(i)))
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9"#
    );
    assert_eq!(query.audit_unwrap().selected_tables(), []);
}

#[test]
fn select_32() {
    let query = Query::select()
        .expr_as(Expr::col(Char::Character), "C")
        .from(Char::Table)
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "character" AS "C" FROM "character""#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Char::Table.into_iden()]
    );
}

#[test]
fn select_33a() {
    let query = Query::select()
        .column(Glyph::Image)
        .from(Glyph::Table)
        .and_where(
            Expr::col(Glyph::Aspect)
                .in_subquery(Query::select().expr(Expr::cust("3 + 2 * 2")).take()),
        )
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "image" FROM "glyph" WHERE "aspect" IN (SELECT 3 + 2 * 2)"#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
fn select_33b() {
    let query = Query::select()
        .column(Glyph::Image)
        .from(Glyph::Table)
        .and_where(
            Expr::col(Glyph::Aspect).in_subquery(
                Query::select()
                    .column(Font::Variant)
                    .from(Font::Table)
                    .take(),
            ),
        )
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "image" FROM "glyph" WHERE "aspect" IN (SELECT "variant" FROM "font")"#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Glyph::Table.into_iden(), Font::Table.into_iden()]
    );
}

#[test]
fn select_34() {
    assert_eq!(
        Query::select()
            .column(Glyph::Aspect)
            .expr(Expr::col(Glyph::Image).max())
            .from(Glyph::Table)
            .group_by_columns([Glyph::Aspect])
            .cond_having(any![
                Expr::col(Glyph::Aspect)
                    .gt(2)
                    .or(Expr::col(Glyph::Aspect).lt(8)),
                Expr::col(Glyph::Aspect)
                    .gt(12)
                    .and(Expr::col(Glyph::Aspect).lt(18)),
                Expr::col(Glyph::Aspect).gt(32),
            ])
            .to_string(PostgresQueryBuilder),
        [
            r#"SELECT "aspect", MAX("image") FROM "glyph" GROUP BY "aspect""#,
            r#"HAVING "aspect" > 2 OR "aspect" < 8"#,
            r#"OR ("aspect" > 12 AND "aspect" < 18)"#,
            r#"OR "aspect" > 32"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_35() {
    let (statement, values) = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .and_where(Expr::col(Glyph::Aspect).is_null())
        .build(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" IS NULL"#
    );
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_36() {
    let (statement, values) = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(Cond::any().add(Expr::col(Glyph::Aspect).is_null()))
        .build(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" IS NULL"#
    );
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_37() {
    let (statement, values) = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(Cond::any().add(Cond::all()).add(Cond::any()))
        .build(PostgresQueryBuilder);

    assert_eq!(statement, r#"SELECT "id" FROM "glyph" WHERE TRUE OR FALSE"#);
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_37a() {
    let (statement, values) = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::all()
                .add(Cond::all().not())
                .add(Cond::any().not())
                .not(),
        )
        .build(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE NOT ((NOT TRUE) AND (NOT FALSE))"#
    );
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_38() {
    let (statement, values) = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::any()
                .add(Expr::col(Glyph::Aspect).is_null())
                .add(Expr::col(Glyph::Aspect).is_not_null()),
        )
        .build(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" IS NULL OR "aspect" IS NOT NULL"#
    );
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_39() {
    let (statement, values) = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::all()
                .add(Expr::col(Glyph::Aspect).is_null())
                .add(Expr::col(Glyph::Aspect).is_not_null()),
        )
        .build(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" IS NULL AND "aspect" IS NOT NULL"#
    );
    assert_eq!(values.0, vec![]);
}

#[test]
fn select_40() {
    let statement = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(any![
            Expr::col(Glyph::Aspect).is_null(),
            all![
                Expr::col(Glyph::Aspect).is_not_null(),
                Expr::col(Glyph::Aspect).lt(8)
            ]
        ])
        .to_string(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" IS NULL OR ("aspect" IS NOT NULL AND "aspect" < 8)"#
    );
}

#[test]
fn select_41() {
    let query = Query::select()
        .columns([Glyph::Aspect])
        .exprs([Expr::col(Glyph::Image).max()])
        .from(Glyph::Table)
        .group_by_columns([Glyph::Aspect])
        .cond_having(any![Expr::col(Glyph::Aspect).gt(2)])
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "aspect", MAX("image") FROM "glyph" GROUP BY "aspect" HAVING "aspect" > 2"#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
fn select_42() {
    let statement = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::all()
                .add_option(Some(Expr::col(Glyph::Aspect).lt(8)))
                .add(Expr::col(Glyph::Aspect).is_not_null()),
        )
        .to_string(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE "aspect" < 8 AND "aspect" IS NOT NULL"#
    );
}

#[test]
fn select_43() {
    let statement = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(Cond::all().add_option::<Expr>(None))
        .to_string(PostgresQueryBuilder);

    assert_eq!(statement, r#"SELECT "id" FROM "glyph" WHERE TRUE"#);
}

#[test]
fn select_44() {
    let statement = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::any()
                .not()
                .add_option(Some(Expr::col(Glyph::Aspect).lt(8))),
        )
        .to_string(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE NOT "aspect" < 8"#
    );
}

#[test]
fn select_45() {
    let statement = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::any()
                .not()
                .add_option(Some(Expr::col(Glyph::Aspect).lt(8)))
                .add(Expr::col(Glyph::Aspect).is_not_null()),
        )
        .to_string(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE NOT ("aspect" < 8 OR "aspect" IS NOT NULL)"#
    );
}

#[test]
fn select_46() {
    let statement = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::all()
                .not()
                .add_option(Some(Expr::col(Glyph::Aspect).lt(8))),
        )
        .to_string(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE NOT "aspect" < 8"#
    );
}

#[test]
fn select_47() {
    let statement = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::all()
                .not()
                .add_option(Some(Expr::col(Glyph::Aspect).lt(8)))
                .add(Expr::col(Glyph::Aspect).is_not_null()),
        )
        .to_string(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE NOT ("aspect" < 8 AND "aspect" IS NOT NULL)"#
    );
}

#[test]
fn select_48() {
    let statement = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::all().add_option(Some(
                Expr::tuple([Expr::col(Glyph::Aspect).into(), Expr::value(100)])
                    .lt(Expr::tuple([Expr::value(8), Expr::value(100)])),
            )),
        )
        .to_string(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE ("aspect", 100) < (8, 100)"#
    );
}

#[test]
fn select_48a() {
    let statement = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(
            Cond::all().add_option(Some(
                Expr::tuple([
                    Expr::col(Glyph::Aspect).into(),
                    Expr::value(String::from("100")),
                ])
                .in_tuples([(8, String::from("100"))]),
            )),
        )
        .to_string(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "glyph" WHERE ("aspect", '100') IN ((8, '100'))"#
    );
}

#[test]
fn select_49() {
    let statement = Query::select()
        .column(Asterisk)
        .from(Char::Table)
        .to_string(PostgresQueryBuilder);

    assert_eq!(statement, r#"SELECT * FROM "character""#);
}

#[test]
fn select_50() {
    let statement = Query::select()
        .column((Char::Table, Asterisk))
        .column((Font::Table, Font::Name))
        .from(Char::Table)
        .inner_join(
            Font::Table,
            Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id)),
        )
        .to_string(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "character".*, "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id""#
    )
}

#[test]
fn select_51() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::col(Glyph::Aspect).if_null(0).gt(2))
            .order_by_with_nulls(Glyph::Image, Order::Desc, NullOrdering::First)
            .order_by_with_nulls(
                (Glyph::Table, Glyph::Aspect),
                Order::Asc,
                NullOrdering::Last
            )
            .to_string(PostgresQueryBuilder),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE COALESCE("aspect", 0) > 2"#,
            r#"ORDER BY "image" DESC NULLS FIRST,"#,
            r#""glyph"."aspect" ASC NULLS LAST"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_52() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::col(Glyph::Aspect).if_null(0).gt(2))
            .order_by_columns_with_nulls([
                (Glyph::Id, Order::Asc, NullOrdering::First),
                (Glyph::Aspect, Order::Desc, NullOrdering::Last),
            ])
            .to_string(PostgresQueryBuilder),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE COALESCE("aspect", 0) > 2"#,
            r#"ORDER BY "id" ASC NULLS FIRST,"#,
            r#""aspect" DESC NULLS LAST"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_53() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::col(Glyph::Aspect).if_null(0).gt(2))
            .order_by_columns_with_nulls([
                ((Glyph::Table, Glyph::Id), Order::Asc, NullOrdering::First),
                (
                    (Glyph::Table, Glyph::Aspect),
                    Order::Desc,
                    NullOrdering::Last
                ),
            ])
            .to_string(PostgresQueryBuilder),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE COALESCE("aspect", 0) > 2"#,
            r#"ORDER BY "glyph"."id" ASC NULLS FIRST,"#,
            r#""glyph"."aspect" DESC NULLS LAST"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_54() {
    assert_eq!(
        Query::select()
            .distinct_on([Glyph::Aspect])
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::col(Glyph::Aspect).if_null(0).gt(2))
            .order_by_columns_with_nulls([
                ((Glyph::Table, Glyph::Id), Order::Asc, NullOrdering::First),
                (
                    (Glyph::Table, Glyph::Aspect),
                    Order::Desc,
                    NullOrdering::Last
                ),
            ])
            .to_string(PostgresQueryBuilder),
        [
            r#"SELECT DISTINCT ON ("aspect") "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE COALESCE("aspect", 0) > 2"#,
            r#"ORDER BY "glyph"."id" ASC NULLS FIRST,"#,
            r#""glyph"."aspect" DESC NULLS LAST"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_55() {
    let query = Query::select()
        .column(Asterisk)
        .from(Char::Table)
        .from(Font::Table)
        .and_where(Expr::col((Font::Table, Font::Id)).equals((Char::Table, Char::FontId)))
        .take();

    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT * FROM "character", "font" WHERE "font"."id" = "character"."font_id""#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Char::Table.into_iden(), Font::Table.into_iden()]
    );
}

#[test]
fn select_56() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::col(Glyph::Aspect).if_null(0).gt(2))
            .order_by(
                Glyph::Id,
                Order::Field(Values(vec![
                    Value::Int(Some(4)),
                    Value::Int(Some(5)),
                    Value::Int(Some(1)),
                    Value::Int(Some(3))
                ]))
            )
            .order_by((Glyph::Table, Glyph::Aspect), Order::Asc)
            .to_string(PostgresQueryBuilder),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE COALESCE("aspect", 0) > 2"#,
            r#"ORDER BY CASE"#,
            r#"WHEN "id"=4 THEN 0"#,
            r#"WHEN "id"=5 THEN 1"#,
            r#"WHEN "id"=1 THEN 2"#,
            r#"WHEN "id"=3 THEN 3"#,
            r#"ELSE 4 END,"#,
            r#""glyph"."aspect" ASC"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_57() {
    assert_eq!(
        Query::select()
            .columns([Glyph::Aspect])
            .from(Glyph::Table)
            .and_where(Expr::col(Glyph::Aspect).if_null(0).gt(2))
            .order_by((Glyph::Table, Glyph::Aspect), Order::Asc)
            .order_by(
                Glyph::Id,
                Order::Field(Values(vec![
                    Value::Int(Some(4)),
                    Value::Int(Some(5)),
                    Value::Int(Some(1)),
                    Value::Int(Some(3))
                ]))
            )
            .to_string(PostgresQueryBuilder),
        [
            r#"SELECT "aspect""#,
            r#"FROM "glyph""#,
            r#"WHERE COALESCE("aspect", 0) > 2"#,
            r#"ORDER BY "glyph"."aspect" ASC,"#,
            r#"CASE WHEN "id"=4 THEN 0"#,
            r#"WHEN "id"=5 THEN 1"#,
            r#"WHEN "id"=1 THEN 2"#,
            r#"WHEN "id"=3 THEN 3"#,
            r#"ELSE 4 END"#,
        ]
        .join(" ")
    );
}

#[test]
fn select_58() {
    let select = SelectStatement::new()
        .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
        .from(Glyph::Table)
        .to_owned();
    let cte = CommonTableExpression::new()
        .query(select)
        .table_name("cte")
        .to_owned();
    let with_clause = WithClause::new().cte(cte).to_owned();
    let select = SelectStatement::new()
        .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
        .from("cte")
        .to_owned();
    let query = select.with(with_clause);
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        [
            r#"WITH "cte" AS"#,
            r#"(SELECT "id", "image", "aspect""#,
            r#"FROM "glyph")"#,
            r#"SELECT "id", "image", "aspect" FROM "cte""#,
        ]
        .join(" ")
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
fn select_59() {
    let query = Query::select()
        .expr_as(
            CaseStatement::new()
                .case(Expr::col((Glyph::Table, Glyph::Aspect)).gt(0), "positive")
                .case(Expr::col((Glyph::Table, Glyph::Aspect)).lt(0), "negative")
                .finally("zero"),
            "polarity",
        )
        .from(Glyph::Table)
        .to_owned();

    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT (CASE WHEN ("glyph"."aspect" > 0) THEN 'positive' WHEN ("glyph"."aspect" < 0) THEN 'negative' ELSE 'zero' END) AS "polarity" FROM "glyph""#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
fn select_60() {
    let (cust_query, cust_values) = Query::select()
        .column(Character::Id)
        .from(Character::Table)
        .and_where(Expr::col(Character::FontSize).eq(3))
        .build(PostgresQueryBuilder);

    let (statement, values) = Query::select()
        .expr(Expr::cust_with_values(&cust_query[7..], cust_values.0))
        .limit(5)
        .build(PostgresQueryBuilder);

    assert_eq!(
        statement,
        r#"SELECT "id" FROM "character" WHERE "font_size" = $1 LIMIT $2"#
    );
    assert_eq!(values, Values(vec![3i32.into(), 5u64.into()]));
}

#[test]
fn select_61() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(Expr::col(Char::Character).like(LikeExpr::new("A").escape('\\')))
            .build(PostgresQueryBuilder),
        (
            r#"SELECT "character" FROM "character" WHERE "character" LIKE $1 ESCAPE E'\\'"#
                .to_owned(),
            Values(vec!["A".into()])
        )
    );
}

#[test]
fn select_62() {
    let select = SelectStatement::new()
        .column(Asterisk)
        .from_values([(1i32, "hello"), (2, "world")], "x")
        .to_owned();
    let cte = CommonTableExpression::new()
        .query(select)
        .table_name("cte")
        .to_owned();
    let with_clause = WithClause::new().cte(cte).to_owned();
    let select = SelectStatement::new()
        .columns(["column1", "column2"])
        .from("cte")
        .to_owned();
    let query = select.with(with_clause);
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        [
            r#"WITH "cte" AS"#,
            r#"(SELECT * FROM (VALUES (1, 'hello'), (2, 'world')) AS "x")"#,
            r#"SELECT "column1", "column2""#,
            r#"FROM "cte""#,
        ]
        .join(" ")
    );
    assert_eq!(query.audit_unwrap().selected_tables(), []);
}

#[test]
fn select_63() {
    let select = SelectStatement::new()
        .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
        .from(Glyph::Table)
        .to_owned();
    let cte = CommonTableExpression::new()
        .query(select)
        .table_name("cte")
        .to_owned();
    let query = SelectStatement::new()
        .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
        .from("cte")
        .with_cte(cte)
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        [
            r#"WITH "cte" AS"#,
            r#"(SELECT "id", "image", "aspect""#,
            r#"FROM "glyph")"#,
            r#"SELECT "id", "image", "aspect" FROM "cte""#,
        ]
        .join(" ")
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_2() {
    let query = Query::insert()
        .into_table(Glyph::Table)
        .columns([Glyph::Image, Glyph::Aspect])
        .values_panic([
            "04108048005887010020060000204E0180400400".into(),
            3.1415.into(),
        ])
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#
    );
    assert_eq!(
        query.audit_unwrap().inserted_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_3() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Image, Glyph::Aspect])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .values_panic([Value::String(None).into(), 2.1345.into()])
            .to_string(PostgresQueryBuilder),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.1415), (NULL, 2.1345)"#
    );
}

#[test]
#[cfg(feature = "with-chrono")]
fn insert_4() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Image])
            .values_panic([chrono::NaiveDateTime::from_timestamp_opt(0, 0)
                .unwrap()
                .into()])
            .to_string(PostgresQueryBuilder),
        "INSERT INTO \"glyph\" (\"image\") VALUES ('1970-01-01 00:00:00.000000')"
    );
}

#[test]
#[cfg(feature = "with-time")]
fn insert_9() {
    use time::macros::{date, time};
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Image])
            .values_panic([date!(1970 - 01 - 01).with_time(time!(00:00:00)).into()])
            .to_string(PostgresQueryBuilder),
        "INSERT INTO \"glyph\" (\"image\") VALUES ('1970-01-01 00:00:00.000000')"
    );
}

#[test]
#[cfg(feature = "with-uuid")]
fn insert_5() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Image])
            .values_panic([uuid::Uuid::nil().into()])
            .to_string(PostgresQueryBuilder),
        "INSERT INTO \"glyph\" (\"image\") VALUES ('00000000-0000-0000-0000-000000000000')"
    );
}

#[test]
fn insert_from_select() {
    let query = Query::insert()
        .into_table(Glyph::Table)
        .or_default_values()
        .columns([Glyph::Aspect, Glyph::Image])
        .select_from(
            Query::select()
                .column(Glyph::Aspect)
                .column(Glyph::Image)
                .from(Font::Table)
                .conditions(
                    true,
                    |x| {
                        x.and_where(Expr::col(Glyph::Image).like("%"));
                    },
                    |x| {
                        x.and_where(Expr::col(Glyph::Id).eq(6));
                    },
                )
                .to_owned(),
        )
        .unwrap()
        .take();

    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"INSERT INTO "glyph" ("aspect", "image") SELECT "aspect", "image" FROM "font" WHERE "image" LIKE '%'"#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Font::Table.into_iden()]
    );
    assert_eq!(
        query.audit_unwrap().inserted_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
fn insert_6() {
    let select = SelectStatement::new()
        .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
        .from(Glyph::Table)
        .to_owned();
    let cte = CommonTableExpression::new()
        .query(select)
        .column(Glyph::Id)
        .column(Glyph::Image)
        .column(Glyph::Aspect)
        .table_name("cte")
        .to_owned();
    let with_clause = WithClause::new().cte(cte).to_owned();
    let select = SelectStatement::new()
        .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
        .from("cte")
        .to_owned();
    let mut insert = Query::insert();
    insert
        .into_table(Glyph::Table)
        .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
        .select_from(select)
        .unwrap();
    let query = insert.with(with_clause);
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        [
            r#"WITH "cte" ("id", "image", "aspect") AS (SELECT "id", "image", "aspect" FROM "glyph")"#,
            r#"INSERT INTO "glyph" ("id", "image", "aspect") SELECT "id", "image", "aspect" FROM "cte""#,
        ].join(" ")
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Glyph::Table.into_iden()]
    );
    assert_eq!(
        query.audit_unwrap().inserted_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
fn insert_7() {
    let query = Query::insert()
        .into_table(Glyph::Table)
        .or_default_values()
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"INSERT INTO "glyph" VALUES (DEFAULT)"#
    );
    assert_eq!(query.audit_unwrap().selected_tables(), []);
    assert_eq!(
        query.audit_unwrap().inserted_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
fn insert_8() {
    let query = Query::insert()
        .into_table(Glyph::Table)
        .or_default_values()
        .returning_col(Glyph::Id)
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"INSERT INTO "glyph" VALUES (DEFAULT) RETURNING "id""#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Glyph::Table.into_iden()]
    );
    assert_eq!(
        query.audit_unwrap().inserted_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
#[cfg(feature = "postgres-array")]
fn insert_10() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Tokens])
            .values_panic([
                3.1415.into(),
                vec![
                    "Token1".to_string(),
                    "Token2".to_string(),
                    "Token3".to_string()
                ]
                .into()
            ])
            .to_string(PostgresQueryBuilder),
        r#"INSERT INTO "glyph" ("aspect", "tokens") VALUES (3.1415, ARRAY ['Token1','Token2','Token3'])"#
    );
}

// regression tests for https://github.com/SeaQL/sea-query/issues/853
#[test]
#[cfg(feature = "postgres-array")]
fn insert_issue_853() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Tokens])
            .values_panic([3.1415.into(), Vec::<String>::new().into()])
            .to_string(PostgresQueryBuilder),
        r#"INSERT INTO "glyph" ("aspect", "tokens") VALUES (3.1415, '{}')"#
    );
}

#[test]
fn insert_11() {
    let select = SelectStatement::new()
        .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
        .from(Glyph::Table)
        .to_owned();
    let cte = CommonTableExpression::new()
        .query(select)
        .column(Glyph::Id)
        .column(Glyph::Image)
        .column(Glyph::Aspect)
        .table_name("cte")
        .to_owned();
    let select = SelectStatement::new()
        .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
        .from("cte")
        .to_owned();
    let insert = Query::insert()
        .into_table(Glyph::Table)
        .columns([Glyph::Id, Glyph::Image, Glyph::Aspect])
        .with_cte(cte)
        .select_from(select)
        .unwrap()
        .take();
    assert_eq!(
        insert.to_string(PostgresQueryBuilder),
        [
            r#"WITH "cte" ("id", "image", "aspect") AS (SELECT "id", "image", "aspect" FROM "glyph")"#,
            r#"INSERT INTO "glyph" ("id", "image", "aspect") SELECT "id", "image", "aspect" FROM "cte""#,
        ].join(" ")
    );
    assert_eq!(
        insert.audit_unwrap().selected_tables(),
        [Glyph::Table.into_iden()]
    );
    assert_eq!(
        insert.audit_unwrap().inserted_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_1() {
    let query = Query::insert()
        .into_table(Glyph::Table)
        .columns([Glyph::Aspect, Glyph::Image])
        .values_panic([
            "04108048005887010020060000204E0180400400".into(),
            3.1415.into(),
        ])
        .on_conflict(
            OnConflict::column(Glyph::Id)
                .update_column(Glyph::Aspect)
                .to_owned(),
        )
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id") DO UPDATE SET "aspect" = "excluded"."aspect""#,
        ]
        .join(" ")
    );
    assert_eq!(query.audit_unwrap().selected_tables(), []);
    assert_eq!(
        query.audit_unwrap().inserted_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_2() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::columns([Glyph::Id, Glyph::Aspect])
                    .update_columns([Glyph::Aspect, Glyph::Image])
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = "excluded"."image""#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_3() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::columns([Glyph::Id, Glyph::Aspect])
                    .values([
                        (Glyph::Aspect, "04108048005887010020060000204E0180400400".into()),
                        (Glyph::Image, 3.1415.into()),
                    ])
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "aspect" = '04108048005887010020060000204E0180400400', "image" = 3.1415"#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_4() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::columns([Glyph::Id, Glyph::Aspect])
                    .value(Glyph::Image, Expr::val(1).add(2))
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "image" = 1 + 2"#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_5() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::columns([Glyph::Id, Glyph::Aspect])
                    .value(Glyph::Aspect, Expr::val("04108048005887010020060000204E0180400400"))
                    .update_column(Glyph::Image)
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "aspect" = '04108048005887010020060000204E0180400400', "image" = "excluded"."image""#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_6() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::columns([Glyph::Id, Glyph::Aspect])
                    .update_column(Glyph::Aspect)
                    .value(Glyph::Image, Expr::val(1).add(2))
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "aspect" = "excluded"."aspect", "image" = 1 + 2"#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_7() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::new()
                    .expr(Expr::col(Glyph::Id))
                    .update_column(Glyph::Aspect)
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id") DO UPDATE SET "aspect" = "excluded"."aspect""#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_8() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::new()
                    .exprs([Expr::col(Glyph::Id), Expr::col(Glyph::Aspect)])
                    .update_column(Glyph::Aspect)
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO UPDATE SET "aspect" = "excluded"."aspect""#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_9() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .on_conflict(
                OnConflict::column(Glyph::Id)
                    .expr(Func::lower(Expr::col(Glyph::Tokens)))
                    .update_column(Glyph::Aspect)
                    .to_owned()
            )
            .to_string(PostgresQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('04108048005887010020060000204E0180400400', 3.1415)"#,
            r#"ON CONFLICT ("id", LOWER("tokens")) DO UPDATE SET "aspect" = "excluded"."aspect""#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_do_nothing() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .values_panic(["abcd".into(), 3.1415.into()])
            .on_conflict(
                OnConflict::columns([Glyph::Id, Glyph::Aspect])
                    .do_nothing()
                    .to_owned(),
            )
            .to_string(PostgresQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('abcd', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO NOTHING"#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_on_conflict_do_nothing_on() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Aspect, Glyph::Image])
            .values_panic(["abcd".into(), 3.1415.into()])
            .on_conflict(
                OnConflict::columns([Glyph::Id, Glyph::Aspect])
                    .do_nothing_on([Glyph::Id])
                    .to_owned(),
            )
            .to_string(PostgresQueryBuilder),
        [
            r#"INSERT INTO "glyph" ("aspect", "image")"#,
            r#"VALUES ('abcd', 3.1415)"#,
            r#"ON CONFLICT ("id", "aspect") DO NOTHING"#,
        ]
        .join(" ")
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_returning_all_columns() {
    assert_eq!(
        Query::insert()
            .into_table(Glyph::Table)
            .columns([Glyph::Image, Glyph::Aspect])
            .values_panic([
                "04108048005887010020060000204E0180400400".into(),
                3.1415.into(),
            ])
            .returning(Query::returning().all())
            .to_string(PostgresQueryBuilder),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.1415) RETURNING *"#
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn insert_returning_specific_columns() {
    let query = Query::insert()
        .into_table(Glyph::Table)
        .columns([Glyph::Image, Glyph::Aspect])
        .values_panic([
            "04108048005887010020060000204E0180400400".into(),
            3.1415.into(),
        ])
        .returning(Query::returning().columns([Glyph::Id, Glyph::Image]))
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"INSERT INTO "glyph" ("image", "aspect") VALUES ('04108048005887010020060000204E0180400400', 3.1415) RETURNING "id", "image""#
    );
    assert_eq!(
        query.audit_unwrap().selected_tables(),
        [Glyph::Table.into_iden()]
    );
    assert_eq!(
        query.audit_unwrap().inserted_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
fn update_1() {
    let query = Query::update()
        .table(Glyph::Table)
        .values([
            (Glyph::Aspect, 2.1345.into()),
            (
                Glyph::Image,
                "24B0E11951B03B07F8300FD003983F03F0780060".into(),
            ),
        ])
        .and_where(Expr::col(Glyph::Id).eq(1))
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"UPDATE "glyph" SET "aspect" = 2.1345, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1"#
    );
    assert_eq!(
        query.audit_unwrap().updated_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
fn update_3() {
    assert_eq!(
        Query::update()
            .table(Glyph::Table)
            .value(Glyph::Aspect, Expr::cust("60 * 24 * 24"))
            .values([(
                Glyph::Image,
                "24B0E11951B03B07F8300FD003983F03F0780060".into()
            )])
            .and_where(Expr::col(Glyph::Id).eq(1))
            .to_string(PostgresQueryBuilder),
        r#"UPDATE "glyph" SET "aspect" = 60 * 24 * 24, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1"#
    );
}

#[test]
fn update_4() {
    assert_eq!(
        Query::update()
            .table(Glyph::Table)
            .value(Glyph::Aspect, Expr::col(Glyph::Aspect).add(1))
            .values([(
                Glyph::Image,
                "24B0E11951B03B07F8300FD003983F03F0780060".into()
            )])
            .and_where(Expr::col(Glyph::Id).eq(1))
            .order_by(Glyph::Id, Order::Asc)
            .limit(1)
            .to_string(PostgresQueryBuilder),
        r#"UPDATE "glyph" SET "aspect" = "aspect" + 1, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1 ORDER BY "id" ASC LIMIT 1"#
    );
}

#[test]
fn update_returning_all_columns() {
    assert_eq!(
        Query::update()
            .table(Glyph::Table)
            .value(Glyph::Aspect, Expr::cust("60 * 24 * 24"))
            .values([(
                Glyph::Image,
                "24B0E11951B03B07F8300FD003983F03F0780060".into()
            )])
            .and_where(Expr::col(Glyph::Id).eq(1))
            .returning(Query::returning().all())
            .to_string(PostgresQueryBuilder),
        r#"UPDATE "glyph" SET "aspect" = 60 * 24 * 24, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1 RETURNING *"#
    );
}

#[test]
fn update_returning_specified_columns() {
    assert_eq!(
        Query::update()
            .table(Glyph::Table)
            .value(Glyph::Aspect, Expr::cust("60 * 24 * 24"))
            .values([(
                Glyph::Image,
                "24B0E11951B03B07F8300FD003983F03F0780060".into()
            )])
            .and_where(Expr::col(Glyph::Id).eq(1))
            .returning(Query::returning().columns([Glyph::Id, Glyph::Image]))
            .to_string(PostgresQueryBuilder),
        r#"UPDATE "glyph" SET "aspect" = 60 * 24 * 24, "image" = '24B0E11951B03B07F8300FD003983F03F0780060' WHERE "id" = 1 RETURNING "id", "image""#
    );
}

#[test]
fn delete_1() {
    let query = Query::delete()
        .from_table(Glyph::Table)
        .and_where(Expr::col(Glyph::Id).eq(1))
        .take();
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"DELETE FROM "glyph" WHERE "id" = 1"#
    );
    assert_eq!(
        query.audit_unwrap().deleted_tables(),
        [Glyph::Table.into_iden()]
    );
}

#[test]
fn escape_1() {
    let test = r#" "abc" "#;
    assert_eq!(
        PostgresQueryBuilder.escape_string(test),
        r#" \"abc\" "#.to_owned()
    );
    assert_eq!(
        PostgresQueryBuilder.unescape_string(PostgresQueryBuilder.escape_string(test).as_str()),
        test
    )
}

#[test]
fn escape_2() {
    let test = "a\nb\tc";
    assert_eq!(
        PostgresQueryBuilder.escape_string(test),
        "a\\nb\\tc".to_owned()
    );
    assert_eq!(
        PostgresQueryBuilder.unescape_string(PostgresQueryBuilder.escape_string(test).as_str()),
        test
    );
}

#[test]
fn escape_3() {
    let test = "a\\b";
    assert_eq!(
        PostgresQueryBuilder.escape_string(test),
        "a\\\\b".to_owned()
    );
    assert_eq!(
        PostgresQueryBuilder.unescape_string(PostgresQueryBuilder.escape_string(test).as_str()),
        test
    );
}

#[test]
fn escape_4() {
    let test = "a\"b";
    assert_eq!(
        PostgresQueryBuilder.escape_string(test),
        "a\\\"b".to_owned()
    );
    assert_eq!(
        PostgresQueryBuilder.unescape_string(PostgresQueryBuilder.escape_string(test).as_str()),
        test
    )
}

#[test]
fn delete_returning_all_columns() {
    assert_eq!(
        Query::delete()
            .from_table(Glyph::Table)
            .and_where(Expr::col(Glyph::Id).eq(1))
            .returning(Query::returning().all())
            .to_string(PostgresQueryBuilder),
        r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING *"#
    );
}

#[test]
fn delete_returning_specific_columns() {
    assert_eq!(
        Query::delete()
            .from_table(Glyph::Table)
            .and_where(Expr::col(Glyph::Id).eq(1))
            .returning(Query::returning().columns([Glyph::Id, Glyph::Image]))
            .to_string(PostgresQueryBuilder),
        r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id", "image""#
    );
}

#[test]
fn delete_returning_specific_exprs() {
    assert_eq!(
        Query::delete()
            .from_table(Glyph::Table)
            .and_where(Expr::col(Glyph::Id).eq(1))
            .returning(Query::returning().exprs([Expr::col(Glyph::Id), Expr::col(Glyph::Image)]))
            .to_string(PostgresQueryBuilder),
        r#"DELETE FROM "glyph" WHERE "id" = 1 RETURNING "id", "image""#
    );
}

#[test]
fn select_pgtrgm_similarity() {
    assert_eq!(
        Query::select()
            .expr(Expr::col(Font::Name).binary(PgBinOper::Similarity, Expr::value("serif")))
            .from(Font::Table)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "name" % 'serif' FROM "font""#
    );
}

#[test]
fn select_pgtrgm_word_similarity() {
    assert_eq!(
        Query::select()
            .expr(Expr::col(Font::Name).binary(PgBinOper::WordSimilarity, Expr::value("serif")))
            .from(Font::Table)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "name" <% 'serif' FROM "font""#
    );
}

#[test]
fn select_pgtrgm_strict_word_similarity() {
    assert_eq!(
        Query::select()
            .expr(
                Expr::col(Font::Name).binary(PgBinOper::StrictWordSimilarity, Expr::value("serif"))
            )
            .from(Font::Table)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "name" <<% 'serif' FROM "font""#
    );
}

#[test]
fn select_pgtrgm_similarity_distance() {
    assert_eq!(
        Query::select()
            .expr(Expr::col(Font::Name).binary(PgBinOper::SimilarityDistance, Expr::value("serif")))
            .from(Font::Table)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "name" <-> 'serif' FROM "font""#
    );
}

#[test]
fn select_pgtrgm_word_similarity_distance() {
    assert_eq!(
        Query::select()
            .expr(
                Expr::col(Font::Name)
                    .binary(PgBinOper::WordSimilarityDistance, Expr::value("serif"))
            )
            .from(Font::Table)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "name" <<-> 'serif' FROM "font""#
    );
}

#[test]
fn select_pgtrgm_strict_word_similarity_distance() {
    assert_eq!(
        Query::select()
            .expr(Expr::col(Font::Name).binary(
                PgBinOper::StrictWordSimilarityDistance,
                Expr::value("serif")
            ))
            .from(Font::Table)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "name" <<<-> 'serif' FROM "font""#
    );
}

#[test]
fn select_custom_operator() {
    assert_eq!(
        Query::select()
            .expr(Expr::col(Font::Name).binary(BinOper::Custom("~*"), Expr::value("serif")))
            .from(Font::Table)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "name" ~* 'serif' FROM "font""#
    );
    assert_eq!(
        Query::select()
            .expr(Expr::col(Font::Name).binary(BinOper::Custom("~"), Expr::value("serif")))
            .from(Font::Table)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "name" ~ 'serif' FROM "font""#
    );
}

#[test]
fn union_1() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .union(
                UnionType::Distinct,
                Query::select()
                    .column(Char::Character)
                    .from(Char::Table)
                    .left_join(
                        Font::Table,
                        Expr::col((Char::Table, Char::FontId)).equals((Font::Table, Font::Id))
                    )
                    .order_by((Font::Table, Font::Id), Order::Asc)
                    .take()
            )
            .to_string(PostgresQueryBuilder),
        [
            r#"SELECT "character" FROM "character" UNION (SELECT "character" FROM "character""#,
            r#"LEFT JOIN "font" ON "character"."font_id" = "font"."id" ORDER BY "font"."id" ASC)"#
        ]
        .join(" ")
    );
}

#[test]
fn sub_query_with_fn() {
    #[derive(Iden)]
    #[iden = "ARRAY"]
    pub struct ArrayFunc;

    let sub_select = Query::select()
        .column(Asterisk)
        .from(Char::Table)
        .to_owned();

    let select = Query::select()
        .expr(Func::cust(ArrayFunc).arg(Expr::SubQuery(None, Box::new(sub_select.into()))))
        .to_owned();

    assert_eq!(
        select.to_string(PostgresQueryBuilder),
        r#"SELECT ARRAY((SELECT * FROM "character"))"#
    );
}

#[test]
fn md5_fn() {
    assert_eq!(
        Query::select()
            .expr(Func::md5(Expr::val("test")))
            .to_string(PostgresQueryBuilder),
        r#"SELECT MD5('test')"#
    );
}

#[test]
fn select_array_contains_bin_oper() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(Expr::col(Char::Character).binary(PgBinOper::Contains, Expr::val("test")))
            .build(PostgresQueryBuilder),
        (
            r#"SELECT "character" FROM "character" WHERE "character" @> $1"#.to_owned(),
            Values(vec!["test".into()])
        )
    );
}

#[test]
fn select_array_contained_bin_oper() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(Expr::col(Char::Character).binary(PgBinOper::Contained, Expr::val("test")))
            .build(PostgresQueryBuilder),
        (
            r#"SELECT "character" FROM "character" WHERE "character" <@ $1"#.to_owned(),
            Values(vec!["test".into()])
        )
    );
}

#[test]
fn select_array_overlap_bin_oper() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(Expr::col(Char::Character).binary(PgBinOper::Overlap, Expr::val("test")))
            .build(PostgresQueryBuilder),
        (
            r#"SELECT "character" FROM "character" WHERE "character" && $1"#.to_owned(),
            Values(vec!["test".into()])
        )
    );
}

#[test]
fn get_json_field_bin_oper() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(
                Expr::col(Char::Character).binary(PgBinOper::GetJsonField, Expr::val("test"))
            )
            .build(PostgresQueryBuilder),
        (
            r#"SELECT "character" FROM "character" WHERE "character" -> $1"#.to_owned(),
            Values(vec!["test".into()])
        )
    );
}

#[test]
fn cast_json_field_bin_oper() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(
                Expr::col(Char::Character).binary(PgBinOper::CastJsonField, Expr::val("test"))
            )
            .build(PostgresQueryBuilder),
        (
            r#"SELECT "character" FROM "character" WHERE "character" ->> $1"#.to_owned(),
            Values(vec!["test".into()])
        )
    );
}

#[test]
fn regex_bin_oper() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(Expr::col(Char::Character).binary(PgBinOper::Regex, Expr::val("test")))
            .build(PostgresQueryBuilder),
        (
            r#"SELECT "character" FROM "character" WHERE "character" ~ $1"#.to_owned(),
            Values(vec!["test".into()])
        )
    );
}

#[test]
fn regex_case_insensitive_bin_oper() {
    assert_eq!(
        Query::select()
            .column(Char::Character)
            .from(Char::Table)
            .and_where(
                Expr::col(Char::Character)
                    .binary(PgBinOper::RegexCaseInsensitive, Expr::val("test"))
            )
            .build(PostgresQueryBuilder),
        (
            r#"SELECT "character" FROM "character" WHERE "character" ~* $1"#.to_owned(),
            Values(vec!["test".into()])
        )
    );
}

#[test]
fn test_issue_674_nested_logical() {
    let t = Expr::Value(true.into());
    let f = Expr::Value(false.into());

    let x_op_y = |x, op, y| Expr::Binary(Box::new(x), op, Box::new(y));
    let t_or_t = x_op_y(t.clone(), BinOper::Or, t.clone());
    let t_or_t_or_f = x_op_y(t_or_t, BinOper::Or, f);
    let t_or_t_or_f_and_t = x_op_y(t_or_t_or_f.clone(), BinOper::And, t);

    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .and_where(t_or_t_or_f_and_t)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE (TRUE OR TRUE OR FALSE) AND TRUE"#
    );
}

#[test]
fn test_issue_674_nested_comparison() {
    let int100 = Expr::Value(100i32.into());
    let int0 = Expr::Value(0i32.into());
    let int1 = Expr::Value(1i32.into());

    let x_op_y = |x, op, y| Expr::Binary(Box::new(x), op, Box::new(y));
    let t_smaller_than_t = x_op_y(int100, BinOper::SmallerThan, int0);
    let t_smaller_than_t_smaller_than_f = x_op_y(t_smaller_than_t, BinOper::SmallerThan, int1);

    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .and_where(t_smaller_than_t_smaller_than_f)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE (100 < 0) < 1"#
    );
}

#[test]
fn test_issue_674_and_inside_not() {
    let t = Expr::Value(true.into());
    let f = Expr::Value(false.into());

    let op_x = |op, x| Expr::Unary(op, Box::new(x));
    let x_op_y = |x, op, y| Expr::Binary(Box::new(x), op, Box::new(y));
    let f_and_t = x_op_y(f, BinOper::And, t);
    let not_f_and_t = op_x(UnOper::Not, f_and_t);

    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .and_where(not_f_and_t)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE NOT (FALSE AND TRUE)"#
    );
}

#[test]
fn test_issue_674_nested_logical_panic() {
    let e = Expr::from(true).and(Expr::from(true).and(true).and(true));

    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .and_where(e)
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE TRUE AND (TRUE AND TRUE AND TRUE)"#
    );
}

#[test]
#[cfg(feature = "postgres-vector")]
fn test_pgvector_select() {
    assert_eq!(
        Query::select()
            .columns([Char::Character])
            .from(Char::Table)
            .and_where(
                Expr::col(Char::Character).eq(Expr::val(pgvector::Vector::from(vec![1.0, 2.0])))
            )
            .to_string(PostgresQueryBuilder),
        r#"SELECT "character" FROM "character" WHERE "character" = '[1,2]'"#
    );
}
