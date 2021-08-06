use sea_query::Iden;

#[derive(Iden)]
enum Asset {
    Table,
    Id,
    AssetName,
    #[iden()]
    Creation(CreationInfo),
}

#[derive(Iden)]
enum CreationInfo {
    UserId,
    #[iden = "creation_date"]
    Date,
}

fn main() {}
