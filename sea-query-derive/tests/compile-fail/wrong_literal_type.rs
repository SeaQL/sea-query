use sea_query::Iden;

#[derive(Clone, Iden)]
enum User {
    Table,
    #[iden = 123]
    Id,
    FirstName,
    LastName,
    Email,
}

fn main() {}
