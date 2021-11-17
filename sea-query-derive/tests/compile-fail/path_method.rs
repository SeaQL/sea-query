use sea_query::Iden;

#[derive(Clone, Iden)]
enum Asset {
    Table,
    Id,
    AssetName,
    #[method]
    Creation,
}

fn main() {}
