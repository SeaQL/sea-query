use sea_query::{error::*, tests_cfg::*, *};

#[test]
fn insert_values_1() {
    let mut insert = Query::insert();
    let result = insert
        .into_table(Glyph::Table)
        .columns(vec![Glyph::Image, Glyph::Aspect])
        .values(vec![String::from("").into()]);

    assert_eq!(result.is_err(), true);
    assert_eq!(
        result.unwrap_err(),
        Error::ColValNumMismatch {
            col_len: 2,
            val_len: 1,
        }
    );
}
