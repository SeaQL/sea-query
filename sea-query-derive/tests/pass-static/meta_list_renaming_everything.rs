use sea_query::{Iden, IdenStatic};
use strum::{EnumIter, IntoEnumIterator};

#[derive(IdenStatic, EnumIter, Copy, Clone)]
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
    Email(i32),
    // Custom method
    #[iden(method = "custom_to_string")]
    Custom,
}

impl Custom {
    pub fn custom_to_string(&self) -> &'static str {
        match self {
            Self::Custom => "custom",
            _ => panic!("not Custom::Custom"),
        }
    }
}

fn main() {
    // custom ends up being default string which is an empty string
    let expected = ["user", "my_id", "name", "surname", "EM`ail", "custom"];
    Custom::iter()
        .map(|var| var.to_string())
        .zip(expected)
        .for_each(|(iden, exp)| assert_eq!(iden, exp));
    
    let mut string = String::new();
    Custom::Email(0).prepare(&mut string, '"'.into());
    assert_eq!(string, "\"EM`ail\"");

    let mut string = String::new();
    Custom::Email(0).prepare(&mut string, b'`'.into());
    assert_eq!(string, "`EM``ail`");
}
