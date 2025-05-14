use chrono::{NaiveDate, NaiveDateTime};
use rusqlite::{Connection, Result, Row};
use sea_query::{ColumnDef, Expr, Func, Iden, Order, Query, SqliteQueryBuilder, Table};

use sea_query_rusqlite::RusqliteBinder;
use serde_json::{Value as Json, json};
use time::{
    PrimitiveDateTime, format_description,
    macros::{date, time},
};
use uuid::Uuid;

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    // Schema

    let sql = [
        Table::drop()
            .table(Character::Table)
            .if_exists()
            .build(SqliteQueryBuilder),
        Table::create()
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
            .build(SqliteQueryBuilder),
    ]
    .join("; ");

    conn.execute_batch(&sql)?;
    println!("Create table character: Ok()");
    println!();

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
            None::<NaiveDate>.into(),
        ])
        .values_panic([
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            Some(
                NaiveDate::from_ymd_opt(2020, 1, 1)
                    .unwrap()
                    .and_hms_opt(2, 2, 2)
                    .unwrap(),
            )
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
            None::<PrimitiveDateTime>.into(),
        ])
        .values_panic([
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            Some(date!(2020 - 1 - 1).with_time(time!(2:2:2))).into(),
        ])
        .build_rusqlite(SqliteQueryBuilder);

    let result = conn.execute(sql.as_str(), &*values.as_params());
    println!("Insert into character: {result:?}\n");
    let id = conn.last_insert_rowid();

    // Read

    let (sql, values) = Query::select()
        .columns([
            Character::Id,
            Character::Uuid,
            Character::FontSize,
            Character::Character,
            Character::Meta,
            Character::Created,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build_rusqlite(SqliteQueryBuilder);

    println!("Select one from character:");
    let mut stmt = conn.prepare(sql.as_str())?;
    let mut rows = stmt.query(&*values.as_params())?;
    while let Some(row) = rows.next()? {
        let item = CharacterStructChrono::from(row);
        println!("{item:?}");

        let item = CharacterStructTime::from(row);
        println!("{item:?}");
    }
    println!();

    // Update

    let (sql, values) = Query::update()
        .table(Character::Table)
        .values([(Character::FontSize, 24.into())])
        .and_where(Expr::col(Character::Id).eq(id))
        .build_rusqlite(SqliteQueryBuilder);

    let result = conn.execute(sql.as_str(), &*values.as_params());
    println!("Update character: {result:?}\n");

    // Read

    let (sql, values) = Query::select()
        .columns([
            Character::Id,
            Character::Uuid,
            Character::FontSize,
            Character::Character,
            Character::Meta,
            Character::Created,
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build_rusqlite(SqliteQueryBuilder);

    println!("Select one from character:");
    let mut stmt = conn.prepare(sql.as_str())?;
    let mut rows = stmt.query(&*values.as_params())?;
    while let Some(row) = rows.next()? {
        let item = CharacterStructChrono::from(row);
        println!("{item:?}");

        let item = CharacterStructTime::from(row);
        println!("{item:?}");
    }
    println!();

    // Count

    let (sql, values) = Query::select()
        .from(Character::Table)
        .expr(Func::count(Expr::col(Character::Id)))
        .build_rusqlite(SqliteQueryBuilder);

    print!("Count character: ");
    let mut stmt = conn.prepare(sql.as_str())?;
    let mut rows = stmt.query(&*values.as_params())?;
    let count: i64 = if let Some(row) = rows.next()? {
        row.get_unwrap(0)
    } else {
        0
    };
    println!("{count}");
    println!();

    // Delete

    let (sql, values) = Query::delete()
        .from_table(Character::Table)
        .and_where(Expr::col(Character::Id).eq(id))
        .build_rusqlite(SqliteQueryBuilder);

    let result = conn.execute(sql.as_str(), &*values.as_params());
    println!("Delete character: {result:?}");

    Ok(())
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

#[derive(Debug)]
#[allow(dead_code)]
struct CharacterStructChrono {
    id: i32,
    uuid: Uuid,
    character: String,
    font_size: i32,
    meta: Json,
    created: Option<NaiveDateTime>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct CharacterStructTime {
    id: i32,
    uuid: Uuid,
    character: String,
    font_size: i32,
    meta: Json,
    created: Option<PrimitiveDateTime>,
}

impl From<&Row<'_>> for CharacterStructChrono {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get_unwrap("id"),
            uuid: row.get_unwrap("uuid"),
            character: row.get_unwrap("character"),
            font_size: row.get_unwrap("font_size"),
            meta: row.get_unwrap("meta"),
            created: row.get_unwrap("created"),
        }
    }
}

impl From<&Row<'_>> for CharacterStructTime {
    fn from(row: &Row) -> Self {
        let created: String = row.get_unwrap("created");
        let format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
        let created = PrimitiveDateTime::parse(&created, &format).ok();
        Self {
            id: row.get_unwrap("id"),
            uuid: row.get_unwrap("uuid"),
            character: row.get_unwrap("character"),
            font_size: row.get_unwrap("font_size"),
            meta: row.get_unwrap("meta"),
            created,
        }
    }
}
