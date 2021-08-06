use sea_query::Iden;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Iden, EnumIter)]
enum Something {
    // ...the Table can also be overwritten like this
    #[iden = "something_else"]
    Table,
    Id,
    AssetName,
    UserId,
}

fn main() {
    let expected = ["something_else", "id", "asset_name", "user_id"];
    Something::iter()
        .map(|var| Iden::to_string(&var))
        .zip(expected)
        .for_each(|(iden, exp)| assert_eq!(iden, exp))
}
