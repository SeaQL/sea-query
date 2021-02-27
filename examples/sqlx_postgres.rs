use async_std::task;
use serde_json::json;
use sqlx::{Postgres, PgPool, postgres::PgArguments};
use sea_query::*;

type SqlxQuery<'a> = sqlx::query::Query<'a, Postgres, PgArguments>;
type SqlxQueryAs<'a, T> = sqlx::query::QueryAs<'a, Postgres, T, PgArguments>;

fn main() {

    let connection = task::block_on(async {
        PgPool::connect("postgres://query:query@127.0.0.1/query_test").await.unwrap()
    });
    let mut pool = connection.try_acquire().unwrap();

    let sql = Table::create()
        .table(Character::Table)
        .create_if_not_exists()
        .col(ColumnDef::new(Character::Id).integer().not_null().auto_increment().primary_key())
        .col(ColumnDef::new(Character::FontSize).integer())
        .col(ColumnDef::new(Character::Character).string())
        .build(PostgresQueryBuilder);

    let result = task::block_on(async {
        sqlx::query(&sql)
            .execute(&mut pool)
            .await
    });
    println!("Create table character: {:?}\n", result);


    let (sql, params) = Query::insert()
        .into_table(Character::Table)
        .columns(vec![
            Character::Character, Character::FontSize
        ])
        .values_panic(vec![
            "A".into(),
            12.into(),
        ])
        .json(json!({
            "character": "B",
            "font_size": 24,
        }))
        .build(PostgresQueryBuilder);

    let result = task::block_on(async {
        bind_query(sqlx::query(&sql), &params)
            .execute(&mut pool)
            .await
    });
    println!("Insert into character: {:?}\n", result);


    let (sql, params) = Query::select()
        .columns(vec![
            Character::Id, Character::Character, Character::FontSize
        ])
        .from(Character::Table)
        .build(PostgresQueryBuilder);

    let rows = task::block_on(async {
        bind_query_as(sqlx::query_as::<_, CharacterStruct>(&sql), &params)
            .fetch_all(&mut pool)
            .await
            .unwrap()
    });
    println!("Select all from character:");
    for row in rows.iter() {
        println!("{:?}", row);
    }
}

pub fn bind_query<'a>(query: SqlxQuery<'a>, params: &'a [Value]) -> SqlxQuery<'a> {
    bind_params_sqlx_postgres!(query, params)
}

pub fn bind_query_as<'a, T>(query: SqlxQueryAs<'a, T>, params: &'a [Value]) -> SqlxQueryAs<'a, T> {
    bind_params_sqlx_postgres!(query, params)
}

#[derive(Iden)]
enum Character {
    Table,
    Id,
    Character,
    FontSize,
}

#[derive(sqlx::FromRow, Debug)]
struct CharacterStruct {
    id: i32,
    character: String,
    font_size: i32,
}
