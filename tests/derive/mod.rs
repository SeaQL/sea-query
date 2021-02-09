use super::*;

#[test]
fn derive_1() {
    #[derive(Iden)]
    enum User {
        Table,
        Id,
        FirstName,
        LastName,
        Email,
    }

    println!("Default field names");
    assert_eq!(Iden::to_string(&User::Table), "user");
    assert_eq!(Iden::to_string(&User::Id), "id");
    assert_eq!(Iden::to_string(&User::FirstName), "first_name");
    assert_eq!(Iden::to_string(&User::LastName), "last_name");
    assert_eq!(Iden::to_string(&User::Email), "email");
}

#[test]
fn derive_2() {
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
        Email,
    }

    println!("Custom field names");
    assert_eq!(Iden::to_string(&Custom::Table), "user");
    assert_eq!(Iden::to_string(&Custom::Id), "my_id");
    assert_eq!(Iden::to_string(&Custom::FirstName), "name");
    assert_eq!(Iden::to_string(&Custom::LastName), "surname");
    assert_eq!(Iden::to_string(&Custom::Email), "EMail");
}

#[test]
fn derive_3() {
    #[derive(Iden)]
    enum Something {
        // ...the Table can also be overwritten like this
        #[iden = "something_else"]
        Table,
        Id,
        AssetName,
        UserId,
    }

    println!("Single custom field name");
    assert_eq!(Iden::to_string(&Something::Table), "something_else");
    assert_eq!(Iden::to_string(&Something::Id), "id");
    assert_eq!(Iden::to_string(&Something::AssetName), "asset_name");
    assert_eq!(Iden::to_string(&Something::UserId), "user_id");
}

#[test]
fn derive_4() {
    #[derive(Iden)]
    pub struct SomeType;

    #[derive(Iden)]
    #[iden = "another_name"]
    pub struct CustomName;

    println!("Unit structs");
    assert_eq!(Iden::to_string(&SomeType), "some_type");
    assert_eq!(Iden::to_string(&CustomName), "another_name");
}