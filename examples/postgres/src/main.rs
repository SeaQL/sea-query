use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};

use postgres::{Client, NoTls, Row};
use rust_decimal::Decimal;
use sea_query::{
    ColumnDef, ColumnType, Iden, IntoIden, Order, PostgresQueryBuilder, Query, Table,
};
use sea_query::extension::postgres::{
    FunctionReturns, PgFunctionStmt, PgTriggerStmt, TriggerEvent,
};
use sea_query_postgres::PostgresBinder;
use time::{
    OffsetDateTime, PrimitiveDateTime,
    macros::{date, offset, time},
};
use uuid::Uuid;

fn main() {
    let mut client = Client::connect("postgresql://sea:sea@localhost/query", NoTls).unwrap();

    // Schema

    let sql = [
        Table::drop()
            .table(Document::Table)
            .if_exists()
            .build(PostgresQueryBuilder),
        Table::create()
            .table(Document::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Document::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Document::Uuid).uuid())
            .col(ColumnDef::new(Document::JsonField).json_binary())
            .col(ColumnDef::new(Document::Timestamp).timestamp())
            .col(ColumnDef::new(Document::TimestampWithTimeZone).timestamp_with_time_zone())
            .col(ColumnDef::new(Document::Decimal).decimal())
            .col(ColumnDef::new(Document::Array).array(ColumnType::Integer))
            .build(PostgresQueryBuilder),
    ]
    .join("; ");

    println!("{sql}");
    let result = client.batch_execute(&sql);
    println!("Create table document: {result:?}\n");

    // Create
    let document_chrono = DocumentStructChrono {
        id: 1,
        uuid: Uuid::new_v4(),
        json_field: serde_json::json! {{
            "a": 25.0,
            "b": "whatever",
            "c": {
                "another": "object",
                "bla": 1
            }
        }},
        timestamp: NaiveDate::from_ymd_opt(2020, 1, 1)
            .unwrap()
            .and_hms_opt(2, 2, 2)
            .unwrap(),
        timestamp_with_time_zone: DateTime::parse_from_rfc3339("2020-01-01T02:02:02+08:00")
            .unwrap(),
        decimal: Decimal::from_i128_with_scale(3141i128, 3),
        array: vec![3, 4, 5, 6],
    };
    let document_time = DocumentStructTime {
        id: 2,
        uuid: Uuid::new_v4(),
        json_field: serde_json::json! {{
            "a": 25.0,
            "b": "whatever",
            "c": {
                "another": "object",
                "bla": 1
            }
        }},
        timestamp: date!(2020 - 1 - 1).with_time(time!(2:2:2)),
        timestamp_with_time_zone: date!(2020 - 01 - 01)
            .with_time(time!(02:02:02))
            .assume_utc()
            .to_offset(offset!(+8)),
        decimal: Decimal::from_i128_with_scale(3141i128, 3),
        array: vec![3, 4, 5, 6],
    };

    let (sql, values) = Query::insert()
        .into_table(Document::Table)
        .columns([
            Document::Uuid,
            Document::JsonField,
            Document::Timestamp,
            Document::TimestampWithTimeZone,
            Document::Decimal,
            Document::Array,
        ])
        .values_panic([
            document_chrono.uuid.into(),
            serde_json::to_value(document_chrono.json_field)
                .unwrap()
                .into(),
            document_chrono.timestamp.into(),
            document_chrono.timestamp_with_time_zone.into(),
            document_chrono.decimal.into(),
            document_chrono.array.into(),
        ])
        .values_panic([
            document_time.uuid.into(),
            serde_json::to_value(document_time.json_field)
                .unwrap()
                .into(),
            document_time.timestamp.into(),
            document_time.timestamp_with_time_zone.into(),
            document_time.decimal.into(),
            document_time.array.into(),
        ])
        .build_postgres(PostgresQueryBuilder);

    let result = client.execute(sql.as_str(), &values.as_params());
    println!("Insert into document: {result:?}\n");

    // Read

    let (sql, values) = Query::select()
        .columns([
            Document::Id,
            Document::Uuid,
            Document::JsonField,
            Document::Timestamp,
            Document::TimestampWithTimeZone,
            Document::Decimal,
            Document::Array,
        ])
        .from(Document::Table)
        .order_by(Document::Id, Order::Desc)
        .limit(1)
        .build_postgres(PostgresQueryBuilder);

    let rows = client.query(sql.as_str(), &values.as_params()).unwrap();
    println!("Select one from document:");
    for row in rows.iter() {
        let item = DocumentStructChrono::from(row);
        println!("{item:?}");

        let item = DocumentStructTime::from(row);
        println!("{item:?}");
    }
    println!();

    // Postgres Extension: Function and Trigger

    // 1. Drop trigger & function if they exist
    let drop_trigger_sql = PgTriggerStmt::drop()
        .name("doc_trigger")
        .table(Document::Table)
        .if_exists()
        .to_string(PostgresQueryBuilder);
    println!("Drop trigger SQL: {drop_trigger_sql}");
    let _ = client.execute(&drop_trigger_sql, &[]);

    let drop_function_sql = PgFunctionStmt::drop()
        .name("doc_trigger_func")
        .if_exists()
        .to_string(PostgresQueryBuilder);
    println!("Drop function SQL: {drop_function_sql}");
    let _ = client.execute(&drop_function_sql, &[]);

    // 2. Create function that returns TRIGGER
    let create_function_sql = PgFunctionStmt::create()
        .or_replace()
        .name("doc_trigger_func")
        .returns(FunctionReturns::Type(ColumnType::Custom("TRIGGER".into_iden())))
        .language("plpgsql")
        .as_definition("BEGIN RETURN NEW; END;")
        .to_string(PostgresQueryBuilder);
    println!("Create function SQL: {create_function_sql}");
    let _ = client.execute(&create_function_sql, &[]);

    // 3. Create trigger executing that function
    let create_trigger_sql = PgTriggerStmt::create()
        .name("doc_trigger")
        .before()
        .event(TriggerEvent::Insert)
        .table(Document::Table)
        .for_each_row()
        .function("doc_trigger_func")
        .to_string(PostgresQueryBuilder);
    println!("Create trigger SQL: {create_trigger_sql}");
    let _ = client.execute(&create_trigger_sql, &[]);

    // 4. Alter trigger name
    let alter_trigger_sql = PgTriggerStmt::alter()
        .name("doc_trigger")
        .table(Document::Table)
        .rename_to("doc_trigger_new")
        .to_string(PostgresQueryBuilder);
    println!("Alter trigger SQL: {alter_trigger_sql}");
    let _ = client.execute(&alter_trigger_sql, &[]);

    // 5. Clean up: Drop new trigger and function
    let drop_new_trigger_sql = PgTriggerStmt::drop()
        .name("doc_trigger_new")
        .table(Document::Table)
        .to_string(PostgresQueryBuilder);
    println!("Drop new trigger SQL: {drop_new_trigger_sql}");
    let _ = client.execute(&drop_new_trigger_sql, &[]);

    let drop_function_final_sql = PgFunctionStmt::drop()
        .name("doc_trigger_func")
        .to_string(PostgresQueryBuilder);
    println!("Drop function SQL: {drop_function_final_sql}");
    let _ = client.execute(&drop_function_final_sql, &[]);
}

#[derive(Iden)]
enum Document {
    Table,
    Id,
    Uuid,
    JsonField,
    Timestamp,
    TimestampWithTimeZone,
    Decimal,
    Array,
}

#[derive(Debug)]
#[allow(dead_code)]
struct DocumentStructChrono {
    id: i32,
    uuid: Uuid,
    json_field: serde_json::Value,
    timestamp: NaiveDateTime,
    timestamp_with_time_zone: DateTime<FixedOffset>,
    decimal: Decimal,
    array: Vec<i32>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct DocumentStructTime {
    id: i32,
    uuid: Uuid,
    json_field: serde_json::Value,
    timestamp: PrimitiveDateTime,
    timestamp_with_time_zone: OffsetDateTime,
    decimal: Decimal,
    array: Vec<i32>,
}

impl From<&Row> for DocumentStructChrono {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            uuid: row.get("uuid"),
            json_field: row.get("json_field"),
            timestamp: row.get("timestamp"),
            timestamp_with_time_zone: row.get("timestamp_with_time_zone"),
            decimal: row.get("decimal"),
            array: row.get("array"),
        }
    }
}
impl From<&Row> for DocumentStructTime {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            uuid: row.get("uuid"),
            json_field: row.get("json_field"),
            timestamp: row.get("timestamp"),
            timestamp_with_time_zone: row.get("timestamp_with_time_zone"),
            decimal: row.get("decimal"),
            array: row.get("array"),
        }
    }
}
