use chrono::{NaiveDate, NaiveDateTime};
use sea_query::{ColumnDef, Expr, Func, Iden, OnConflict, Order, Query, SqliteQueryBuilder, Table};
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};
use time::{date, time, PrimitiveDateTime};

sea_query::sea_query_driver_sqlite!();
use sea_query_driver_sqlite::{bind_query, bind_query_as};
use serde_json::{json, Value as Json};
use uuid::Uuid;

#[async_std::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

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
        .col(ColumnDef::new(Character::Created).date_time())
        .build(SqliteQueryBuilder);

    let result = sqlx::query(&sql).execute(&pool).await;
    println!("Create table character: {:?}\n", result);

    // Create
    let (sql, values) = Query::insert()
        .into_table(Character::Table)
        .columns([
            Character::Uuid,
            Character::FontSize,
            Character::Character,
            Character::Meta,
            Character::Created,
        ])
        .values_panic(vec![
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            NaiveDate::from_ymd(2020, 8, 20).and_hms(0, 0, 0).into(),
        ])
        .values_panic(vec![
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            date!(2020 - 8 - 20).with_time(time!(0:0:0)).into(),
        ])
        .build(SqliteQueryBuilder);

    //TODO: Implement RETURNING (returning_col) for the Sqlite driver.
    let row = bind_query(sqlx::query(&sql), &values)
        .execute(&pool)
        .await
        .unwrap();

    let id: i64 = row.last_insert_rowid();
    println!("Insert into character: last_insert_id = {}\n", id);

    // Read
    let (sql, values) = Query::select()
        .columns([
            Character::Id,
            Character::Uuid,
            Character::Character,
            Character::FontSize,
            Character::Meta,
            Character::Created,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build(SqliteQueryBuilder);

    let rows = bind_query_as(sqlx::query_as::<_, CharacterStructChrono>(&sql), &values)
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{:?}", row);
    }
    println!();

    let rows = bind_query(sqlx::query(&sql), &values)
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        let item = CharacterStructTime::try_from(row).unwrap();
        println!("{:?}", item);
    }
    println!();

    // Update
    let (sql, values) = Query::update()
        .table(Character::Table)
        .values(vec![(Character::FontSize, 24.into())])
        .and_where(Expr::col(Character::Id).eq(id))
        .build(SqliteQueryBuilder);

    let result = bind_query(sqlx::query(&sql), &values).execute(&pool).await;
    println!("Update character: {:?}\n", result);

    // Read
    let (sql, values) = Query::select()
        .columns([
            Character::Id,
            Character::Uuid,
            Character::Character,
            Character::FontSize,
            Character::Meta,
            Character::Created,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build(SqliteQueryBuilder);

    let rows = bind_query_as(sqlx::query_as::<_, CharacterStructChrono>(&sql), &values)
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{:?}\n", row);
    }

    let rows = bind_query(sqlx::query(&sql), &values)
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        let item = CharacterStructTime::try_from(row).unwrap();
        println!("{:?}", item);
    }
    println!();

    // Count
    let (sql, values) = Query::select()
        .from(Character::Table)
        .expr(Func::count(Expr::col(Character::Id)))
        .build(SqliteQueryBuilder);

    let row = bind_query(sqlx::query(&sql), &values)
        .fetch_one(&pool)
        .await
        .unwrap();

    let count: i64 = row.try_get(0).unwrap();
    println!("Count character: {}\n", count);

    // Upsert
    let (sql, values) = Query::insert()
        .into_table(Character::Table)
        .columns([Character::Id, Character::FontSize, Character::Character])
        .values_panic(vec![1.into(), 16.into(), "B".into()])
        .values_panic(vec![2.into(), 24.into(), "C".into()])
        .on_conflict(
            OnConflict::column(Character::Id)
                .update_columns([Character::FontSize, Character::Character])
                .to_owned(),
        )
        .build(SqliteQueryBuilder);

    let result = bind_query(sqlx::query(&sql), &values).execute(&pool).await;
    println!("Insert into character (with upsert): {:?}\n", result);

    // Read
    let (sql, values) = Query::select()
        .columns([
            Character::Id,
            Character::Uuid,
            Character::Character,
            Character::FontSize,
            Character::Meta,
            Character::Created,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .build(SqliteQueryBuilder);

    let rows = bind_query_as(sqlx::query_as::<_, CharacterStructChrono>(&sql), &values)
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select all characters:");
    for row in rows.iter() {
        println!("{:?}\n", row);
    }

    let rows = bind_query(sqlx::query(&sql), &values)
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select all characters:");
    for row in rows.iter() {
        let item = CharacterStructTime::try_from(row).unwrap();
        println!("{:?}", item);
    }
    println!();

    // Delete
    let (sql, values) = Query::delete()
        .from_table(Character::Table)
        .and_where(Expr::col(Character::Id).eq(id))
        .build(SqliteQueryBuilder);

    let result = bind_query(sqlx::query(&sql), &values).execute(&pool).await;

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
    Created,
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
struct CharacterStructChrono {
    id: i32,
    uuid: Uuid,
    character: String,
    font_size: i32,
    meta: Json,
    created: NaiveDateTime,
}

#[derive(Debug)]
#[allow(dead_code)]
struct CharacterStructTime {
    id: i32,
    uuid: Uuid,
    character: String,
    font_size: i32,
    meta: Json,
    created: PrimitiveDateTime,
}

impl TryFrom<&SqliteRow> for CharacterStructTime {
    type Error = sqlx::Error;

    fn try_from(row: &SqliteRow) -> Result<Self, Self::Error> {
        let created: String = dbg!(row.try_get("created")?);
        let created = PrimitiveDateTime::parse(&created, "%Y-%m-%d %H:%M:%S").unwrap();
        Ok(Self {
            id: row.try_get("id")?,
            uuid: row.try_get("uuid")?,
            character: row.try_get("character")?,
            font_size: row.try_get("font_size")?,
            meta: row.try_get("meta")?,
            created,
        })
    }
}
