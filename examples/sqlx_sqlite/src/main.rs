use chrono::{NaiveDate, NaiveDateTime};
use sea_query::{
    ColumnDef, Expr, ExprTrait, Func, Iden, OnConflict, Order, Query, SqliteQueryBuilder, Table,
};
use sea_query_sqlx::SqlxBinder;
use serde_json::{Value as Json, json};
use sqlx::{Row, SqlitePool};
use time::{
    PrimitiveDateTime,
    macros::{date, time},
};
use uuid::Uuid;

async fn create_table(pool: &SqlitePool) {
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

    let res = sqlx::query(&sql).execute(pool).await;
    println!("Create table character: {res:?}");
}

async fn query_builder_crud(pool: &SqlitePool) {
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
            18.into(),
            "B".into(),
            json!({
                "notes": "more notes here",
            })
            .into(),
            date!(2020 - 8 - 20).with_time(time!(0:0:0)).into(),
        ])
        .build_sqlx(SqliteQueryBuilder);
    println!("{sql}");

    let res = sqlx::query_with(&sql, values).execute(pool).await.unwrap();

    let id: i64 = res.last_insert_rowid();
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
    println!("{sql}");

    let rows = sqlx::query_as_with::<_, CharacterStructChrono, _>(&sql, values.clone())
        .fetch_all(pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{row:?}");
    }
    println!();

    let rows = sqlx::query_as_with::<_, CharacterStructTime, _>(&sql, values.clone())
        .fetch_all(pool)
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
    println!("{sql}");

    let res = sqlx::query_with(&sql, values).execute(pool).await;
    println!("Update character: {res:?}\n");

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
    println!("{sql}");

    let rows = sqlx::query_as_with::<_, CharacterStructChrono, _>(&sql, values.clone())
        .fetch_all(pool)
        .await
        .unwrap();
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{row:?}\n");
    }
    let rows = sqlx::query_as_with::<_, CharacterStructTime, _>(&sql, values.clone())
        .fetch_all(pool)
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
        .expr_as(Func::count(Expr::col(Character::Id)), "count")
        .build_sqlx(SqliteQueryBuilder);
    println!("{sql}");

    let row = sqlx::query_with(&sql, values)
        .fetch_one(pool)
        .await
        .unwrap();

    let count: i64 = row.get("count"); // or row.try_get(0).unwrap()
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
    println!("{sql}");

    let res = sqlx::query_with(&sql, values).execute(pool).await;
    println!("Insert into character (with upsert): {res:?}\n");

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
        .and_where(Expr::col(Character::Id).is_in([1, 2]))
        .order_by(Character::Id, Order::Desc)
        .build_sqlx(SqliteQueryBuilder);
    println!("{sql}");

    let rows = sqlx::query_as_with::<_, CharacterStructChrono, _>(&sql, values.clone())
        .fetch_all(pool)
        .await
        .unwrap();
    println!("Select two characters:");
    for row in rows.iter() {
        println!("{row:?}");
    }

    let rows = sqlx::query_as_with::<_, CharacterStructTime, _>(&sql, values.clone())
        .fetch_all(pool)
        .await
        .unwrap();
    println!("Select two characters:");
    for row in rows.iter() {
        println!("{row:?}");
    }
    println!();

    let ids = [1, 2];

    // Delete
    let (sql, values) = Query::delete()
        .from_table(Character::Table)
        .and_where(Expr::col(Character::Id).is_in(ids))
        .build_sqlx(SqliteQueryBuilder);
    println!("{sql}");

    let res = sqlx::query_with(&sql, values).execute(pool).await;

    println!("Delete character: {res:?}");
}

async fn raw_sql_test(pool: &SqlitePool) {
    let mut sql;

    let rows: Vec<CharacterStructTime> = sea_query::sqlx::sqlite::query_as!(
        sql = r#"SELECT "id", "uuid", "character", "font_size", "meta", "created" FROM "character" ORDER BY "id" ASC"#
    ).fetch_all(pool).await.unwrap();
    println!("{sql}");
    assert!(rows.is_empty());
    println!("Got {} rows", rows.len());
    println!();

    let values = vec![
        (
            Uuid::new_v4(),
            12,
            "A",
            json!({
                "notes": "some notes here",
            }),
            NaiveDate::from_ymd_opt(2020, 8, 20)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        ),
        (
            Uuid::new_v4(),
            18,
            "B",
            json!({
                "notes": "more notes here",
            }),
            NaiveDate::from_ymd_opt(2020, 8, 22)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        ),
    ];

    let res = sea_query::sqlx::sqlite::query!(
        sql = r#"INSERT INTO "character" ("uuid", "font_size", "character", "meta", "created") VALUES {..(values.0:4),}"#
    ).execute(pool).await.unwrap();
    println!("{sql}");

    let id: i64 = res.last_insert_rowid();
    println!("Insert into character: last_insert_id = {id}\n");

    let rows: Vec<CharacterStructTime> = sea_query::sqlx::sqlite::query_as!(
        sql = r#"SELECT "id", "uuid", "character", "font_size", "meta", "created" FROM "character" ORDER BY "id" ASC"#
    ).fetch_all(pool).await.unwrap();
    println!("{sql}");
    println!("Got {} rows", rows.len());
    for row in rows.iter() {
        println!("{row:?}");
    }
    println!();

    let mut character = rows[0].clone();
    character.font_size = 18;

    let res = sea_query::sqlx::sqlite::query!(
        sql = r#"UPDATE "character" SET "font_size" = {character.font_size} WHERE "id" = {character.id}"#
    ).execute(pool).await.unwrap();
    println!("{sql}");
    println!("Update character: {res:?}");
    println!();

    let ids = vec![rows[0].id, rows[1].id];

    let rows: Vec<CharacterStructTime> = sea_query::sqlx::sqlite::query_as!(
        sql = r#"SELECT "id", "uuid", "character", "font_size", "meta", "created" FROM "character" WHERE "id" IN ({..ids})"#
    ).fetch_all(pool).await.unwrap();
    println!("{sql}");
    println!("Got {} rows", rows.len());
    for row in rows.iter() {
        println!("{row:?}");
    }
    println!();

    let res =
        sea_query::sqlx::sqlite::query!(sql = r#"DELETE FROM "character" WHERE "id" IN ({..ids})"#)
            .execute(pool)
            .await
            .unwrap();
    println!("{sql}");
    println!("Delete character: {res:?}");
    println!();

    let rows: Vec<CharacterStructTime> = sea_query::sqlx::sqlite::query_as!(
        sql = r#"SELECT "id", "uuid", "character", "font_size", "meta", "created" FROM "character" ORDER BY "id" ASC"#
    ).fetch_all(pool).await.unwrap();
    println!("{sql}");
    assert!(rows.is_empty());
    println!("Got {} rows", rows.len());
}

#[async_std::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    println!(" ===== CREATE TALBE ===== ");
    create_table(&pool).await;
    println!();

    println!(" ===== QUERY BUILDER ===== ");
    query_builder_crud(&pool).await;
    println!();

    println!(" ===== RAW SQL ===== ");
    raw_sql_test(&pool).await;
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

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
struct CharacterStructTime {
    id: i32,
    uuid: Uuid,
    character: String,
    font_size: i32,
    meta: Json,
    created: PrimitiveDateTime,
}
