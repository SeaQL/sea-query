use async_std::task;
use sea_query::{ColumnDef, Expr, Func, Iden, MysqlQueryBuilder, Order, Query, Table, Value, bind_params_sqlx_mysql};
use sqlx::{MySql, MySqlPool, mysql::MySqlArguments, Row};

type SqlxQuery<'a> = sqlx::query::Query<'a, MySql, MySqlArguments>;
type SqlxQueryAs<'a, T> = sqlx::query::QueryAs<'a, MySql, T, MySqlArguments>;

fn main() {

    let connection = task::block_on(async {
        MySqlPool::connect("mysql://query:query@127.0.0.1/query_test").await.unwrap()
    });
    let mut pool = connection.try_acquire().unwrap();

    // Schema

    let sql = Table::create()
        .table(Character::Table)
        .create_if_not_exists()
        .col(ColumnDef::new(Character::Id).integer().not_null().auto_increment().primary_key())
        .col(ColumnDef::new(Character::FontSize).integer())
        .col(ColumnDef::new(Character::Character).string())
        .build(MysqlQueryBuilder);

    let result = task::block_on(async {
        sqlx::query(&sql)
            .execute(&mut pool)
            .await
    });
    println!("Create table character: {:?}\n", result);

    // Create

    let (sql, params) = Query::insert()
        .into_table(Character::Table)
        .columns(vec![
            Character::Character, Character::FontSize
        ])
        .values_panic(vec![
            "A".into(),
            12.into(),
        ])
        .build(MysqlQueryBuilder);

    let result = task::block_on(async {
        bind_query(sqlx::query(&sql), &params)
            .execute(&mut pool)
            .await
    });
    println!("Insert into character: {:?}\n", result);
    let id = result.unwrap().last_insert_id();

    // Read

    let (sql, params) = Query::select()
        .columns(vec![
            Character::Id, Character::Character, Character::FontSize
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build(MysqlQueryBuilder);

    let rows = task::block_on(async {
        bind_query_as(sqlx::query_as::<_, CharacterStruct>(&sql), &params)
            .fetch_all(&mut pool)
            .await
            .unwrap()
    });
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{:?}", row);
    }
    println!();

    // Update

    let (sql, params) = Query::update()
        .table(Character::Table)
        .values(vec![
            (Character::FontSize, 24.into()),
        ])
        .and_where(Expr::col(Character::Id).eq(id))
        .build(MysqlQueryBuilder);

    let result = task::block_on(async {
        bind_query(sqlx::query(&sql), &params)
            .execute(&mut pool)
            .await
    });
    println!("Update character: {:?}\n", result);

    // Read

    let (sql, params) = Query::select()
        .columns(vec![
            Character::Id, Character::Character, Character::FontSize
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build(MysqlQueryBuilder);

    let rows = task::block_on(async {
        bind_query_as(sqlx::query_as::<_, CharacterStruct>(&sql), &params)
            .fetch_all(&mut pool)
            .await
            .unwrap()
    });
    println!("Select one from character:");
    for row in rows.iter() {
        println!("{:?}", row);
    }
    println!();

    // Delete

    let (sql, params) = Query::delete()
        .from_table(Character::Table)
        .and_where(Expr::col(Character::Id).eq(id))
        .build(MysqlQueryBuilder);

    let result = task::block_on(async {
        bind_query(sqlx::query(&sql), &params)
            .execute(&mut pool)
            .await
    });
    println!("Delete character: {:?}\n", result);

    // Count

    let (sql, params) = Query::select()
        .from(Character::Table)
        .expr(Func::count(Expr::col(Character::Id)))
        .build(MysqlQueryBuilder);

    let row = task::block_on(async {
        bind_query(sqlx::query(&sql), &params)
            .fetch_one(&mut pool)
            .await
            .unwrap()
    });
    print!("Count character: ");
    let count: i64 = row.try_get(0).unwrap();
    println!("{}", count);
}

pub fn bind_query<'a>(query: SqlxQuery<'a>, params: &'a [Value]) -> SqlxQuery<'a> {
    bind_params_sqlx_mysql!(query, params)
}

pub fn bind_query_as<'a, T>(query: SqlxQueryAs<'a, T>, params: &'a [Value]) -> SqlxQueryAs<'a, T> {
    bind_params_sqlx_mysql!(query, params)
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
