use sea_query::Iden;

#[derive(Copy, Clone, Iden)]
pub struct SomeType;

#[derive(Copy, Clone, Iden)]
#[iden(rename = "Hel`lo")]
pub struct SomeTypeWithRename;

fn main() {
    assert_eq!(SomeType.to_string(), "some_type");
    assert_eq!(SomeTypeWithRename.to_string(), "Hel`lo");

    let mut string = String::new();
    SomeType.prepare(&mut string, '"'.into());
    assert_eq!(string, "\"some_type\"");

    let mut string = String::new();
    SomeTypeWithRename.prepare(&mut string, '"'.into());
    assert_eq!(string, "\"Hel`lo\"");

    let mut string = String::new();
    SomeTypeWithRename.prepare(&mut string, b'`'.into());
    assert_eq!(string, "`Hel``lo`");
}
