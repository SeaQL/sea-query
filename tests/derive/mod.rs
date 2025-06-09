use sea_query::*;

#[test]
fn derive_1() {
    #[derive(Debug, Iden)]
    enum User {
        Table,
        Id,
        FirstName,
        LastName,
        Email,
    }

    println!("Default field names");
    assert_eq!(Iden::from(&User::Table).to_string(), "user");
    assert_eq!(Iden::from(&User::Id).to_string(), "id");
    assert_eq!(Iden::from(&User::FirstName).to_string(), "first_name");
    assert_eq!(Iden::from(&User::LastName).to_string(), "last_name");
    assert_eq!(Iden::from(&User::Email).to_string(), "email");
}

#[test]
fn derive_2() {
    #[derive(Debug, Iden)]
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
        Email,
    }

    println!("Custom field names");
    assert_eq!(Iden::from(&Custom::Table).to_string(), "user");
    assert_eq!(Iden::from(&Custom::Id).to_string(), "my_id");
    assert_eq!(Iden::from(&Custom::FirstName).to_string(), "name");
    assert_eq!(Iden::from(&Custom::LastName).to_string(), "surname");
    assert_eq!(Iden::from(&Custom::Email).to_string(), "EMail");
}

#[test]
fn derive_3() {
    #[derive(Debug, Iden)]
    enum Something {
        // ...the Table can also be overwritten like this
        #[iden = "something_else"]
        Table,
        Id,
        AssetName,
        UserId,
    }

    println!("Single custom field name");
    assert_eq!(Iden::from(&Something::Table).to_string(), "something_else");
    assert_eq!(Iden::from(&Something::Id).to_string(), "id");
    assert_eq!(Iden::from(&Something::AssetName).to_string(), "asset_name");
    assert_eq!(Iden::from(&Something::UserId).to_string(), "user_id");
}

#[test]
fn derive_4() {
    #[derive(Debug, Iden)]
    pub struct SomeType;

    #[derive(Debug, Iden)]
    #[iden = "another_name"]
    pub struct CustomName;

    println!("Unit structs");
    assert_eq!(Iden::from(&SomeType).to_string(), "some_type");
    assert_eq!(Iden::from(&CustomName).to_string(), "another_name");
}
