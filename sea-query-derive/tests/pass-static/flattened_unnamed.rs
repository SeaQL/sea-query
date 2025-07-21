use sea_query::{Iden, IdenStatic};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Copy, Clone, IdenStatic, EnumIter)]
enum Asset {
    Table,
    Id,
    AssetName,
    #[iden(flatten)]
    Creation(CreationInfo),
}

#[derive(Default, Copy, Clone, IdenStatic)]
enum CreationInfo {
    #[default]
    UserId,
    #[iden = "creation_date"]
    Date,
}

fn main() {
    // custom ends up being default string which is an empty string
    let expected = ["asset", "id", "asset_name", "user_id", "creation_date"];
    Asset::iter()
        .chain(std::iter::once(Asset::Creation(CreationInfo::Date)))
        .zip(expected)
        .for_each(|(var, exp)| {
            assert_eq!(var.to_string(), exp);
            assert_eq!(var.as_str(), exp)
        })
}
