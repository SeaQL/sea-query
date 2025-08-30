use chrono::NaiveDate;
use sea_query::{
    ColumnDef, Expr, ExprTrait, Func, Iden, MysqlQueryBuilder, OnConflict, Order,
    PostgresQueryBuilder, Query, QueryBuilder, SchemaBuilder, SqliteQueryBuilder, Table,
};
use sqlx::{AnyPool, Row};
use std::env;

use sea_query_sqlx::SqlxBinder;

#[async_std::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || (args[1] != "postgres" && args[1] != "mysql" && args[1] != "sqlite") {
        println!(
            "Expected a single argument out of {{postgres, mysql, sqlite}}, got {}",
            args.split_at(1).1.join(", ")
        );
        return;
    }
    if args[1] == "postgres" {
        exec(
            "postgres://sea:sea@127.0.0.1/query",
            &PostgresQueryBuilder {},
            &PostgresQueryBuilder {},
        )
        .await
    } else if args[1] == "sqlite" {
        exec(
            "sqlite::memory:",
            &SqliteQueryBuilder {},
            &SqliteQueryBuilder {},
        )
        .await
    } else if args[1] == "mysql" {
        exec(
            "mysql://sea:sea@127.0.0.1/query",
            &MysqlQueryBuilder {},
            &MysqlQueryBuilder {},
        )
        .await
    } else {
        panic!()
    };
}

async fn exec(url: &str, query_builder: &impl QueryBuilder, schema_builder: &impl SchemaBuilder) {
    let connection = AnyPool::connect(url).await.unwrap();
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
        .col(ColumnDef::new(Character::FontSize).integer())
        .col(ColumnDef::new(Character::Character).string())
        .col(ColumnDef::new(Character::Created).date_time())
        .build_any(schema_builder);

    let result = sqlx::query(&sql).execute(&mut *pool).await;
    println!("Create table character: {result:?}\n");

    // Create

    let (sql, values) = Query::insert()
        .into_table(Character::Table)
        .columns([
            Character::FontSize,
            Character::Character,
            Character::Created,
        ])
        .values_panic([
            12.into(),
            "A".into(),
            NaiveDate::from_ymd_opt(2020, 8, 20)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .into(),
        ])
        .returning_col(Character::Id)
        .build_any_sqlx(query_builder);

    let row = sqlx::query_with(&sql, values)
        .fetch_one(&mut *pool)
        .await
        .unwrap();
    let id: i32 = row.try_get(0).unwrap();
    println!("Insert into character: last_insert_id = {id}\n");

    // Read

    let (sql, values) = Query::select()
        .columns([
            Character::Id,
            Character::Character,
            Character::FontSize,
            Character::Created,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build_any_sqlx(query_builder);

    let rows = sqlx::query_as_with::<_, CharacterStructChrono, _>(&sql, values)
        .fetch_all(&mut *pool)
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
        .build_any_sqlx(query_builder);

    let result = sqlx::query_with(&sql, values).execute(&mut *pool).await;
    println!("Update character: {result:?}\n");

    // Read

    let (sql, values) = Query::select()
        .columns([
            Character::Id,
            Character::Character,
            Character::FontSize,
            Character::Created,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build_any_sqlx(query_builder);

    let rows = sqlx::query_as_with::<_, CharacterStructChrono, _>(&sql, values)
        .fetch_all(&mut *pool)
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
        .build_any_sqlx(query_builder);

    let row = sqlx::query_with(&sql, values)
        .fetch_one(&mut *pool)
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
        .on_conflict(
            OnConflict::column(Character::Id)
                .update_columns([Character::FontSize, Character::Character])
                .to_owned(),
        )
        .build_any_sqlx(query_builder);

    let result = sqlx::query_with(&sql, values).execute(&mut *pool).await;
    println!("Insert into character (with upsert): {result:?}\n");

    // Read

    let (sql, values) = Query::select()
        .columns([
            Character::Id,
            Character::Character,
            Character::FontSize,
            Character::Created,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .build_any_sqlx(query_builder);

    let rows = sqlx::query_as_with::<_, CharacterStructChrono, _>(&sql, values)
        .fetch_all(&mut *pool)
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
        .build_any_sqlx(query_builder);

    let result = sqlx::query_with(&sql, values).execute(&mut *pool).await;
    println!("Delete character: {result:?}");
}

#[derive(Iden)]
enum Character {
    Table,
    Id,
    Character,
    FontSize,
    Created,
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
struct CharacterStructChrono {
    id: i32,
    character: String,
    font_size: i32,
    created: String,
}
