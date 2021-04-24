use super::*;

#[test]
fn create_1() {
    assert_eq!(
        Index::create()
            .name("idx-glyph-aspect")
            .table(Glyph::Table)
            .col(Glyph::Aspect)
            .to_string(SqliteQueryBuilder),
        "CREATE INDEX `idx-glyph-aspect` ON `glyph` (`aspect`)"
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
            .to_string(SqliteQueryBuilder),
        "CREATE UNIQUE INDEX `idx-glyph-aspect-image` ON `glyph` (`aspect`, `image`)"
    );
}

#[test]
fn drop_1() {
    assert_eq!(
        Index::drop()
            .name("idx-glyph-aspect")
            .table(Glyph::Table)
            .to_string(SqliteQueryBuilder),
        "DROP INDEX `idx-glyph-aspect` ON `glyph`"
    );
}