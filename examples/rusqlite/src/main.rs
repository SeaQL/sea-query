use rusqlite::{Connection, Result, Row};
use sea_query::{ColumnDef, Expr, Func, Iden, Order, SqliteQueryBuilder, Query, Table};

mod sea_query_rusqlite {
    use rusqlite::{Result, ToSql, types::ToSqlOutput};
    use sea_query::{Values, Value};

    pub struct RusqliteValue(pub Value);

    pub struct RusqliteValues(pub Vec<RusqliteValue>);

    impl From<Values> for RusqliteValues {
        fn from(values: Values) -> RusqliteValues {
            RusqliteValues(values.0.into_iter().map(|v| RusqliteValue(v)).collect())
        }
    }

    impl<'a> RusqliteValues {
        pub fn as_params(&'a self) -> Vec<&'a dyn ToSql> {
            self.0.iter().map(|x| {
                let y: &dyn ToSql = x;
                y
            }).collect()
        }
    }

    impl ToSql for RusqliteValue {
        fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
            match &self.0 {
                Value::Null => None::<bool>.to_sql(),
                Value::Bool(v) => v.to_sql(),
                Value::TinyInt(v) => v.to_sql(),
                Value::SmallInt(v) => v.to_sql(),
                Value::Int(v) => v.to_sql(),
                Value::BigInt(v) => v.to_sql(),
                Value::TinyUnsigned(v) => v.to_sql(),
                Value::SmallUnsigned(v) => v.to_sql(),
                Value::Unsigned(v) => v.to_sql(),
                Value::BigUnsigned(v) => v.to_sql(),
                Value::Float(v) => v.to_sql(),
                Value::Double(v) => v.to_sql(),
                Value::String(v) => v.as_str().to_sql(),
                Value::Bytes(v) => v.as_ref().to_sql(),
                _ => {
                    if self.0.is_json() {
                        (*self.0.as_ref_json()).to_sql()
                    } else if self.0.is_date_time() {
                        (*self.0.as_ref_date_time()).to_sql()
                    } else if self.0.is_uuid() {
                        (*self.0.as_ref_uuid()).to_sql()
                    } else {
                        unimplemented!();
                    }
                },
            }
        }
    }
}

use sea_query_rusqlite::*;

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
            .col(ColumnDef::new(Character::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(Character::FontSize).integer())
            .col(ColumnDef::new(Character::Character).string())
            .build(SqliteQueryBuilder),
    ].join("; ");

    let result = conn.execute_batch(&sql)?;
    println!("Create table character: Ok()");
    println!();

    // Create

    let (sql, values) = Query::insert()
        .into_table(Character::Table)
        .columns(vec![
            Character::Character, Character::FontSize
        ])
        .values_panic(vec![
            "A".into(),
            12.into(),
        ])
        .build(SqliteQueryBuilder);

    let result = conn.execute(sql.as_str(), RusqliteValues::from(values).as_params().as_slice());
    println!("Insert into character: {:?}\n", result);
    let id = conn.last_insert_rowid();

    // Read

    let (sql, values) = Query::select()
        .columns(vec![
            Character::Id, Character::Character, Character::FontSize
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build(SqliteQueryBuilder);

    println!("Select one from character:");
    let mut stmt = conn.prepare(sql.as_str())?;
    let mut rows = stmt.query(RusqliteValues::from(values).as_params().as_slice())?;
    while let Some(row) = rows.next()? {
        let item = CharacterStruct::from(row);
        println!("{:?}", item);
    }
    println!();

    // Update

    let (sql, values) = Query::update()
        .table(Character::Table)
        .values(vec![
            (Character::FontSize, 24.into()),
        ])
        .and_where(Expr::col(Character::Id).eq(id))
        .build(SqliteQueryBuilder);

    let result = conn.execute(sql.as_str(), RusqliteValues::from(values).as_params().as_slice());
    println!("Update character: {:?}\n", result);

    // Read

    let (sql, values) = Query::select()
        .columns(vec![
            Character::Id, Character::Character, Character::FontSize
        ])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build(SqliteQueryBuilder);

    println!("Select one from character:");
    let mut stmt = conn.prepare(sql.as_str())?;
    let mut rows = stmt.query(RusqliteValues::from(values).as_params().as_slice())?;
    while let Some(row) = rows.next()? {
        let item = CharacterStruct::from(row);
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

    let result = conn.execute(sql.as_str(), RusqliteValues::from(values).as_params().as_slice());
    println!("Delete character: {:?}", result);

    Ok(())
}

#[derive(Iden)]
enum Character {
    Table,
    Id,
    Character,
    FontSize,
}

#[derive(Debug)]
struct CharacterStruct {
    id: i32,
    character: String,
    font_size: i32,
}

impl From<&Row<'_>> for CharacterStruct {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get_unwrap("id"),
            character: row.get_unwrap("character"),
            font_size: row.get_unwrap("font_size"),
        }
    }
}
