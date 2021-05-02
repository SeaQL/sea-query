use sea_query::Iden;

#[derive(Iden)]
enum User {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
}

#[derive(Iden)]
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

#[derive(Iden)]
enum Something {
    // ...the Table can also be overwritten like this
    #[iden = "something_else"]
    Table,
    Id,
    AssetName,
    UserId,
}

#[derive(Iden)]
pub struct SomeType;
#[derive(Iden)]
#[iden = "another_name"]
pub struct CustomName;

fn main() {
    println!("Default field names");
    assert_eq!(dbg!(Iden::to_string(&User::Table)), "user");
    assert_eq!(dbg!(Iden::to_string(&User::Id)), "id");
    assert_eq!(dbg!(Iden::to_string(&User::FirstName)), "first_name");
    assert_eq!(dbg!(Iden::to_string(&User::LastName)), "last_name");
    assert_eq!(dbg!(Iden::to_string(&User::Email)), "email");

    println!("Custom field names");
    assert_eq!(dbg!(Iden::to_string(&Custom::Table)), "user");
    assert_eq!(dbg!(Iden::to_string(&Custom::Id)), "my_id");
    assert_eq!(dbg!(Iden::to_string(&Custom::FirstName)), "name");
    assert_eq!(dbg!(Iden::to_string(&Custom::LastName)), "surname");
    assert_eq!(dbg!(Iden::to_string(&Custom::Email("chris@gmail.com".to_owned()))), "EMail");
    assert_eq!(dbg!(Iden::to_string(&Custom::Custom("hello".to_owned()))), "hello");

    println!("Single custom field name");
    assert_eq!(dbg!(Iden::to_string(&Something::Table)), "something_else");
    assert_eq!(dbg!(Iden::to_string(&Something::Id)), "id");
    assert_eq!(dbg!(Iden::to_string(&Something::AssetName)), "asset_name");
    assert_eq!(dbg!(Iden::to_string(&Something::UserId)), "user_id");

    println!("Unit structs");
    assert_eq!(dbg!(Iden::to_string(&SomeType)), "some_type");
    assert_eq!(dbg!(Iden::to_string(&CustomName)), "another_name");
}
