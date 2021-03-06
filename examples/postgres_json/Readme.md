# SeaQuery Postgres JSON with DateTime example

Running:
```sh
cargo run
```

Example output:
```
DROP TABLE IF EXISTS "document"; CREATE TABLE IF NOT EXISTS "document" ( "id" serial NOT NULL PRIMARY KEY, "json_field" jsonb, "timestamp" timestamp )
Create table document: ()

Insert into document: Ok(1)

Select one from document:
DocumentStruct { id: 1, json_field: Object({"a": Number(25.0), "b": String("whatever"), "c": Object({"another": String("object"), "bla": Number(1)})}), timestamp: 2020-01-01T02:02:02 }
```