use chrono::{NaiveDate, NaiveDateTime};
use diesel::backend::Backend;
use diesel::connection::SimpleConnection;
use diesel::deserialize::{self, FromSql};
use diesel::sql_types::BigInt;
use diesel::sql_types::{Blob, Text};
use diesel::{Connection, QueryableByName, RunQueryDsl, SqliteConnection};
use sea_query::{Alias, ColumnDef, Expr, Func, Iden, Order, Query, SqliteQueryBuilder, Table};
use sea_query_diesel::DieselBinder;
use serde_json::json;
use time::macros::{date, time};
use time::PrimitiveDateTime;
use uuid::Uuid;

// NOTE: Until https://github.com/diesel-rs/diesel/issues/3693 is fixed, we can't mix and match
// values from time and chrono, just be mindful of that!
fn main() {
    let conn = &mut SqliteConnection::establish(":memory:").unwrap();

    // Schema

    let sql = [
        Table::drop()
            .table(Character::Table)
            .if_exists()
            .build(SqliteQueryBuilder),
        Table::create()
            .table(Character::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Character::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Character::Uuid).uuid().not_null())
            .col(ColumnDef::new(Character::FontSize).integer().not_null())
            .col(ColumnDef::new(Character::Name).string().not_null())
            .col(ColumnDef::new(Character::Meta).json().not_null())
            .col(ColumnDef::new(Character::Created).date_time())
            .build(SqliteQueryBuilder),
    ]
    .join("; ");

    let result = conn.batch_execute(&sql);
    println!("Create table character: {result:?}");
    println!();

    // Create

    let query = Query::insert()
        .into_table(Character::Table)
        .columns([
            Character::Uuid,
            Character::FontSize,
            Character::Name,
            Character::Meta,
            Character::Created,
        ])
        .values_panic([
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            None::<NaiveDate>.into(),
        ])
        .values_panic([
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            Some(
                NaiveDate::from_ymd_opt(2020, 1, 1)
                    .unwrap()
                    .and_hms_opt(2, 2, 2)
                    .unwrap(),
            )
            .into(),
        ])
        .values_panic([
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            None::<PrimitiveDateTime>.into(),
        ])
        .values_panic([
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            Some(date!(2020 - 1 - 1).with_time(time!(2:2:2))).into(),
        ])
        .to_owned();

    let result = query.build_diesel().unwrap().execute(conn);
    println!("Insert into character {result:?}\n");

    // Read

    let query = Query::select()
        .columns([
            Character::Id,
            Character::Uuid,
            Character::FontSize,
            Character::Name,
            Character::Meta,
            Character::Created,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .to_owned();

    // let rows = query
    //     .build_diesel()
    //     .unwrap()
    //     .get_results::<CharacterStructChrono>(conn)
    //     .unwrap();
    // println!("Select one from character:");
    // for row in rows.iter() {
    //     println!("{row:?}\n");
    // }
    let rows = query
        .build_diesel()
        .unwrap()
        .get_results::<CharacterStructTime>(conn)
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{row:?}\n");
    }
    println!();

    // Update

    let query = Query::update()
        .table(Character::Table)
        .values([(Character::FontSize, 24.into())])
        .and_where(Expr::col(Character::Id).eq(4))
        .to_owned();

    let result = query.build_diesel().unwrap().execute(conn);
    println!("Update character: {result:?}\n");

    // Read

    let query = Query::select()
        .columns([
            Character::Id,
            Character::Uuid,
            Character::FontSize,
            Character::Name,
            Character::Meta,
            Character::Created,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .to_owned();

    // let rows = query
    //     .build_diesel()
    //     .unwrap()
    //     .get_results::<CharacterStructChrono>(conn)
    //     .unwrap();
    // println!("Select one from character:");
    // for row in rows.iter() {
    //     println!("{row:?}\n");
    // }
    let rows = query
        .build_diesel()
        .unwrap()
        .get_results::<CharacterStructTime>(conn)
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{row:?}\n");
    }
    println!();

    // Count

    let query = Query::select()
        .from(Character::Table)
        .expr_as(Func::count(Expr::col(Character::Id)), Alias::new("count"))
        .to_owned();

    print!("Count character: ");
    let count = query.build_diesel().unwrap().get_result::<CountField>(conn);
    println!("{count:?}");
    println!();

    // Delete

    let query = Query::delete()
        .from_table(Character::Table)
        .and_where(Expr::col(Character::Id).eq(4))
        .to_owned();

    let result = query.build_diesel().unwrap().execute(conn);
    println!("Delete character: {result:?}");
}

#[derive(Iden)]
enum Character {
    Table,
    Id,
    Uuid,
    Name,
    FontSize,
    Meta,
    Created,
}

#[derive(QueryableByName, Debug)]
#[diesel(table_name = character)]
#[allow(dead_code)]
struct CharacterStructChrono {
    id: i32,
    uuid: UUID,
    name: String,
    font_size: i32,
    meta: Json,
    created: Option<NaiveDateTime>,
}

#[derive(QueryableByName, Debug)]
#[diesel(table_name = character)]
#[allow(dead_code)]
struct CharacterStructTime {
    id: i32,
    uuid: UUID,
    name: String,
    font_size: i32,
    meta: Json,
    created: Option<PrimitiveDateTime>,
}

#[derive(QueryableByName, Debug)]
#[allow(dead_code)]
struct CountField {
    #[diesel(sql_type = BigInt)]
    count: i64,
}

#[derive(Debug)]
struct Json(serde_json::Value);

impl<DB> FromSql<Text, DB> for Json
where
    DB: Backend,
    *const str: deserialize::FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let raw = <*const str as FromSql<Text, DB>>::from_sql(bytes)?;
        let string = unsafe { &*raw }; // We know that the pointer impl will never return null
        let value = serde_json::from_str(string)?;
        Ok(Self(value))
    }
}

#[derive(Debug)]
#[warn(clippy::upper_case_acronyms)]
struct UUID(uuid::Uuid);

impl<DB> FromSql<Blob, DB> for UUID
where
    DB: Backend,
    *const [u8]: deserialize::FromSql<Blob, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let raw = <*const [u8] as FromSql<Blob, DB>>::from_sql(bytes)?;
        let slice = unsafe { &*raw }; // We know that the pointer impl will never return null
        let value = uuid::Uuid::from_slice(slice)?;
        Ok(Self(value))
    }
}

diesel::table! {
    character (id) {
        id -> Integer,
        uuid -> Blob,
        name -> Text,
        font_size -> Integer,
        meta -> Text,
        created -> Nullable<TimestamptzSqlite>,
    }
}
