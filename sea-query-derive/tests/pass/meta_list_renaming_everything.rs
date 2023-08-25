use sea_query::Iden;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Iden, EnumIter)]
// Outer iden attributes overrides what's used for "Table"...
#[iden(rename = "user")]
enum Custom {
    Table,
    #[iden(rename = "my_id")]
    Id,
    #[iden(rename = "name")]
    FirstName,
    #[iden(rename = "surname")]
    LastName,
    // Custom casing if needed
    #[iden(rename = "EM`ail")]
    // the tuple value will be ignored
    Email(String),
    // Custom method
    #[iden(method = "custom_to_string")]
    Custom(String),
}

impl Custom {
    pub fn custom_to_string(&self) -> &str {
        match self {
            Self::Custom(custom) => custom,
            _ => panic!("not Custom::Custom"),
        }
    }
}

fn main() {
    // custom ends up being default string which is an empty string
    let expected = ["user", "my_id", "name", "surname", "EM`ail", ""];
    Custom::iter()
        .map(|var| var.to_string())
        .zip(expected)
        .for_each(|(iden, exp)| assert_eq!(iden, exp));
    
    let mut string = String::new();
    Custom::Email("".to_owned()).prepare(&mut string, '"'.into());
    assert_eq!(string, "\"EM`ail\"");

    let mut string = String::new();
    Custom::Email("".to_owned()).prepare(&mut string, b'`'.into());
    assert_eq!(string, "`EM``ail`");
}
