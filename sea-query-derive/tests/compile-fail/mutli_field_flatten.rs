use sea_query::Iden;

#[derive(Clone, Iden)]
enum Asset {
    Table,
    Id,
    AssetName,
    #[iden(flatten)]
    Creation(CreationInfo, CreationInfo),
}

#[derive(Clone, Iden)]
enum CreationInfo {
    UserId,
    #[iden = "creation_date"]
    Date,
}

fn main() {}
