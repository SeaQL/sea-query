use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use sea_query::{ColumnDef, Expr, Func, Iden, MysqlQueryBuilder, Order, Query, Table};
use sqlx::{MySqlPool, Row, types::chrono::NaiveDateTime};

sea_query::sea_query_driver_mysql!();
use sea_query_driver_mysql::{bind_query, bind_query_as};
use uuid::Uuid;
use serde_json::{Value as Json, json};

#[async_std::main]
async fn main() {
    let connection = MySqlPool::connect("mysql://sea:sea@127.0.0.1/query")
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
        .build(MysqlQueryBuilder);

    let result = sqlx::query(&sql).execute(&mut pool).await;
    println!("Create table character: {:?}\n", result);

    // Create

    let (sql, values) = Query::insert()
        .into_table(Character::Table)
        .columns(vec![
            Character::Uuid,
            Character::FontSize,
            Character::Character,
            Character::Meta,
            Character::Decimal,
            Character::BigDecimal,
            Character::Created,
        ])
        .values_panic(vec![
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            }).into(),
            Decimal::from_i128_with_scale(3141i128, 3).into(),
            BigDecimal::from_i128(3141i128).unwrap().with_scale(3).into(),
            NaiveDate::from_ymd(2020, 8, 20).and_hms(0, 0, 0).into(),
        ])
        .build(MysqlQueryBuilder);

    let result = bind_query(sqlx::query(&sql), &values)
        .execute(&mut pool)
        .await;
    println!("Insert into character: {:?}\n", result);
    let id = result.unwrap().last_insert_id();

    // Read

    let (sql, values) = Query::select()
        .columns(vec![
            Character::Id,
            Character::Uuid,
            Character::Character,
            Character::FontSize,
            Character::Meta,
            Character::Decimal,
            Character::BigDecimal,
            Character::Created,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build(MysqlQueryBuilder);

    let rows = bind_query_as(sqlx::query_as::<_, CharacterStruct>(&sql), &values)
        .fetch_all(&mut pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{:?}", row);
    }
    println!();

    // Update

    let (sql, values) = Query::update()
        .table(Character::Table)
        .values(vec![(Character::FontSize, 24.into())])
        .and_where(Expr::col(Character::Id).eq(id))
        .build(MysqlQueryBuilder);

    let result = bind_query(sqlx::query(&sql), &values)
        .execute(&mut pool)
        .await;
    println!("Update character: {:?}\n", result);

    // Read

    let (sql, values) = Query::select()
        .columns(vec![
            Character::Id,
            Character::Uuid,
            Character::Character,
            Character::FontSize,
            Character::Meta,
            Character::Decimal,
            Character::BigDecimal,
            Character::Created,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build(MysqlQueryBuilder);

    let rows = bind_query_as(sqlx::query_as::<_, CharacterStruct>(&sql), &values)
        .fetch_all(&mut pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{:?}", row);
    }
    println!();

    // Count

    let (sql, values) = Query::select()
        .from(Character::Table)
        .expr(Func::count(Expr::col(Character::Id)))
        .build(MysqlQueryBuilder);

    let row = bind_query(sqlx::query(&sql), &values)
        .fetch_one(&mut pool)
        .await
        .unwrap();
    print!("Count character: ");
    let count: i64 = row.try_get(0).unwrap();
    println!("{}", count);
    println!();

    // Delete

    let (sql, values) = Query::delete()
        .from_table(Character::Table)
        .and_where(Expr::col(Character::Id).eq(id))
        .build(MysqlQueryBuilder);

    let result = bind_query(sqlx::query(&sql), &values)
        .execute(&mut pool)
        .await;
    println!("Delete character: {:?}", result);
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
}

#[derive(sqlx::FromRow, Debug)]
struct CharacterStruct {
    id: i32,
    uuid: Uuid,
    character: String,
    font_size: i32,
    meta: Json,
    decimal: Decimal,
    big_decimal: BigDecimal,
    created: NaiveDateTime,
}
