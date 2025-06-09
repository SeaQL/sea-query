use sea_query::Iden;
use sea_query_derive::enum_def;
use std::convert::AsRef;

#[enum_def(table_name = "HelloTable")]
pub struct Hello {
    pub name: String,
}

fn main() {
    assert_eq!(
        "HelloTable".to_string(),
        Iden::from(HelloIden::Table).to_string()
    );
    assert_eq!("name".to_string(), Iden::from(HelloIden::Name).to_string());

    assert_eq!("HelloTable", AsRef::<str>::as_ref(&HelloIden::Table));
    assert_eq!("name", AsRef::<str>::as_ref(&HelloIden::Name));
}
