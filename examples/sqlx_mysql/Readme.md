# SeaQuery SQLx Postgres example

Running:
```sh
cargo run
```

Example output:
```
Create table character: Ok(MySqlQueryResult { rows_affected: 0, last_insert_id: 0 })

Insert into character: Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 1 })

Select one from character:
CharacterStruct { id: 1, character: "A", font_size: 12 }

Update character: Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 0 })

Select one from character:
CharacterStruct { id: 1, character: "A", font_size: 24 }

Count character: 1

Delete character: Ok(MySqlQueryResult { rows_affected: 1, last_insert_id: 0 })
```