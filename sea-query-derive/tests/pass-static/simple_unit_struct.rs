use sea_query::{Iden, IntoIden, IdenStatic, MysqlQueryBuilder, PostgresQueryBuilder, QuotedBuilder};

#[derive(Copy, Clone, IdenStatic)]
pub struct SomeType;

#[derive(Copy, Clone, IdenStatic)]
#[iden(rename = "Hel`lo")]
pub struct SomeTypeWithRename;

fn main() {
    assert_eq!(SomeType.to_string(), "some_type");
    assert_eq!(SomeTypeWithRename.to_string(), "Hel`lo");

    let mut string = String::new();
    PostgresQueryBuilder.prepare_dyn_iden(&SomeType.into_iden(), &mut string);
    assert_eq!(string, "\"some_type\"");

    let mut string = String::new();
    PostgresQueryBuilder.prepare_dyn_iden(&SomeTypeWithRename.into_iden(), &mut string);
    assert_eq!(string, "\"Hel`lo\"");

    let mut string = String::new();
    MysqlQueryBuilder.prepare_dyn_iden(&SomeTypeWithRename.into_iden(), &mut string);
    assert_eq!(string, "`Hel``lo`");
}
