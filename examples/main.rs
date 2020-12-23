use sea_query::*;
use sqlx::{Any, AnyPool, any::AnyArguments};
use serde_json::json;
use async_std::task;
use std::fmt;

type SqlxQuery<'a> = sqlx::query::Query<'a, Any, AnyArguments<'a>>;
type SqlxQueryAs<'a, T> = sqlx::query::QueryAs<'a, Any, T, AnyArguments<'a>>;

fn main() {
    let connection = task::block_on(async {
        AnyPool::connect("mysql://query:query@127.0.0.1/query_test").await.unwrap()
    });
    let mut pool = connection.try_acquire().unwrap();

    let database = "mysql";

    let table_builder: Box<dyn GenericBuilder> = match database {
        "mysql" => Box::new(MysqlQueryBuilder),
        "postgres" => Box::new(PostgresQueryBuilder),
        _ => panic!("unsupported database connection string"),
    };

    let sql = Table::create()
        .table(Char::Table)
        .create_if_not_exists()
        .col(ColumnDef::new(Char::Id).integer().not_null().auto_increment().primary_key())
        .col(ColumnDef::new(Char::FontSize).integer())
        .col(ColumnDef::new(Char::Character).string())
        .col(ColumnDef::new(Char::SizeW).integer())
        .col(ColumnDef::new(Char::SizeH).integer())
        .build_any(table_builder.table_builder());

    let result = task::block_on(async {
        sqlx::query(&sql)
            .execute(&mut pool)
            .await
    });
    println!("Create table character: {:?}\n", result);


    let (sql, params) = Query::insert()
        .into_table(Char::Table)
        .columns(vec![
            Char::Character, Char::SizeW, Char::SizeH, Char::FontSize
        ])
        .values_panic(vec![
            "Character".into(),
            123.into(),
            456.into(),
            3.into(),
        ])
        .json(json!({
            "character": "S",
            "size_w": 12,
            "size_h": 34,
            "font_size": 2,
        }))
        .build_any(table_builder.query_builder());

    let result = task::block_on(async {
        bind(sqlx::query(&sql), &params)
            .execute(&mut pool)
            .await
    });
    println!("Insert into character: {:?}\n", result);


    let (sql, params) = Query::select()
        .columns(vec![
            Char::Id, Char::Character, Char::SizeW, Char::SizeH, Char::FontSize
        ])
        .from(Char::Table)
        .build_any(table_builder.query_builder());

    let rows = task::block_on(async {
        bind_query(sqlx::query_as::<_, CharacterStruct>(&sql), &params)
            .fetch_all(&mut pool)
            .await
            .unwrap()
    });
    println!("Select all from character:");
    for row in rows.iter() {
        println!("{:?}", row);
    }
}

enum Character {
    Table,
    Id,
    Character,
    FontSize,
    SizeW,
    SizeH,
}

type Char = Character;

impl Iden for Character {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
        write!(s, "{}", match self {
            Self::Table => "character",
            Self::Id => "id",
            Self::Character => "character",
            Self::FontSize => "font_size",
            Self::SizeW => "size_w",
            Self::SizeH => "size_h",
        }).unwrap();
    }
}

#[derive(sqlx::FromRow, Debug)]
struct CharacterStruct {
    id: i32,
    character: String,
    font_size: i32,
    size_w: i32,
    size_h: i32,
}

fn bind<'a>(query: SqlxQuery<'a>, params: &'a [Value]) -> SqlxQuery<'a> {
    bind_params!(query, params)
}

fn bind_query<'a, T>(query: SqlxQueryAs<'a, T>, params: &'a [Value]) -> SqlxQueryAs<'a, T> {
    bind_params!(query, params)
}

#[macro_export]
macro_rules! bind_params {
    ( $query:expr, $params:expr ) => {
        {
            let mut query = $query;
            for value in $params.iter() {
                query = match value {
                    Value::NULL => query.bind(None::<i32>),
                    Value::Bytes(v) => query.bind(std::str::from_utf8(v).unwrap()),
                    Value::Int(v) => query.bind(v),
                    Value::UInt(v) => query.bind(format!("{}", v)),
                    Value::Float(v) => query.bind(v),
                    Value::Double(v) => query.bind(format!("{}", v)),
                    Value::Date(year, month, day, hour, minutes, seconds, _micro_seconds) => 
                        query.bind(format!(
                            "{:04}{:02}{:02} {:02}{:02}{:02}",
                            year, month, day, hour, minutes, seconds
                        )),
                    Value::Time(negative, days, hours, minutes, seconds, _micro_seconds) => 
                        query.bind(format!(
                            "{}{:02}{:02} {:02}{:02}.{:03}",
                            if *negative { "-" } else { "" }, days, hours, minutes, seconds, _micro_seconds / 1000
                        )),
                };
            }
            query
        }
    };
}
