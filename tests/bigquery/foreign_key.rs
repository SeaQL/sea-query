use super::*;

#[test]
#[should_panic(expected = "Not supported")]
fn create_1() {
    ForeignKey::create()
        .name("FK_2e303c3a712662f1fc2a4d0aad6")
        .from(Char::Table, Char::FontId)
        .to(Font::Table, Font::Id)
        .on_delete(ForeignKeyAction::Cascade)
        .on_update(ForeignKeyAction::Cascade)
        .to_string(BigQueryQueryBuilder);
}

#[test]
#[should_panic(expected = "Not supported")]
fn drop_1() {
    ForeignKey::drop()
        .name("FK_2e303c3a712662f1fc2a4d0aad6")
        .table(Char::Table)
        .to_string(BigQueryQueryBuilder);
}
