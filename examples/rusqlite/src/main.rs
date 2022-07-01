use chrono::{NaiveDate, NaiveDateTime};
use rusqlite::{Connection, Result, Row};
use sea_query::{ColumnDef, Expr, Func, Iden, Order, Query, SqliteQueryBuilder, Table};

sea_query::sea_query_driver_rusqlite!();
use sea_query_driver_rusqlite::RusqliteValues;
use serde_json::{json, Value as Json};
use time::{date, time, PrimitiveDateTime};
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
        .values_panic(vec![
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            None::<NaiveDate>.into(),
        ])
        .values_panic(vec![
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            Some(NaiveDate::from_ymd(2020, 1, 1).and_hms(2, 2, 2)).into(),
        ])
        .values_panic(vec![
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            None::<PrimitiveDateTime>.into(),
        ])
        .values_panic(vec![
            Uuid::new_v4().into(),
            12.into(),
            "A".into(),
            json!({
                "notes": "some notes here",
            })
            .into(),
            Some(date!(2020 - 1 - 1).with_time(time!(2:2:2))).into(),
        ])
        .build(SqliteQueryBuilder);

    let result = conn.execute(
        sql.as_str(),
        RusqliteValues::from(values).as_params().as_slice(),
    );
    println!("Insert into character: {:?}\n", result);
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
        .build(SqliteQueryBuilder);

    println!("Select one from character:");
    let mut stmt = conn.prepare(sql.as_str())?;
    let mut rows = stmt.query(RusqliteValues::from(values).as_params().as_slice())?;
    while let Some(row) = rows.next()? {
        let item = CharacterStructChrono::from(row);
        println!("{:?}", item);

        let item = CharacterStructTime::from(row);
        println!("{:?}", item);
    }
    println!();

    // Update

    let (sql, values) = Query::update()
        .table(Character::Table)
        .values(vec![(Character::FontSize, 24.into())])
        .and_where(Expr::col(Character::Id).eq(id))
        .build(SqliteQueryBuilder);

    let result = conn.execute(
        sql.as_str(),
        RusqliteValues::from(values).as_params().as_slice(),
    );
    println!("Update character: {:?}\n", result);

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
        .build(SqliteQueryBuilder);

    println!("Select one from character:");
    let mut stmt = conn.prepare(sql.as_str())?;
    let mut rows = stmt.query(RusqliteValues::from(values).as_params().as_slice())?;
    while let Some(row) = rows.next()? {
        let item = CharacterStructChrono::from(row);
        println!("{:?}", item);

        let item = CharacterStructTime::from(row);
        println!("{:?}", item);
    }
    println!();

    // Count

    let (sql, values) = Query::select()
        .from(Character::Table)
        .expr(Func::count(Expr::col(Character::Id)))
        .build(SqliteQueryBuilder);

    print!("Count character: ");
    let mut stmt = conn.prepare(sql.as_str())?;
    let mut rows = stmt.query(RusqliteValues::from(values).as_params().as_slice())?;
    let count: i64 = if let Some(row) = rows.next()? {
        row.get_unwrap(0)
    } else {
        0
    };
    println!("{}", count);
    println!();

    // Delete

    let (sql, values) = Query::delete()
        .from_table(Character::Table)
        .and_where(Expr::col(Character::Id).eq(id))
        .build(SqliteQueryBuilder);

    let result = conn.execute(
        sql.as_str(),
        RusqliteValues::from(values).as_params().as_slice(),
    );
    println!("Delete character: {:?}", result);

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
        let created = PrimitiveDateTime::parse(created, "%Y-%m-%d %H:%M:%S").ok();
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
