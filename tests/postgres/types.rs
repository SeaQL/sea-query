use sea_query::{extension::postgres::Type, tests_cfg::Glyph};

use super::*;

fn create_1() {
    assert_eq!(
        Type::create()
            .as_enum(Font::Table)
            .values(vec![Font::Name, Font::Variant, Font::Language])
            .to_string(PostgresQueryBuilder),
        r#"CREATE TYPE "font" AS ENUM ('name', 'variant', 'language')"#
    );
}

#[test]
fn drop_1() {
    assert_eq!(
        Type::drop()
            .if_exists()
            .name(Font::Table)
            .restrict()
            .to_string(PostgresQueryBuilder),
        r#"DROP TYPE IF EXISTS "font" RESTRICT"#
    )
}

#[test]
fn drop_2() {
    assert_eq!(
        Type::drop()
            .name(Font::Table)
            .to_string(PostgresQueryBuilder),
        r#"DROP TYPE "font""#
    );
}

#[test]
fn drop_3() {
    use sea_query::extension::postgres::Type;

    assert_eq!(
        Type::drop()
            .if_exists()
            .name(Font::Table)
            .cascade()
            .to_string(PostgresQueryBuilder),
        r#"DROP TYPE IF EXISTS "font" CASCADE"#
    );
}