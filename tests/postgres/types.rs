use super::*;
use pretty_assertions::assert_eq;
use sea_query::{PostgresQueryBuilder, extension::postgres::Type};

#[test]
fn create_1() {
    assert_eq!(
        Type::create()
            .as_enum(Font::Table)
            .values([Font::Name, Font::Variant, Font::Language])
            .to_string(PostgresQueryBuilder),
        r#"CREATE TYPE "font" AS ENUM ('name', 'variant', 'language')"#
    );
}

#[test]
fn create_2() {
    assert_eq!(
        Type::create()
            .as_enum(("schema", Font::Table))
            .values([Font::Name, Font::Variant, Font::Language])
            .to_string(PostgresQueryBuilder),
        r#"CREATE TYPE "schema"."font" AS ENUM ('name', 'variant', 'language')"#
    );
}

#[test]
fn create_3() {
    assert_eq!(
        Type::create()
            .as_enum(Tea::Enum)
            .values([Tea::EverydayTea, Tea::BreakfastTea])
            .to_string(PostgresQueryBuilder),
        r#"CREATE TYPE "tea" AS ENUM ('EverydayTea', 'BreakfastTea')"#
    );

    enum Tea {
        Enum,
        EverydayTea,
        BreakfastTea,
    }

    impl sea_query::Iden for Tea {
        fn unquoted(&self) -> &str {
            match self {
                Self::Enum => "tea",
                Self::EverydayTea => "EverydayTea",
                Self::BreakfastTea => "BreakfastTea",
            }
        }
    }
}

#[test]
fn create_4() {
    #[derive(Iden)]
    enum FontFamily {
        #[iden = "font_family"]
        Type,
        Serif,
        Sans,
        Monospace,
    }
    assert_eq!(
        Type::create()
            .as_composite(FontFamily::Type)
            .fields([
                (FontFamily::Serif, ColumnType::Text),
                (FontFamily::Sans, ColumnType::Text),
                (FontFamily::Monospace, ColumnType::Text),
            ])
            .to_string(PostgresQueryBuilder),
        r#"CREATE TYPE "font_family" AS ("serif" text, "sans" text, "monospace" text)"#,
    );
}
#[test]
fn create_5() {
    assert_eq!(
        Type::create()
            .as_composite(("schema", Font::Table))
            .fields([
                ("serif", ColumnType::Text),
                ("sans", ColumnType::Text),
                ("monospace", ColumnType::Text),
            ])
            .to_string(PostgresQueryBuilder),
        r#"CREATE TYPE "schema"."font" AS ("serif" text, "sans" text, "monospace" text)"#,
    );
}
#[test]
fn create_6() {
    assert_eq!(
        Type::create()
            .as_composite("outer_type")
            .fields([("inner", ColumnType::custom("inner_type"))])
            .to_string(PostgresQueryBuilder),
        r#"CREATE TYPE "outer_type" AS ("inner" inner_type)"#,
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
    assert_eq!(
        Type::drop()
            .if_exists()
            .name(Font::Table)
            .cascade()
            .to_string(PostgresQueryBuilder),
        r#"DROP TYPE IF EXISTS "font" CASCADE"#
    );
}

#[test]
fn drop_4() {
    assert_eq!(
        Type::drop()
            .name(("schema", Font::Table))
            .to_string(PostgresQueryBuilder),
        r#"DROP TYPE "schema"."font""#
    );
}

#[test]
fn alter_1() {
    assert_eq!(
        Type::alter()
            .name(Font::Table)
            .add_value("weight")
            .to_string(PostgresQueryBuilder),
        r#"ALTER TYPE "font" ADD VALUE 'weight'"#
    )
}
#[test]
fn alter_2() {
    assert_eq!(
        Type::alter()
            .name(Font::Table)
            .add_value("weight")
            .before(Font::Variant)
            .to_string(PostgresQueryBuilder),
        r#"ALTER TYPE "font" ADD VALUE 'weight' BEFORE 'variant'"#
    )
}

#[test]
fn alter_3() {
    assert_eq!(
        Type::alter()
            .name(Font::Table)
            .add_value("weight")
            .after(Font::Variant)
            .to_string(PostgresQueryBuilder),
        r#"ALTER TYPE "font" ADD VALUE 'weight' AFTER 'variant'"#
    )
}

#[test]
fn alter_4() {
    assert_eq!(
        Type::alter()
            .name(Font::Table)
            .rename_to("typeface")
            .to_string(PostgresQueryBuilder),
        r#"ALTER TYPE "font" RENAME TO "typeface""#
    )
}

#[test]
fn alter_5() {
    assert_eq!(
        Type::alter()
            .name(Font::Table)
            .rename_value(Font::Variant, Font::Language)
            .to_string(PostgresQueryBuilder),
        r#"ALTER TYPE "font" RENAME VALUE 'variant' TO 'language'"#
    )
}

#[test]
fn alter_6() {
    assert_eq!(
        Type::alter()
            .name(("schema", Font::Table))
            .rename_to("typeface")
            .to_string(PostgresQueryBuilder),
        r#"ALTER TYPE "schema"."font" RENAME TO "typeface""#
    )
}

#[test]
fn alter_7() {
    assert_eq!(
        Type::alter()
            .name(Font::Table)
            .add_attribute(Font::Variant, ColumnType::Text)
            .to_string(PostgresQueryBuilder),
        r#"ALTER TYPE "font" ADD ATTRIBUTE "variant" text"#
    )
}

#[test]
fn alter_8() {
    assert_eq!(
        Type::alter()
            .name(Font::Table)
            .drop_attribute(Font::Variant)
            .to_string(PostgresQueryBuilder),
        r#"ALTER TYPE "font" DROP ATTRIBUTE "variant""#
    );
}

#[test]
fn unsigned_types() {
    let query_builder = PostgresQueryBuilder {};

    let column_to_string = |column_type| {
        let mut out = String::new();
        query_builder.prepare_column_type(column_type, &mut out);
        out
    };

    assert_eq!(column_to_string(&ColumnType::TinyUnsigned), "smallint");
    assert_eq!(column_to_string(&ColumnType::SmallUnsigned), "integer");
    assert_eq!(column_to_string(&ColumnType::Unsigned), "bigint");
    assert_eq!(column_to_string(&ColumnType::BigUnsigned), "bigint");
}
