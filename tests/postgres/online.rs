use super::*;

#[test]
#[ignore]
#[allow(clippy::approx_constant)]
fn online_1() {
    let mut env = TestEnv::new("postgresql://query:query@127.0.0.1/query_test");

    let sql = Table::create()
        .table(Font::Table)
        .col(ColumnDef::new(Font::Id).integer().not_null().primary_key().auto_increment())
        .col(ColumnDef::new(Font::Name).string_len(255).not_null())
        .col(ColumnDef::new(Font::Variant).string_len(255).not_null())
        .col(ColumnDef::new(Font::Language).string_len(255).not_null())
        .to_string(PostgresQueryBuilder);
    assert_eq!(
        sql,
        vec![
            r#"CREATE TABLE "font" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""name" varchar(255) NOT NULL,"#,
                r#""variant" varchar(255) NOT NULL,"#,
                r#""language" varchar(255) NOT NULL"#,
            r#")"#,
        ].join(" ")
    );
    env.exec(&sql);

    let sql = Index::create()
        .name("idx-font-name")
        .table(Font::Table)
        .col(Font::Name)
        .to_string(PostgresQueryBuilder);
    assert_eq!(
        sql,
        r#"CREATE INDEX "idx-font-name" ON "font" ("name")"#
    );
    env.exec(&sql);

    let sql = Index::drop()
        .name("idx-font-name")
        .to_string(PostgresQueryBuilder);
    assert_eq!(
        sql,
        r#"DROP INDEX "idx-font-name""#
    );
    env.exec(&sql);

    let sql = Table::create()
        .table(Char::Table)
        .create_if_not_exists()
        .col(ColumnDef::new(Char::Id).integer().not_null().primary_key().auto_increment())
        .col(ColumnDef::new(Char::FontSize).integer().not_null())
        .col(ColumnDef::new(Char::Character).string_len(255).not_null())
        .col(ColumnDef::new(Char::SizeW).integer().not_null())
        .col(ColumnDef::new(Char::SizeH).integer().not_null())
        .col(ColumnDef::new(Char::FontId).integer().default(Value::Null))
        .to_string(PostgresQueryBuilder);
    assert_eq!(
        sql,
        vec![
            r#"CREATE TABLE IF NOT EXISTS "character" ("#,
                r#""id" serial NOT NULL PRIMARY KEY,"#,
                r#""font_size" integer NOT NULL,"#,
                r#""character" varchar(255) NOT NULL,"#,
                r#""size_w" integer NOT NULL,"#,
                r#""size_h" integer NOT NULL,"#,
                r#""font_id" integer DEFAULT NULL"#,
            r#")"#,
        ].join(" ")
    );
    env.exec(&sql);

    let sql = Index::create()
        .name("idx-character-font_size")
        .table(Char::Table)
        .col(Char::FontSize)
        .to_string(PostgresQueryBuilder);
    assert_eq!(
        sql,
        r#"CREATE INDEX "idx-character-font_size" ON "character" ("font_size")"#
    );
    env.exec(&sql);

    let sql = Index::drop()
        .name("idx-character-font_size")
        .to_string(PostgresQueryBuilder);
    assert_eq!(
        sql,
        r#"DROP INDEX "idx-character-font_size""#
    );
    env.exec(&sql);

    let sql = ForeignKey::create()
        .name("FK_2e303c3a712662f1fc2a4d0aad6")
        .table(Char::Table, Font::Table)
        .col(Char::FontId, Font::Id)
        .on_delete(ForeignKeyAction::Cascade)
        .on_update(ForeignKeyAction::Cascade)
        .to_string(PostgresQueryBuilder);
    assert_eq!(
        sql,
        vec![
            r#"ALTER TABLE "character""#,
            r#"ADD CONSTRAINT "FK_2e303c3a712662f1fc2a4d0aad6""#,
            r#"FOREIGN KEY ("font_id") REFERENCES "font" ("id")"#,
            r#"ON DELETE CASCADE ON UPDATE CASCADE"#,
        ].join(" ")
    );
    env.exec(&sql);

    let sql = ForeignKey::drop()
        .name("FK_2e303c3a712662f1fc2a4d0aad6")
        .table(Char::Table)
        .to_string(PostgresQueryBuilder);
    assert_eq!(
        sql,
        r#"ALTER TABLE "character" DROP CONSTRAINT "FK_2e303c3a712662f1fc2a4d0aad6""#
    );
    env.exec(&sql);

    let sql = Query::select()
        .columns(vec![
            Char::Character, Char::SizeW, Char::SizeH
        ])
        .from(Char::Table)
        .limit(10)
        .offset(100)
        .to_string(PostgresQueryBuilder);
    assert_eq!(
        sql,
        r#"SELECT "character", "size_w", "size_h" FROM "character" LIMIT 10 OFFSET 100"#
    );
    env.exec(&sql);

    let sql = Query::insert()
        .into_table(Char::Table)
        .columns(vec![
            Char::Character, Char::SizeW, Char::SizeH, Char::FontSize, Char::FontId
        ])
        .values_panic(vec![
            "Character".into(),
            123.into(),
            456.into(),
            3.into(),
            Value::Null,
        ])
        .values_panic(vec![
            "S".into(),
            12.into(),
            34.into(),
            2.into(),
        ])
        .to_string(PostgresQueryBuilder);
    assert_eq!(
        sql,
        r#"INSERT INTO "character" ("character", "size_w", "size_h", "font_size", "font_id") VALUES ('Character', 123, 456, 3, NULL), ('S', 12, 34, 2, NULL)"#
    );
    env.exec(&sql);

    let sql = Query::update()
        .table(Char::Table)
        .values(vec![
            (Char::Character, "S".into()),
            (Char::SizeW, 1233.into()),
            (Char::SizeH, 12.into()),
        ])
        .and_where(Expr::col(Char::Id).eq(1))
        .to_string(PostgresQueryBuilder);
    assert_eq!(
        sql,
        r#"UPDATE "character" SET "character" = 'S', "size_w" = 1233, "size_h" = 12 WHERE "id" = 1"#
    );
    env.exec(&sql);

    let sql = Table::truncate()
        .table(Char::Table)
        .to_string(PostgresQueryBuilder);
    assert_eq!(
        sql,
        r#"TRUNCATE TABLE "character""#
    );
    env.exec(&sql);

    let sql = Table::drop()
        .table(Char::Table)
        .table(Font::Table)
        .cascade()
        .to_string(PostgresQueryBuilder);
    assert_eq!(
        sql,
        r#"DROP TABLE "character", "font" CASCADE"#
    );
    env.exec(&sql);
}