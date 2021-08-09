use sea_query::Iden;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Iden, EnumIter)]
enum Asset {
    Table,
    Id,
    AssetName,
    #[iden(flatten)]
    First {
        first: FirstLevel,
    },
}

#[derive(Iden)]
enum FirstLevel {
    LevelOne,
    #[iden(flatten)]
    Second(SecondLevel),
}

#[derive(Iden, EnumIter)]
enum SecondLevel {
    LevelTwo,
    #[iden(flatten)]
    Third(LevelThree),
    UserId,
}

#[derive(Iden, Default)]
struct LevelThree;

impl Default for FirstLevel {
    fn default() -> Self {
        Self::LevelOne
    }
}

fn main() {
    // custom ends up being default string which is an empty string
    let expected = [
        "asset",
        "id",
        "asset_name",
        "level_one",
        "level_two",
        "level_three",
        "user_id",
    ];
    Asset::iter()
        .chain(
            SecondLevel::iter()
                .map(FirstLevel::Second)
                .map(|s| Asset::First { first: s }),
        )
        .map(|var| Iden::to_string(&var))
        .zip(expected)
        .for_each(|(iden, exp)| assert_eq!(iden, exp))
}
