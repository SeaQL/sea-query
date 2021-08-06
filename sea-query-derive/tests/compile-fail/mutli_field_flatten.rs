use sea_query::Iden;

#[derive(Iden)]
enum Asset {
    Table,
    Id,
    AssetName,
    #[iden(flatten)]
    Creation(CreationInfo, CreationInfo),
}

#[derive(Iden)]
enum CreationInfo {
    UserId,
    #[iden = "creation_date"]
    Date,
}

fn main() {}
