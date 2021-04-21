# SeaQuery Rusqlite example

Running:
```sh
cargo run
```

Example output:
```
Create table character: Ok()

Insert into character: Ok(1)

Select one from character:
CharacterStruct { id: 1, character: "A", font_size: 12 }

Update character: Ok(1)

Select one from character:
CharacterStruct { id: 1, character: "A", font_size: 24 }

Count character: 1

Delete character: Ok(1)
```