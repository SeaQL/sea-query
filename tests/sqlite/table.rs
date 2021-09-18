use super::*;

#[test]
fn create_1() {
    assert_eq!(
        Table::create()
            .table(Glyph::Table)
            .col(
                ColumnDef::new(Glyph::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col(ColumnDef::new(Glyph::Aspect).double().not_null())
            .col(ColumnDef::new(Glyph::Image).text())
            .to_string(SqliteQueryBuilder),
        vec![
            "CREATE TABLE `glyph` (",
            "`id` integer NOT NULL PRIMARY KEY AUTOINCREMENT,",
            "`aspect` real NOT NULL,",
            "`image` text",
            ")",
        ]
        .join(" ")
    );
}

#[test]
fn create_2() {
    assert_eq!(
        Table::create()
            .table(Font::Table)
            .col(
                ColumnDef::new(Font::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment()
            )
            .col(ColumnDef::new(Font::Name).string().not_null())
            .col(ColumnDef::new(Font::Variant).string().not_null())
            .col(ColumnDef::new(Font::Language).string().not_null())
            .to_string(SqliteQueryBuilder),
        vec![
            "CREATE TABLE `font` (",
            "`id` integer NOT NULL PRIMARY KEY AUTOINCREMENT,",
            "`name` text NOT NULL,",
            "`variant` text NOT NULL,",
            "`language` text NOT NULL",
            ")",
        ]
        .join(" ")
    );
}

#[test]
fn create_3() {
    assert_eq!(
        Table::create()
            .table(Char::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Char::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col(ColumnDef::new(Char::FontSize).integer().not_null())
            .col(ColumnDef::new(Char::Character).string().not_null())
            .col(ColumnDef::new(Char::SizeW).integer().not_null())
            .col(ColumnDef::new(Char::SizeH).integer().not_null())
            .col(
                ColumnDef::new(Char::FontId)
                    .integer()
                    .default(Value::Int(None))
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Char::Table, Char::FontId)
                    .to(Font::Table, Font::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
            )
            .to_string(SqliteQueryBuilder),
        vec![
            "CREATE TABLE IF NOT EXISTS `character` (",
            "`id` integer NOT NULL PRIMARY KEY AUTOINCREMENT,",
            "`font_size` integer NOT NULL,",
            "`character` text NOT NULL,",
            "`size_w` integer NOT NULL,",
            "`size_h` integer NOT NULL,",
            "`font_id` integer DEFAULT NULL,",
            "FOREIGN KEY (`font_id`) REFERENCES `font` (`id`) ON DELETE CASCADE ON UPDATE CASCADE",
            ")",
        ]
        .join(" ")
    );
}

#[test]
fn drop_1() {
    assert_eq!(
        Table::drop()
            .table(Glyph::Table)
            .table(Char::Table)
            .cascade()
            .to_string(SqliteQueryBuilder),
        "DROP TABLE `glyph`, `character`"
    );
}

#[test]
fn truncate_1() {
    assert_eq!(
        Table::truncate()
            .table(Font::Table)
            .to_string(SqliteQueryBuilder),
        "TRUNCATE TABLE `font`"
    );
}

#[test]
fn alter_1() {
    assert_eq!(
        Table::alter()
            .table(Font::Table)
            .add_column(
                ColumnDef::new(Alias::new("new_col"))
                    .integer()
                    .not_null()
                    .default(99)
            )
            .to_string(SqliteQueryBuilder),
        "ALTER TABLE `font` ADD COLUMN `new_col` integer NOT NULL DEFAULT 99"
    );
}

#[test]
#[should_panic(expected = "Sqlite not support modifying table column")]
fn alter_2() {
    Table::alter()
        .table(Font::Table)
        .modify_column(ColumnDef::new(Alias::new("new_col")).double())
        .to_string(SqliteQueryBuilder);
}

#[test]
fn alter_3() {
    assert_eq!(
        Table::alter()
            .table(Font::Table)
            .rename_column(Alias::new("new_col"), Alias::new("new_column"))
            .to_string(SqliteQueryBuilder),
        "ALTER TABLE `font` RENAME COLUMN `new_col` TO `new_column`"
    );
}

#[test]
#[should_panic(expected = "Sqlite not support dropping table column")]
fn alter_4() {
    Table::alter()
        .table(Font::Table)
        .drop_column(Alias::new("new_column"))
        .to_string(SqliteQueryBuilder);
}

#[test]
fn alter_5() {
    assert_eq!(
        Table::rename()
            .table(Font::Table, Alias::new("font_new"))
            .to_string(SqliteQueryBuilder),
        "ALTER TABLE `font` RENAME TO `font_new`"
    );
}

#[test]
#[should_panic(expected = "No alter option found")]
fn alter_6() {
    Table::alter().to_string(SqliteQueryBuilder);
}
