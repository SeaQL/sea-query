# SeaQuery Postgres example

Running:
```sh
cargo run
```

Example output:
```
Create table character: ()

Insert into character: Ok(1)

Select one from character:
CharacterStruct { id: 1, character: "A", font_size: 12 }

Update character: Ok(1)

Select one from character:
CharacterStruct { id: 1, character: "A", font_size: 24 }

Delete character: Ok(1)

Count character: 0
```