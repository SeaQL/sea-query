use sea_query::Iden;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Iden, EnumIter)]
enum Asset {
    Table,
    Id,
    AssetName,
    #[iden(flatten)]
    Creation(CreationInfo),
}

#[derive(Iden)]
enum CreationInfo {
    UserId,
    #[iden = "creation_date"]
    Date,
}

impl Default for CreationInfo {
    fn default() -> Self {
        Self::UserId
    }
}

fn main() {
    // custom ends up being default string which is an empty string
    let expected = ["asset", "id", "asset_name", "user_id", "creation_date"];
    Asset::iter()
        .chain(std::iter::once(Asset::Creation(CreationInfo::Date)))
        .map(|var| Iden::to_string(&var))
        .zip(expected)
        .for_each(|(iden, exp)| assert_eq!(iden, exp))
}
