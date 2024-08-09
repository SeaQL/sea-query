# SeaQuery Postgres JSON with DateTime example

Running:
```sh
cargo run
```

Example output:
```
DROP TABLE IF EXISTS "document"; CREATE TABLE IF NOT EXISTS "document" ( "id" serial NOT NULL PRIMARY KEY, "uuid" uuid, "json_field" jsonb, "timestamp" timestamp, "timestamp_with_time_zone" timestamp with time zone, "decimal" decimal, "array" integer[] ); CREATE INDEX "partial_index_small_decimal" ON "document" ("decimal") WHERE NOT "decimal" < 11
Create table document: Ok(())

Insert into document: Ok(1)

Select one from document:
DocumentStruct { id: 1, json_field: Object({"a": Number(25.0), "b": String("whatever"), "c": Object({"another": String("object"), "bla": Number(1)})}), timestamp: 2020-01-01T02:02:02 }
```