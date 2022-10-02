use chrono::{NaiveDate, NaiveDateTime};
use sea_query::{
    ColumnDef, Expr, Func, Iden, MysqlQueryBuilder, OnConflict, Order, PostgresQueryBuilder, Query,
    QueryBuilder, SchemaBuilder, SqliteQueryBuilder, Table,
};
use sqlx::{AnyPool, Row};
use std::env;

use sea_query_binder::SqlxBinder;

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
    let (url, box_query_builder, box_schema_builder): (
        &str,
        Box<dyn QueryBuilder>,
        Box<dyn SchemaBuilder>,
    ) = if args[1] == "postgres" {
        (
            "postgres://sea:sea@127.0.0.1/query",
            Box::new(PostgresQueryBuilder {}),
            Box::new(PostgresQueryBuilder {}),
        )
    } else if args[1] == "sqlite" {
        (
            "sqlite::memory:",
            Box::new(SqliteQueryBuilder {}),
            Box::new(SqliteQueryBuilder {}),
        )
    } else if args[1] == "mysql" {
        (
            "mysql://sea:sea@127.0.0.1/query",
            Box::new(MysqlQueryBuilder {}),
            Box::new(MysqlQueryBuilder {}),
        )
    } else {
        panic!()
    };
    let query_builder = &*box_query_builder;
    let schema_builder = &*box_schema_builder;
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

    let result = sqlx::query(&sql).execute(&mut pool).await;
    println!("Create table character: {:?}\n", result);

    // Create

    let (sql, values) = Query::insert()
        .into_table(Character::Table)
        .columns([
            Character::FontSize,
            Character::Character,
            Character::Created,
        ])
        .exprs_panic([
            12.into(),
            "A".into(),
            NaiveDate::from_ymd(2020, 8, 20).and_hms(0, 0, 0).into(),
        ])
        .returning_col(Character::Id)
        .build_any_sqlx(query_builder);

    let row = sqlx::query_with(&sql, values)
        .fetch_one(&mut pool)
        .await
        .unwrap();
    let id: i32 = row.try_get(0).unwrap();
    println!("Insert into character: last_insert_id = {}\n", id);

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
        .exprs([(Character::FontSize, 24.into())])
        .and_where(Expr::col(Character::Id).eq(id))
        .build_any_sqlx(query_builder);

    let result = sqlx::query_with(&sql, values).execute(&mut pool).await;
    println!("Update character: {:?}\n", result);

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
        .build_any_sqlx(query_builder);

    let row = sqlx::query_with(&sql, values)
        .fetch_one(&mut pool)
        .await
        .unwrap();
    print!("Count character: ");
    let count: i64 = row.try_get(0).unwrap();
    println!("{}", count);
    println!();

    // Upsert

    let (sql, values) = Query::insert()
        .into_table(Character::Table)
        .columns([Character::Id, Character::FontSize, Character::Character])
        .exprs_panic([1.into(), 16.into(), "B".into()])
        .on_conflict(
            OnConflict::column(Character::Id)
                .update_columns([Character::FontSize, Character::Character])
                .to_owned(),
        )
        .build_any_sqlx(query_builder);

    let result = sqlx::query_with(&sql, values).execute(&mut pool).await;
    println!("Insert into character (with upsert): {:?}\n", result);

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
        .fetch_all(&mut pool)
        .await
        .unwrap();
    println!("Select all characters:");
    for row in rows.iter() {
        println!("{:?}", row);
    }
    println!();

    // Delete

    let (sql, values) = Query::delete()
        .from_table(Character::Table)
        .and_where(Expr::col(Character::Id).eq(id))
        .build_any_sqlx(query_builder);

    let result = sqlx::query_with(&sql, values).execute(&mut pool).await;
    println!("Delete character: {:?}", result);
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
    created: NaiveDateTime,
}
