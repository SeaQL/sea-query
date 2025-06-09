use sea_query::IdenStatic;
use sea_query_derive::enum_def;
use std::convert::AsRef;

#[enum_def(table_name = "HelloTable")]
pub struct Hello {
    pub name: String,
}

fn main() {
    assert_eq!("HelloTable".to_string(), HelloIden::Table.to_string());
    assert_eq!("name".to_string(), HelloIden::Name.to_string());

    assert_eq!("HelloTable", HelloIden::Table.as_str());
    assert_eq!("name", HelloIden::Name.as_str());

    assert_eq!("HelloTable", AsRef::<str>::as_ref(&HelloIden::Table));
    assert_eq!("name", AsRef::<str>::as_ref(&HelloIden::Name));
}
