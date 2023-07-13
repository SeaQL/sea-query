use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use sea_query::{
    ColumnDef, Expr, Func, Iden, OnConflict, Order, PostgresQueryBuilder, Query, Table,
};
use sea_query_binder::SqlxBinder;
use sqlx::{PgPool, Row};
use std::net::{IpAddr, Ipv4Addr};
use time::{
    macros::{date, time},
    PrimitiveDateTime,
};

use ipnetwork::IpNetwork;
use mac_address::{get_mac_address, MacAddress};
use serde_json::{json, Value as Json};
use uuid::Uuid;

#[async_std::main]
async fn main() {
    let connection = PgPool::connect("postgres://sea:sea@127.0.0.1/query")
        .await
        .unwrap();
    let mut pool = connection.try_acquire().unwrap();

    // Schema

    let sql = Table::create()
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
        .col(ColumnDef::new(Character::Character).string())
        .col(ColumnDef::new(Character::Meta).json())
        .col(ColumnDef::new(Character::Decimal).decimal())
        .col(ColumnDef::new(Character::BigDecimal).decimal())
        .col(ColumnDef::new(Character::Created).date_time())
        .col(ColumnDef::new(Character::Inet).inet())
        .col(ColumnDef::new(Character::MacAddress).mac_address())
        .build(PostgresQueryBuilder);

    let result = sqlx::query(&sql).execute(&pool).await;
    println!("Create table character: {result:?}\n");

    // Create

    let (sql, values) = Query::insert()
        .into_table(Character::Table)
        .columns([
            Character::Uuid,
            Character::FontSize,
            Character::Character,
            Character::Meta,
            Character::Decimal,
            Character::BigDecimal,
            Character::Created,
            Character::Inet,
            Character::MacAddress,
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
        ])
        .returning_col(Character::Id)
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_with(&sql, values)
        .fetch_one(&pool)
        .await
        .unwrap();
    let id: i32 = row.try_get(0).unwrap();
    println!("Insert into character: last_insert_id = {id}\n");

    // Read

    let (sql, values) = Query::select()
        .columns([
            Character::Id,
            Character::Uuid,
            Character::Character,
            Character::FontSize,
            Character::Meta,
            Character::Decimal,
            Character::BigDecimal,
            Character::Created,
            Character::Inet,
            Character::MacAddress,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, CharacterStructChrono, _>(&sql, values.clone())
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{row:?}");
    }
    println!();

    let rows = sqlx::query_as_with::<_, CharacterStructTime, _>(&sql, values)
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{row:?}");
    }
    println!();

    // Update

    let (sql, values) = Query::update()
        .table(Character::Table)
        .values([(Character::FontSize, 24.into())])
        .and_where(Expr::col(Character::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_with(&sql, values).execute(&pool).await;
    println!("Update character: {result:?}\n");

    // Read

    let (sql, values) = Query::select()
        .columns([
            Character::Id,
            Character::Uuid,
            Character::Character,
            Character::FontSize,
            Character::Meta,
            Character::Decimal,
            Character::BigDecimal,
            Character::Created,
            Character::Inet,
            Character::MacAddress,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, CharacterStructChrono, _>(&sql, values.clone())
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{row:?}");
    }
    println!();

    let rows = sqlx::query_as_with::<_, CharacterStructTime, _>(&sql, values)
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{row:?}");
    }
    println!();

    // Count

    let (sql, values) = Query::select()
        .from(Character::Table)
        .expr(Func::count(Expr::col(Character::Id)))
        .build_sqlx(PostgresQueryBuilder);

    let row = sqlx::query_with(&sql, values)
        .fetch_one(&pool)
        .await
        .unwrap();
    print!("Count character: ");
    let count: i64 = row.try_get(0).unwrap();
    println!("{count}");
    println!();

    // Upsert

    let (sql, values) = Query::insert()
        .into_table(Character::Table)
        .columns([Character::Id, Character::FontSize, Character::Character])
        .values_panic([1.into(), 16.into(), "B".into()])
        .values_panic([2.into(), 24.into(), "C".into()])
        .on_conflict(
            OnConflict::column(Character::Id)
                .update_columns([Character::FontSize, Character::Character])
                .to_owned(),
        )
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_with(&sql, values).execute(&pool).await;
    println!("Insert into character (with upsert): {result:?}\n");

    // Read

    let (sql, values) = Query::select()
        .columns([
            Character::Id,
            Character::Uuid,
            Character::Character,
            Character::FontSize,
            Character::Meta,
            Character::Decimal,
            Character::BigDecimal,
            Character::Created,
            Character::Inet,
            Character::MacAddress,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .build_sqlx(PostgresQueryBuilder);

    let rows = sqlx::query_as_with::<_, CharacterStructChrono, _>(&sql, values.clone())
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select all characters:");
    for row in rows.iter() {
        println!("{row:?}");
    }
    println!();

    let rows = sqlx::query_as_with::<_, CharacterStructTime, _>(&sql, values)
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select all characters:");
    for row in rows.iter() {
        println!("{row:?}");
    }
    println!();

    // Delete

    let (sql, values) = Query::delete()
        .from_table(Character::Table)
        .and_where(Expr::col(Character::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_with(&sql, values).execute(&pool).await;
    println!("Delete character: {result:?}");
}

#[derive(Iden)]
enum Character {
    Table,
    Id,
    Uuid,
    Character,
    FontSize,
    Meta,
    Decimal,
    BigDecimal,
    Created,
    Inet,
    MacAddress,
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
struct CharacterStructChrono {
    id: i32,
    uuid: Uuid,
    character: String,
    font_size: i32,
    meta: Json,
    decimal: Decimal,
    big_decimal: BigDecimal,
    created: NaiveDateTime,
    inet: IpNetwork,
    mac_address: MacAddress,
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
struct CharacterStructTime {
    id: i32,
    uuid: Uuid,
    character: String,
    font_size: i32,
    meta: Json,
    decimal: Decimal,
    big_decimal: BigDecimal,
    created: PrimitiveDateTime,
    inet: IpNetwork,
    mac_address: MacAddress,
}
