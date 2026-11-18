use super::*;
use pretty_assertions::assert_eq;

#[test]
fn create_1() {
    assert_eq!(
        Constraint::create()
            .primary()
            .constraint_name("PK_2e303c3a712662f1fc2a4d0aad6")
            .table(Font::Table)
            .col(Font::Id)
            .to_string(PostgresQueryBuilder),
        [
            r#"ALTER TABLE "font" ADD CONSTRAINT "PK_2e303c3a712662f1fc2a4d0aad6""#,
            r#"PRIMARY KEY ("id")"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_2() {
    assert_eq!(
        Constraint::create()
            .unique()
            .constraint_name("UQ_2e303c3a712662f1fc2a4d0aad6")
            .table(Font::Table)
            .col(Font::Name)
            .to_string(PostgresQueryBuilder),
        [
            r#"ALTER TABLE "font" ADD CONSTRAINT "UQ_2e303c3a712662f1fc2a4d0aad6""#,
            r#"UNIQUE ("name")"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_3() {
    assert_eq!(
        Constraint::create()
            .constraint_name("id_range")
            .check(Expr::col(Glyph::Id).lt(20))
            .table(Glyph::Table)
            .to_string(PostgresQueryBuilder),
        [r#"ALTER TABLE "glyph" ADD CONSTRAINT "id_range" CHECK ("id" < 20)"#,].join(" ")
    );
    assert_eq!(
        Constraint::create()
            .check(Expr::col(Glyph::Id).lt(20))
            .constraint_name("id_range")
            .table(Glyph::Table)
            .to_string(PostgresQueryBuilder),
        [r#"ALTER TABLE "glyph" ADD CONSTRAINT "id_range" CHECK ("id" < 20)"#,].join(" ")
    );
}

#[test]
fn create_4() {
    assert_eq!(
        Constraint::create()
            .check(Expr::col(Glyph::Id).lt(20))
            .table(Glyph::Table)
            .to_string(PostgresQueryBuilder),
        [r#"ALTER TABLE "glyph" ADD CHECK ("id" < 20)"#,].join(" ")
    );
}

#[test]
#[should_panic(
    expected = "Postgres does not support combining USING INDEX with columns or index options"
)]
fn create_5() {
    Constraint::create()
        .unique()
        .constraint_name("font_name_key")
        .using_index("idx_font_name")
        .col(Font::Name)
        .table(Font::Table)
        .to_string(PostgresQueryBuilder);
}

#[test]
#[should_panic(
    expected = "Postgres does not support NULLS NOT DISTINCT on PRIMARY KEY constraints"
)]
fn create_6() {
    Constraint::create()
        .primary()
        .constraint_name("font_pkey")
        .nulls_not_distinct()
        .col(Font::Id)
        .table(Font::Table)
        .to_string(PostgresQueryBuilder);
}

#[test]
#[should_panic(expected = "Postgres does not support index types in ADD CONSTRAINT")]
fn create_7() {
    Constraint::create()
        .unique()
        .constraint_name("font_name_key")
        .index_type(IndexType::Hash)
        .col(Font::Name)
        .table(Font::Table)
        .to_string(PostgresQueryBuilder);
}

#[test]
#[should_panic(expected = "Postgres does not support USING INDEX on CHECK constraints")]
fn create_8() {
    Constraint::create()
        .check(Expr::col(Glyph::Id).lt(20))
        .using_index("idx_glyph_id")
        .table(Glyph::Table)
        .to_string(PostgresQueryBuilder);
}

#[test]
#[should_panic(
    expected = "Postgres does not support columns or index options on CHECK constraints"
)]
fn create_9() {
    Constraint::create()
        .check(Expr::col(Glyph::Id).lt(20))
        .include(Glyph::Aspect)
        .table(Glyph::Table)
        .to_string(PostgresQueryBuilder);
}
