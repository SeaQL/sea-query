use crate::{tests_cfg::*, *};
use Character as CharReexport;
use pretty_assertions::assert_eq;

#[test]
fn test_identifier() {
    let query = Query::select().column("hello-World_").to_owned();

    #[cfg(feature = "backend-mysql")]
    assert_eq!(query.to_string(MysqlQueryBuilder), r"SELECT `hello-World_`");
    #[cfg(feature = "backend-postgres")]
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "hello-World_""#
    );
    #[cfg(feature = "backend-sqlite")]
    assert_eq!(
        query.to_string(SqliteQueryBuilder),
        r#"SELECT "hello-World_""#
    );
}

#[test]
fn test_quoted_identifier_1() {
    let query = Query::select().column("hel`lo").to_owned();

    #[cfg(feature = "backend-mysql")]
    assert_eq!(query.to_string(MysqlQueryBuilder), r"SELECT `hel``lo`");
    #[cfg(feature = "backend-sqlite")]
    assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT "hel`lo""#);

    let query = Query::select().column("hel\"lo").to_owned();

    #[cfg(feature = "backend-postgres")]
    assert_eq!(query.to_string(PostgresQueryBuilder), r#"SELECT "hel""lo""#);
}

#[test]
fn test_quoted_identifier_2() {
    let query = Query::select().column("hel``lo").to_owned();

    #[cfg(feature = "backend-mysql")]
    assert_eq!(query.to_string(MysqlQueryBuilder), r"SELECT `hel````lo`");
    #[cfg(feature = "backend-sqlite")]
    assert_eq!(query.to_string(SqliteQueryBuilder), r#"SELECT "hel``lo""#);

    let query = Query::select().column("hel\"\"lo").to_owned();

    #[cfg(feature = "backend-postgres")]
    assert_eq!(
        query.to_string(PostgresQueryBuilder),
        r#"SELECT "hel""""lo""#
    );
}

#[test]
fn test_cmp_identifier() {
    type CharLocal = Character;

    assert_eq!(
        ColumnRef::Column(Character::Id.into()),
        ColumnRef::Column(Character::Id.into())
    );
    assert_eq!(
        ColumnRef::Column(Character::Id.into()),
        ColumnRef::Column(Char::Id.into())
    );
    assert_eq!(
        ColumnRef::Column(Character::Id.into()),
        ColumnRef::Column(CharLocal::Id.into())
    );
    assert_eq!(
        ColumnRef::Column(Character::Id.into()),
        ColumnRef::Column(CharReexport::Id.into())
    );
    assert_eq!(
        ColumnRef::Column("id".into()),
        ColumnRef::Column("id".into())
    );
    assert_ne!(
        ColumnRef::Column("id".into()),
        ColumnRef::Column("id_".into())
    );
    assert_eq!(
        ColumnRef::Column(Character::Id.into()),
        ColumnRef::Column("id".into())
    );
    assert_ne!(
        ColumnRef::Column(Character::Id.into()),
        ColumnRef::Column(Character::Table.into())
    );
    assert_eq!(
        ColumnRef::Column(Character::Id.into()),
        ColumnRef::Column(Font::Id.into())
    );
}
