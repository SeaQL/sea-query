# SeaQuery SQLx SQLite example

Running:
```sh
cargo run
```

Example output:
```
Create table character: Ok(SqliteQueryResult { changes: 0, last_insert_rowid: 0 })

Insert into character: last_insert_id = 1

Select one from character:
CharacterStruct { id: 1, character: "A", font_size: 12 }

Update character: Ok(SqliteQueryResult { changes: 1, last_insert_rowid: 1 })

Select one from character:
CharacterStruct { id: 1, character: "A", font_size: 24 }

Count character: 1

Delete character: Ok(SqliteQueryResult { changes: 1, last_insert_rowid: 1 })
```