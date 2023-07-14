# SeaQuery Diesel Sqlite example

Running:

```sh
cargo run
```

Example output:

```
Create table character: Ok(())

Insert into character Ok(4)

Select one from character:
CharacterStructChrono { id: 4, uuid: UUID(3a23c42d-8cd9-4a0f-a8c3-0ced15d42228), name: "A", font_size: 12, meta: Object {"notes": String("some notes here")}, created: Some(2020-01-01T02:02:02) }

Select one from character:
CharacterStructTime { id: 4, uuid: UUID(3a23c42d-8cd9-4a0f-a8c3-0ced15d42228), name: "A", font_size: 12, meta: Object {"notes": String("some notes here")}, created: Some(2020-01-01 2:02:02.0) }


Update character: Ok(1)

Select one from character:
CharacterStructChrono { id: 4, uuid: UUID(3a23c42d-8cd9-4a0f-a8c3-0ced15d42228), name: "A", font_size: 24, meta: Object {"notes": String("some notes here")}, created: Some(2020-01-01T02:02:02) }

Select one from character:
CharacterStructTime { id: 4, uuid: UUID(3a23c42d-8cd9-4a0f-a8c3-0ced15d42228), name: "A", font_size: 24, meta: Object {"notes": String("some notes here")}, created: Some(2020-01-01 2:02:02.0) }


Count character: Ok(CountField { count: 4 })

Delete character: Ok(1)
```
