use super::*;

#[test]
#[ignore]
#[allow(clippy::approx_constant)]
fn online_1() {
    let mut env = TestEnv::new("mysql://query:query@127.0.0.1/query_test");

    let sql = Table::create()
        .table(Font::Table)
        .col(ColumnDef::new(Font::Id).integer_len(11).not_null().auto_increment().primary_key())
        .col(ColumnDef::new(Font::Name).string_len(255).not_null())
        .col(ColumnDef::new(Font::Variant).string_len(255).not_null())
        .col(ColumnDef::new(Font::Language).string_len(255).not_null())
        .engine("InnoDB")
        .character_set("utf8mb4")
        .collate("utf8mb4_unicode_ci")
        .to_string(MysqlQueryBuilder);
    assert_eq!(
        sql,
        vec![
            "CREATE TABLE `font` (",
                "`id` int(11) NOT NULL AUTO_INCREMENT PRIMARY KEY,",
                "`name` varchar(255) NOT NULL,",
                "`variant` varchar(255) NOT NULL,",
                "`language` varchar(255) NOT NULL",
            ") ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci",
        ].join(" ")
    );
    env.exec(&sql);

    let sql = Index::create()
        .name("idx-font-name")
        .table(Font::Table)
        .col(Font::Name)
        .to_string(MysqlQueryBuilder);
    assert_eq!(
        sql,
        "CREATE INDEX `idx-font-name` ON `font` (`name`)"
    );
    env.exec(&sql);

    let sql = Index::drop()
        .name("idx-font-name")
        .table(Font::Table)
        .to_string(MysqlQueryBuilder);
    assert_eq!(
        sql,
        "DROP INDEX `idx-font-name` ON `font`"
    );
    env.exec(&sql);

    let sql = Table::create()
        .table(Char::Table)
        .create_if_not_exists()
        .col(ColumnDef::new(Char::Id).integer_len(11).not_null().auto_increment().primary_key())
        .col(ColumnDef::new(Char::FontSize).integer_len(11).not_null())
        .col(ColumnDef::new(Char::Character).string_len(255).not_null())
        .col(ColumnDef::new(Char::SizeW).integer_len(11).not_null())
        .col(ColumnDef::new(Char::SizeH).integer_len(11).not_null())
        .col(ColumnDef::new(Char::FontId).integer_len(11).default(Value::Null))
        .engine("InnoDB")
        .character_set("utf8mb4")
        .collate("utf8mb4_unicode_ci")
        .to_string(MysqlQueryBuilder);
    assert_eq!(
        sql,
        vec![
            "CREATE TABLE IF NOT EXISTS `character` (",
                "`id` int(11) NOT NULL AUTO_INCREMENT PRIMARY KEY,",
                "`font_size` int(11) NOT NULL,",
                "`character` varchar(255) NOT NULL,",
                "`size_w` int(11) NOT NULL,",
                "`size_h` int(11) NOT NULL,",
                "`font_id` int(11) DEFAULT NULL",
            ") ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci",
        ].join(" ")
    );
    env.exec(&sql);

    let sql = Index::create()
        .name("idx-character-font_size")
        .table(Char::Table)
        .col(Char::FontSize)
        .to_string(MysqlQueryBuilder);
    assert_eq!(
        sql,
        "CREATE INDEX `idx-character-font_size` ON `character` (`font_size`)"
    );
    env.exec(&sql);

    let sql = Index::drop()
        .name("idx-character-font_size")
        .table(Char::Table)
        .to_string(MysqlQueryBuilder);
    assert_eq!(
        sql,
        "DROP INDEX `idx-character-font_size` ON `character`"
    );
    env.exec(&sql);

    let sql = ForeignKey::create()
        .name("FK_2e303c3a712662f1fc2a4d0aad6")
        .table(Char::Table, Font::Table)
        .col(Char::FontId, Font::Id)
        .on_delete(ForeignKeyAction::Cascade)
        .on_update(ForeignKeyAction::Cascade)
        .to_string(MysqlQueryBuilder);
    assert_eq!(
        sql,
        vec![
            "ALTER TABLE `character`",
            "ADD CONSTRAINT `FK_2e303c3a712662f1fc2a4d0aad6`",
            "FOREIGN KEY `FK_2e303c3a712662f1fc2a4d0aad6` (`font_id`) REFERENCES `font` (`id`)",
            "ON DELETE CASCADE ON UPDATE CASCADE",
        ].join(" ")
    );
    env.exec(&sql);

    let sql = ForeignKey::drop()
        .name("FK_2e303c3a712662f1fc2a4d0aad6")
        .table(Char::Table)
        .to_string(MysqlQueryBuilder);
    assert_eq!(
        sql,
        "ALTER TABLE `character` DROP FOREIGN KEY `FK_2e303c3a712662f1fc2a4d0aad6`"
    );
    env.exec(&sql);

    let sql = Query::select()
        .columns(vec![
            Char::Character, Char::SizeW, Char::SizeH
        ])
        .from(Char::Table)
        .limit(10)
        .offset(100)
        .to_string(MysqlQueryBuilder);
    assert_eq!(
        sql,
        "SELECT `character`, `size_w`, `size_h` FROM `character` LIMIT 10 OFFSET 100"
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
        .to_string(MysqlQueryBuilder);
    assert_eq!(
        sql,
        "INSERT INTO `character` (`character`, `size_w`, `size_h`, `font_size`, `font_id`) VALUES ('Character', 123, 456, 3, NULL), ('S', 12, 34, 2, NULL)"
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
        .order_by(Char::Id, Order::Asc)
        .limit(1)
        .to_string(MysqlQueryBuilder);
    assert_eq!(
        sql,
        "UPDATE `character` SET `character` = 'S', `size_w` = 1233, `size_h` = 12 WHERE `id` = 1 ORDER BY `id` ASC LIMIT 1"
    );
    env.exec(&sql);

    let sql = Table::truncate()
        .table(Char::Table)
        .to_string(MysqlQueryBuilder);
    assert_eq!(
        sql,
        "TRUNCATE TABLE `character`"
    );
    env.exec(&sql);

    let sql = Table::drop()
        .table(Char::Table)
        .cascade()
        .to_string(MysqlQueryBuilder);
    assert_eq!(
        sql,
        "DROP TABLE `character` CASCADE"
    );
    env.exec(&sql);
}