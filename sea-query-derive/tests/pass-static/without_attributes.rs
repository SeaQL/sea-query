use sea_query::{Iden, IdenStatic};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Copy, Clone, IdenStatic, EnumIter)]
enum User {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
}

#[derive(Copy, Clone, IdenStatic, EnumIter)]
enum UserStatic {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
}

fn main() {
    let expected = ["user", "id", "first_name", "last_name", "email"];
    User::iter().zip(expected).for_each(|(var, exp)| {
        assert_eq!(var.to_string(), exp);
        assert_eq!(var.as_str(), exp)
    });
}
