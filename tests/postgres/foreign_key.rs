use super::*;

#[test]
fn create_1() {
    assert_eq!(
        ForeignKey::create()
            .name("FK_2e303c3a712662f1fc2a4d0aad6")
            .from(Char::Table, Char::FontId)
            .to(Font::Table, Font::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_string(PostgresQueryBuilder),
        [
            r#"ALTER TABLE "character" ADD CONSTRAINT "FK_2e303c3a712662f1fc2a4d0aad6""#,
            r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id")"#,
            r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_2() {
    assert_eq!(
        ForeignKey::create()
            .name("FK_2e303c3a712662f1fc2a4d0aad6")
            .from((Alias::new("schema"), Char::Table), Char::FontId,)
            .to(Font::Table, Font::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_string(PostgresQueryBuilder),
        [
            r#"ALTER TABLE "schema"."character" ADD CONSTRAINT "FK_2e303c3a712662f1fc2a4d0aad6""#,
            r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id")"#,
            r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_3() {
    assert_eq!(
        ForeignKey::create()
            .name("FK_2e303c3a712662f1fc2a4d0aad6")
            .from((Alias::new("schema"), Char::Table), Char::FontId,)
            .to(Font::Table, Font::Id)
            .deferrable(Deferrable::DeferrableInitiallyDeferred)
            .to_string(PostgresQueryBuilder),
        [
            r#"ALTER TABLE "schema"."character" ADD CONSTRAINT "FK_2e303c3a712662f1fc2a4d0aad6""#,
            r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id")"#,
            r#"DEFERRABLE INITIALLY DEFERRED"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_4() {
    assert_eq!(
        ForeignKey::create()
            .name("FK_2e303c3a712662f1fc2a4d0aad6")
            .from((Alias::new("schema"), Char::Table), Char::FontId,)
            .to(Font::Table, Font::Id)
            .deferrable(Deferrable::DeferrableInitiallyImmediate)
            .to_string(PostgresQueryBuilder),
        [
            r#"ALTER TABLE "schema"."character" ADD CONSTRAINT "FK_2e303c3a712662f1fc2a4d0aad6""#,
            r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id")"#,
            r#"DEFERRABLE INITIALLY IMMEDIATE"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_5() {
    assert_eq!(
        ForeignKey::create()
            .name("FK_2e303c3a712662f1fc2a4d0aad6")
            .from((Alias::new("schema"), Char::Table), Char::FontId,)
            .to(Font::Table, Font::Id)
            .deferrable(Deferrable::NotDeferrable)
            .to_string(PostgresQueryBuilder),
        [
            r#"ALTER TABLE "schema"."character" ADD CONSTRAINT "FK_2e303c3a712662f1fc2a4d0aad6""#,
            r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id")"#,
            r#"NOT DEFERRABLE"#,
        ]
        .join(" ")
    );
}

#[test]
fn drop_1() {
    assert_eq!(
        ForeignKey::drop()
            .name("FK_2e303c3a712662f1fc2a4d0aad6")
            .table(Char::Table)
            .to_string(PostgresQueryBuilder),
        r#"ALTER TABLE "character" DROP CONSTRAINT "FK_2e303c3a712662f1fc2a4d0aad6""#
    );
}

#[test]
fn drop_2() {
    assert_eq!(
        ForeignKey::drop()
            .name("FK_2e303c3a712662f1fc2a4d0aad6")
            .table((Alias::new("schema"), Char::Table))
            .to_string(PostgresQueryBuilder),
        r#"ALTER TABLE "schema"."character" DROP CONSTRAINT "FK_2e303c3a712662f1fc2a4d0aad6""#
    );
}
