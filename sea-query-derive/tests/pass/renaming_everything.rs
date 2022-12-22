use sea_query::Iden;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Iden, EnumIter)]
// Outer iden attributes overrides what's used for "Table"...
#[iden = "user"]
enum Custom {
    Table,
    #[iden = "my_id"]
    Id,
    #[iden = "name"]
    FirstName,
    #[iden = "surname"]
    LastName,
    // Custom casing if needed
    #[iden = "EMail"]
    // the tuple value will be ignored
    Email(String),
    // Custom method
    #[method = "custom_to_string"]
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
    let expected = ["user", "my_id", "name", "surname", "EMail", ""];
    Custom::iter().zip(expected).for_each(|(var, exp)| {
        assert_eq!(var.to_string(), exp);
    })
}
