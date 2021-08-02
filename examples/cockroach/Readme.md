# SeaQuery Cockroach example

Postgres' INT creates a 32-bit signed integer, but cockroach's INT makes a 64-bit one. That means you need to use i64 and not i32 (don't forget about literals, see line 40 of `main.rs`).

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

Count character: 1

Delete character: Ok(1)
```
