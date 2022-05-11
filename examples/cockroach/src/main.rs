use postgres::{Client, NoTls, Row};
use sea_query::{
    ColumnDef, Expr, Func, Iden, Order, PostgresDriver, PostgresQueryBuilder, Query, Table,
};

fn main() {
    let mut client = Client::connect("postgresql://root:@localhost/query", NoTls).unwrap();

    // Schema

    let sql = [
        Table::drop()
            .table(Character::Table)
            .if_exists()
            .build(PostgresQueryBuilder),
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
            .col(ColumnDef::new(Character::FontSize).integer())
            .col(ColumnDef::new(Character::Character).string())
            .build(PostgresQueryBuilder),
    ]
    .join("; ");

    let result = client.batch_execute(&sql).unwrap();
    println!("Create table character: {:?}\n", result);

    // Create

    let (sql, values) = Query::insert()
        .into_table(Character::Table)
        .columns([Character::Character, Character::FontSize])
        .values_panic(vec!["A".into(), 12i64.into()])
        .returning_col(Character::Id)
        .build(PostgresQueryBuilder);

    let row = client.query_one(sql.as_str(), &values.as_params()).unwrap();
    let id: i64 = row.try_get(0).unwrap();
    println!("Insert into character: last_insert_id = {}\n", id);

    // Read

    let (sql, values) = Query::select()
        .columns([Character::Id, Character::Character, Character::FontSize])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build(PostgresQueryBuilder);

    let rows = client.query(sql.as_str(), &values.as_params()).unwrap();
    println!("Select one from character:");
    for row in rows.into_iter() {
        let item = CharacterStruct::from(row);
        println!("{:?}", item);
    }
    println!();

    // Update

    let (sql, values) = Query::update()
        .table(Character::Table)
        .values(vec![(Character::FontSize, 24i64.into())])
        .and_where(Expr::col(Character::Id).eq(id))
        .build(PostgresQueryBuilder);

    let result = client.execute(sql.as_str(), &values.as_params());
    println!("Update character: {:?}\n", result);

    // Read

    let (sql, values) = Query::select()
        .columns([Character::Id, Character::Character, Character::FontSize])
        .from(Character::Table)
        .order_by(Character::Id, Order::Desc)
        .limit(1)
        .build(PostgresQueryBuilder);

    let rows = client.query(sql.as_str(), &values.as_params()).unwrap();
    println!("Select one from character:");
    for row in rows.into_iter() {
        let item = CharacterStruct::from(row);
        println!("{:?}", item);
    }
    println!();

    // Count

    let (sql, values) = Query::select()
        .from(Character::Table)
        .expr(Func::count(Expr::col(Character::Id)))
        .build(PostgresQueryBuilder);

    let row = client.query_one(sql.as_str(), &values.as_params()).unwrap();
    print!("Count character: ");
    let count: i64 = row.try_get(0).unwrap();
    println!("{}", count);
    println!();

    // Delete

    let (sql, values) = Query::delete()
        .from_table(Character::Table)
        .and_where(Expr::col(Character::Id).eq(id))
        .build(PostgresQueryBuilder);

    let result = client.execute(sql.as_str(), &values.as_params());
    println!("Delete character: {:?}", result);
}

#[derive(Iden)]
enum Character {
    Table,
    Id,
    Character,
    FontSize,
}

#[derive(Debug)]
#[allow(dead_code)]
struct CharacterStruct {
    id: i64,
    character: String,
    font_size: i64,
}

impl From<Row> for CharacterStruct {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            character: row.get("character"),
            font_size: row.get("font_size"),
        }
    }
}
