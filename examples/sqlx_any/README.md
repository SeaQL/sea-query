# SeaQuery SQLx Any example

Running:
```sh
cargo run
```

Example output:
```
Create table character: Ok(AnyQueryResult { rows_affected: 0, last_insert_id: Some(0) })

Insert into character: last_insert_id = 1

Select one from character:
CharacterStructChrono { id: 1, character: "A", font_size: 12, created: 2020-08-20T00:00:00 }

Update character: Ok(AnyQueryResult { rows_affected: 1, last_insert_id: Some(1) })

Select one from character:
CharacterStructChrono { id: 1, character: "A", font_size: 24, created: 2020-08-20T00:00:00 }

Count character: 1

Insert into character (with upsert): Ok(AnyQueryResult { rows_affected: 1, last_insert_id: Some(1) })

Select all characters:
CharacterStructChrono { id: 1, character: "B", font_size: 16, created: 2020-08-20T00:00:00 }

Delete character: Ok(AnyQueryResult { rows_affected: 1, last_insert_id: Some(1) })
```
