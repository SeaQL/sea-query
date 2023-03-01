use super::*;
use pretty_assertions::assert_eq;

#[test]
fn create_1() {
    assert_eq!(
        Index::create()
            .name("idx-glyph-aspect")
            .table(Glyph::Table)
            .col(Glyph::Aspect)
            .to_string(PostgresQueryBuilder),
        r#"CREATE INDEX "idx-glyph-aspect" ON "glyph" ("aspect")"#
    );
}

#[test]
fn create_2() {
    assert_eq!(
        Index::create()
            .unique()
            .name("idx-glyph-aspect-image")
            .table(Glyph::Table)
            .col(Glyph::Aspect)
            .col(Glyph::Image)
            .to_string(PostgresQueryBuilder),
        r#"CREATE UNIQUE INDEX "idx-glyph-aspect-image" ON "glyph" ("aspect", "image")"#
    );
}

#[test]
fn create_3() {
    assert_eq!(
        Index::create()
            .full_text()
            .name("idx-glyph-image")
            .table(Glyph::Table)
            .col(Glyph::Image)
            .to_string(PostgresQueryBuilder),
        r#"CREATE INDEX "idx-glyph-image" ON "glyph" USING GIN ("image")"#
    );
}

#[test]
fn create_4() {
    assert_eq!(
        Index::create()
            .if_not_exists()
            .full_text()
            .name("idx-glyph-image")
            .table(Glyph::Table)
            .col(Glyph::Image)
            .to_string(PostgresQueryBuilder),
        r#"CREATE INDEX IF NOT EXISTS "idx-glyph-image" ON "glyph" USING GIN ("image")"#
    );
}

#[test]
fn create_5() {
    assert_eq!(
        Index::create()
            .unique()
            .name("idx-glyph-aspect-image")
            .table((Alias::new("schema"), Glyph::Table))
            .col(Glyph::Aspect)
            .col(Glyph::Image)
            .to_string(PostgresQueryBuilder),
        r#"CREATE UNIQUE INDEX "idx-glyph-aspect-image" ON "schema"."glyph" ("aspect", "image")"#
    );
}

#[test]
fn create_6() {
    assert_eq!(
        Index::create()
            .unique()
            .nulls_not_distinct()
            .name("idx-glyph-aspect-image")
            .table(Glyph::Table)
            .col(Glyph::Aspect)
            .col(Glyph::Image)
            .to_string(PostgresQueryBuilder),
        r#"CREATE UNIQUE INDEX "idx-glyph-aspect-image" ON "glyph" ("aspect", "image") NULLS NOT DISTINCT"#
    );
}

#[test]
fn drop_1() {
    assert_eq!(
        Index::drop()
            .name("idx-glyph-aspect")
            .to_string(PostgresQueryBuilder),
        r#"DROP INDEX "idx-glyph-aspect""#
    );
}

#[test]
fn drop_2() {
    assert_eq!(
        Index::drop()
            .name("idx-glyph-aspect")
            .table((Alias::new("schema"), Glyph::Table))
            .to_string(PostgresQueryBuilder),
        r#"DROP INDEX "schema"."idx-glyph-aspect""#
    );
}

#[test]
fn drop_3() {
    assert_eq!(
        Index::drop()
            .name("idx-glyph-aspect")
            .table(Glyph::Table)
            .to_string(PostgresQueryBuilder),
        r#"DROP INDEX "idx-glyph-aspect""#
    );
}

#[test]
#[should_panic(expected = "Not supported")]
fn drop_4() {
    Index::drop()
        .name("idx-glyph-aspect")
        .table((Alias::new("database"), Alias::new("schema"), Glyph::Table))
        .to_string(PostgresQueryBuilder);
}
