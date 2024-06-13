use super::*;

#[test]
#[should_panic(expected = "Not supported")]
fn create_1() {
    Index::create()
        .name("idx-glyph-aspect")
        .table(Glyph::Table)
        .col(Glyph::Aspect)
        .to_string(BigQueryQueryBuilder);
}

#[test]
#[should_panic(expected = "Not supported")]
fn drop_1() {
    Index::drop()
        .name("idx-glyph-aspect")
        .to_string(BigQueryQueryBuilder);
}
