use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::backend::Backend;
use diesel::connection::SimpleConnection;
use diesel::deserialize::{self, FromSql};
use diesel::pg::sql_types::MacAddr;
use diesel::sql_types::BigInt;
use diesel::{Connection, PgConnection, QueryableByName, RunQueryDsl};
use ipnetwork::IpNetwork;
use mac_address::get_mac_address;
use rust_decimal::Decimal;
use sea_query::{
    ColumnDef, ColumnType, Expr, ExprTrait, Func, Iden, OnConflict, Order, PostgresQueryBuilder,
    Query, Table,
};
use sea_query_diesel::DieselBinder;
use serde_json::{Value as Json, json};
use std::net::{IpAddr, Ipv4Addr};
use time::{
    PrimitiveDateTime,
    macros::{date, time},
};
use uuid::Uuid;

fn main() {
    let conn = &mut PgConnection::establish("postgres://sea:sea@127.0.0.1/query").unwrap();

    // Schema

    let sql = [
        Table::drop()
            .table(Character::Table)
            .if_exists()
            .build(PostgresQueryBuilder),
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
            .col(ColumnDef::new(Character::Uuid).uuid())
            .col(ColumnDef::new(Character::FontSize).integer())
            .col(ColumnDef::new(Character::Name).string())
            .col(ColumnDef::new(Character::Meta).json())
            .col(ColumnDef::new(Character::Decimal).decimal())
            .col(ColumnDef::new(Character::BigDecimal).decimal())
            .col(ColumnDef::new(Character::Created).date_time())
            .col(ColumnDef::new(Character::Inet).inet())
            .col(ColumnDef::new(Character::MacAddress).mac_address())
            .col(ColumnDef::new(Character::ArrayBool).array(ColumnType::Boolean))
            .build(PostgresQueryBuilder),
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
            Character::Decimal,
            Character::BigDecimal,
            Character::Created,
            Character::Inet,
            Character::MacAddress,
            Character::ArrayBool,
        ])
        .values_panic([
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            Decimal::from_i128_with_scale(3141i128, 3).into(),
            BigDecimal::from_i128(3141i128)
                .unwrap()
                .with_scale(3)
                .into(),
            NaiveDate::from_ymd_opt(2020, 8, 20)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .into(),
            IpNetwork::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8)
                .unwrap()
                .into(),
            get_mac_address().unwrap().unwrap().into(),
            vec![true, false].into(),
        ])
        .values_panic([
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            Decimal::from_i128_with_scale(3141i128, 3).into(),
            BigDecimal::from_i128(3141i128)
                .unwrap()
                .with_scale(3)
                .into(),
            date!(2020 - 8 - 20).with_time(time!(0:0:0)).into(),
            IpNetwork::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8)
                .unwrap()
                .into(),
            get_mac_address().unwrap().unwrap().into(),
            vec![false, false].into(),
        ])
        .returning_col(Character::Id)
        .to_owned();

    let result = query.build_diesel().unwrap().execute(conn);
    println!("Insert into character {result:?}\n");

    // Read

    let query = Query::select()
        .columns([
            Character::Id,
            Character::Uuid,
            Character::Name,
            Character::FontSize,
            Character::Meta,
            Character::Decimal,
            Character::BigDecimal,
            Character::Created,
            Character::Inet,
            Character::MacAddress,
            Character::ArrayBool,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .to_owned();

    let rows = query
        .build_diesel()
        .unwrap()
        .get_results::<CharacterStructChrono>(conn)
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{row:?}\n");
    }
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
        .and_where(Expr::col(Character::Id).eq(2))
        .to_owned();

    let result = query.build_diesel().unwrap().execute(conn);
    println!("Update character: {result:?}\n");

    // Read

    let query = Query::select()
        .columns([
            Character::Id,
            Character::Uuid,
            Character::Name,
            Character::FontSize,
            Character::Meta,
            Character::Decimal,
            Character::BigDecimal,
            Character::Created,
            Character::Inet,
            Character::MacAddress,
            Character::ArrayBool,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .to_owned();

    let rows = query
        .build_diesel()
        .unwrap()
        .get_results::<CharacterStructChrono>(conn)
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{row:?}\n");
    }
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
        .expr(Func::count(Expr::col(Character::Id)))
        .to_owned();

    print!("Count character: ");
    let count = query.build_diesel().unwrap().get_result::<CountField>(conn);
    println!("{count:?}");
    println!();

    // Upsert

    let query = Query::insert()
        .into_table(Character::Table)
        .columns([Character::Id, Character::FontSize, Character::Name])
        .values_panic([1.into(), 16.into(), "B".into()])
        .values_panic([2.into(), 24.into(), "C".into()])
        .on_conflict(
            OnConflict::column(Character::Id)
                .update_columns([Character::FontSize, Character::Name])
                .to_owned(),
        )
        .to_owned();

    let result = query.build_diesel().unwrap().execute(conn);
    println!("Insert into character (with upsert): {result:?}\n");

    // Read

    let query = Query::select()
        .columns([
            Character::Id,
            Character::Uuid,
            Character::Name,
            Character::FontSize,
            Character::Meta,
            Character::Decimal,
            Character::BigDecimal,
            Character::Created,
            Character::Inet,
            Character::MacAddress,
            Character::ArrayBool,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .to_owned();

    let rows = query
        .build_diesel()
        .unwrap()
        .get_results::<CharacterStructChrono>(conn)
        .unwrap();
    println!("Select all characters:");
    for row in rows.iter() {
        println!("{row:?}\n");
    }
    let rows = query
        .build_diesel()
        .unwrap()
        .get_results::<CharacterStructTime>(conn)
        .unwrap();
    println!("Select all characters:");
    for row in rows.iter() {
        println!("{row:?}\n");
    }
    println!();

    // Delete

    let query = Query::delete()
        .from_table(Character::Table)
        .and_where(Expr::col(Character::Id).eq(1))
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
    Decimal,
    BigDecimal,
    Created,
    Inet,
    MacAddress,
    ArrayBool,
}

#[derive(QueryableByName, Debug)]
#[diesel(table_name = character)]
#[allow(dead_code)]
struct CharacterStructChrono {
    id: i32,
    uuid: Uuid,
    name: String,
    font_size: i32,
    meta: Json,
    decimal: Decimal,
    big_decimal: BigDecimal,
    created: NaiveDateTime,
    inet: IpNetwork,
    mac_address: MacAddress,
    array_bool: Vec<bool>,
}

#[derive(QueryableByName, Debug)]
#[diesel(table_name = character)]
#[allow(dead_code)]
struct CharacterStructTime {
    id: i32,
    uuid: Uuid,
    name: String,
    font_size: i32,
    meta: Json,
    decimal: Decimal,
    big_decimal: BigDecimal,
    created: PrimitiveDateTime,
    inet: IpNetwork,
    mac_address: MacAddress,
    array_bool: Vec<bool>,
}

#[derive(QueryableByName, Debug)]
#[allow(dead_code)]
struct CountField {
    #[diesel(sql_type = BigInt)]
    count: i64,
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
struct MacAddress(mac_address::MacAddress);

impl<DB> FromSql<MacAddr, DB> for MacAddress
where
    DB: Backend,
    [u8; 6]: FromSql<MacAddr, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let slice = <[u8; 6] as FromSql<MacAddr, DB>>::from_sql(bytes)?;
        let value = mac_address::MacAddress::new(slice);
        Ok(Self(value))
    }
}

diesel::table! {
    character (id) {
        id -> Integer,
        uuid -> Uuid,
        name -> Text,
        font_size -> Integer,
        meta -> Json,
        decimal -> Numeric,
        big_decimal -> Numeric,
        created -> Timestamp,
        inet -> Inet,
        mac_address -> MacAddr,
        array_bool -> Array<Bool>,
    }
}
