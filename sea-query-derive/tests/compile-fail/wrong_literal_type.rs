use sea_query::Iden;

#[derive(Iden)]
enum User {
    Table,
    #[iden = 123]
    Id,
    FirstName,
    LastName,
    Email,
}

fn main() {}
