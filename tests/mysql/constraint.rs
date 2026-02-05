use super::*;
use pretty_assertions::assert_eq;

#[test]
fn create_1() {
    assert_eq!(
        Constraint::create()
            .primary()
            .constraint_name("PK_2e303c3a712662f1fc2a4d0aad6")
            .table(Font::Table)
            .col(Font::Id)
            .to_string(MysqlQueryBuilder),
        [
            r#"ALTER TABLE `font` ADD CONSTRAINT `PK_2e303c3a712662f1fc2a4d0aad6`"#,
            r#"PRIMARY KEY (`id`)"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_2() {
    assert_eq!(
        Constraint::create()
            .unique()
            .constraint_name("UQ_2e303c3a712662f1fc2a4d0aad6")
            .table(Font::Table)
            .col(Font::Name)
            .to_string(MysqlQueryBuilder),
        [
            r#"ALTER TABLE `font` ADD CONSTRAINT `UQ_2e303c3a712662f1fc2a4d0aad6`"#,
            r#"UNIQUE KEY (`name`)"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_3() {
    assert_eq!(
        Constraint::create()
            .unique()
            .constraint_name("UQ_2e303c3a712662f1fc2a4d0aad6")
            .index_name("idx_2e303c3a712662f1fc2a4d0aad6")
            .table(Font::Table)
            .col(Font::Name)
            .to_string(MysqlQueryBuilder),
        [
            r#"ALTER TABLE `font` ADD CONSTRAINT `UQ_2e303c3a712662f1fc2a4d0aad6`"#,
            r#"UNIQUE KEY `idx_2e303c3a712662f1fc2a4d0aad6` (`name`)"#,
        ]
        .join(" ")
    );
}

#[test]
fn create_4() {
    assert_eq!(
        Constraint::create()
            .check(("id_range", Expr::col(Glyph::Id).lt(20)))
            .table(Glyph::Table)
            .to_string(MysqlQueryBuilder),
        [r#"ALTER TABLE `glyph` ADD CONSTRAINT `id_range` CHECK (`id` < 20)"#,].join(" ")
    );
}

#[test]
fn create_5() {
    assert_eq!(
        Constraint::create()
            .check(Expr::col(Glyph::Id).lt(20))
            .table(Glyph::Table)
            .to_string(MysqlQueryBuilder),
        [r#"ALTER TABLE `glyph` ADD CHECK (`id` < 20)"#,].join(" ")
    );
}
