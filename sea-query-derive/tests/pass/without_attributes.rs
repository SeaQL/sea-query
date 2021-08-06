use sea_query::{Iden, IntoIden};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Iden, EnumIter)]
enum User {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
}

fn main() {
    let expected = ["user", "id", "first_name", "last_name", "email"];
    User::iter()
        .map(Iden::to_string)
        .zip(expected)
        .for_each(|(iden, exp)| assert_eq!(iden, exp))
}
