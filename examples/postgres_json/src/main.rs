use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use postgres::{Client, NoTls, Row};
use rust_decimal::Decimal;
use sea_query::{ColumnDef, Iden, Order, PostgresDriver, PostgresQueryBuilder, Query, Table};
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
            .build(PostgresQueryBuilder),
    ]
    .join("; ");

    println!("{}", sql);
    let result = client.batch_execute(&sql).unwrap();
    println!("Create table document: {:?}\n", result);

    // Create
    let document = DocumentStruct {
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
        timestamp: NaiveDate::from_ymd(2020, 1, 1).and_hms(2, 2, 2),
        timestamp_with_time_zone: DateTime::parse_from_rfc3339("2020-01-01T02:02:02+08:00")
            .unwrap(),
        decimal: Decimal::from_i128_with_scale(3141i128, 3),
    };
    let (sql, values) = Query::insert()
        .into_table(Document::Table)
        .columns(vec![
            Document::Uuid,
            Document::JsonField,
            Document::Timestamp,
            Document::TimestampWithTimeZone,
            Document::Decimal,
        ])
        .values_panic(vec![
            document.uuid.into(),
            serde_json::to_value(document.json_field).unwrap().into(),
            document.timestamp.into(),
            document.timestamp_with_time_zone.into(),
            document.decimal.into(),
        ])
        .build(PostgresQueryBuilder);

    let result = client.execute(sql.as_str(), &values.as_params());
    println!("Insert into document: {:?}\n", result);

    // Read

    let (sql, values) = Query::select()
        .columns(vec![
            Document::Id,
            Document::Uuid,
            Document::JsonField,
            Document::Timestamp,
            Document::TimestampWithTimeZone,
            Document::Decimal,
        ])
        .from(Document::Table)
        .order_by(Document::Id, Order::Desc)
        .limit(1)
        .build(PostgresQueryBuilder);

    let rows = client.query(sql.as_str(), &values.as_params()).unwrap();
    println!("Select one from document:");
    for row in rows.into_iter() {
        let item = DocumentStruct::from(row);
        println!("{:?}", item);
    }
    println!();
}

#[derive(Clone, Copy, Iden)]
enum Document {
    Table,
    Id,
    Uuid,
    JsonField,
    Timestamp,
    TimestampWithTimeZone,
    Decimal,
}

#[derive(Debug)]
struct DocumentStruct {
    id: i32,
    uuid: Uuid,
    json_field: serde_json::Value,
    timestamp: NaiveDateTime,
    timestamp_with_time_zone: DateTime<FixedOffset>,
    decimal: Decimal,
}

impl From<Row> for DocumentStruct {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            uuid: row.get("uuid"),
            json_field: row.get("json_field"),
            timestamp: row.get("timestamp"),
            timestamp_with_time_zone: row.get("timestamp_with_time_zone"),
            decimal: row.get("decimal"),
        }
    }
}
