use chrono::{NaiveDate, NaiveDateTime};
use sea_query::{
    ColumnDef, ConditionalStatement, Expr, Func, Iden, Index, OnConflict, Order, Query,
    SqliteQueryBuilder, Table,
};
use sea_query_binder::SqlxBinder;
use serde_json::{json, Value as Json};
use sqlx::{Row, SqlitePool};
use time::{
    macros::{date, time},
    PrimitiveDateTime,
};
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
    println!("Create table character: {result:?}\n");

    // Partial Index
    let partial_index = Index::create()
        .name("partial_index_small_font")
        .table(Character::Table)
        .col(Character::FontSize)
        .and_where(Expr::col(Character::FontSize).lt(11).not())
        .build(SqliteQueryBuilder);

    let index = sqlx::query(&partial_index).execute(&pool).await;
    println!("Create partial index: {index:?}\n");

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
        .values_panic([
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            NaiveDate::from_ymd_opt(2020, 8, 20)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
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
            date!(2020 - 8 - 20).with_time(time!(0:0:0)).into(),
        ])
        .build_sqlx(SqliteQueryBuilder);

    //TODO: Implement RETURNING (returning_col) for the Sqlite driver.
    let row = sqlx::query_with(&sql, values).execute(&pool).await.unwrap();

    let id: i64 = row.last_insert_rowid();
    println!("Insert into character: last_insert_id = {id}\n");

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
        .build_sqlx(SqliteQueryBuilder);

    let rows = sqlx::query_as_with::<_, CharacterStructChrono, _>(&sql, values.clone())
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{row:?}");
    }
    println!();

    let rows = sqlx::query_as_with::<_, CharacterStructTime, _>(&sql, values.clone())
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{row:?}\n");
    }
    println!();

    // Update
    let (sql, values) = Query::update()
        .table(Character::Table)
        .values([(Character::FontSize, 24.into())])
        .and_where(Expr::col(Character::Id).eq(id))
        .build_sqlx(SqliteQueryBuilder);

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
            Character::Created,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build_sqlx(SqliteQueryBuilder);

    let rows = sqlx::query_as_with::<_, CharacterStructChrono, _>(&sql, values.clone())
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{row:?}\n");
    }
    let rows = sqlx::query_as_with::<_, CharacterStructTime, _>(&sql, values.clone())
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{row:?}\n");
    }
    println!();

    // Count
    let (sql, values) = Query::select()
        .from(Character::Table)
        .expr(Func::count(Expr::col(Character::Id)))
        .build_sqlx(SqliteQueryBuilder);

    let row = sqlx::query_with(&sql, values)
        .fetch_one(&pool)
        .await
        .unwrap();

    let count: i64 = row.try_get(0).unwrap();
    println!("Count character: {count}\n");

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
        .build_sqlx(SqliteQueryBuilder);

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
            Character::Created,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .build_sqlx(SqliteQueryBuilder);

    let rows = sqlx::query_as_with::<_, CharacterStructChrono, _>(&sql, values.clone())
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("Select all characters:");
    for row in rows.iter() {
        println!("{row:?}");
    }

    let rows = sqlx::query_as_with::<_, CharacterStructTime, _>(&sql, values.clone())
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
        .build_sqlx(SqliteQueryBuilder);

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

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
struct CharacterStructTime {
    id: i32,
    uuid: Uuid,
    character: String,
    font_size: i32,
    meta: Json,
    created: PrimitiveDateTime,
}
