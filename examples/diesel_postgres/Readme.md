# SeaQuery Diesel Postgres example

> WARN: If you enable `with-bigdecimal`, you HAVE to update the version used by default by `diesel`
> otherwise it will fail to build. Use `cargo update -p bigdecimal:0.4.2 --precise 0.3.1`.

Running:

```sh
cargo run
```

Example output:

```
Create table character: Ok(())

Insert into character Ok(2)

Select one from character:
CharacterStructChrono { id: 2, uuid: 6c95bcb8-3411-484c-9b33-f70c97c2a13a, name: "A", font_size: 12, meta: Object {"notes": String("some notes here")}, decimal: 3.141, big_decimal: BigDecimal("3141.000"), created: 2020-08-20T00:00:00, inet: V4(Ipv4Network { addr: 127.0.0.1, prefix: 8 }), mac_address: MacAddress(MacAddress { bytes: [96, 3, 8, 143, 139, 58] }), array_bool: [false, false] }

Select one from character:
CharacterStructTime { id: 2, uuid: 6c95bcb8-3411-484c-9b33-f70c97c2a13a, name: "A", font_size: 12, meta: Object {"notes": String("some notes here")}, decimal: 3.141, big_decimal: BigDecimal("3141.000"), created: 2020-08-20 0:00:00.0, inet: V4(Ipv4Network { addr: 127.0.0.1, prefix: 8 }), mac_address: MacAddress(MacAddress { bytes: [96, 3, 8, 143, 139, 58] }), array_bool: [false, false] }


Update character: Ok(1)

Select one from character:
CharacterStructChrono { id: 2, uuid: 6c95bcb8-3411-484c-9b33-f70c97c2a13a, name: "A", font_size: 24, meta: Object {"notes": String("some notes here")}, decimal: 3.141, big_decimal: BigDecimal("3141.000"), created: 2020-08-20T00:00:00, inet: V4(Ipv4Network { addr: 127.0.0.1, prefix: 8 }), mac_address: MacAddress(MacAddress { bytes: [96, 3, 8, 143, 139, 58] }), array_bool: [false, false] }

Select one from character:
CharacterStructTime { id: 2, uuid: 6c95bcb8-3411-484c-9b33-f70c97c2a13a, name: "A", font_size: 24, meta: Object {"notes": String("some notes here")}, decimal: 3.141, big_decimal: BigDecimal("3141.000"), created: 2020-08-20 0:00:00.0, inet: V4(Ipv4Network { addr: 127.0.0.1, prefix: 8 }), mac_address: MacAddress(MacAddress { bytes: [96, 3, 8, 143, 139, 58] }), array_bool: [false, false] }


Count character: Ok(CountField { count: 2 })

Insert into character (with upsert): Ok(2)

Select all characters:
CharacterStructChrono { id: 2, uuid: 6c95bcb8-3411-484c-9b33-f70c97c2a13a, name: "C", font_size: 24, meta: Object {"notes": String("some notes here")}, decimal: 3.141, big_decimal: BigDecimal("3141.000"), created: 2020-08-20T00:00:00, inet: V4(Ipv4Network { addr: 127.0.0.1, prefix: 8 }), mac_address: MacAddress(MacAddress { bytes: [96, 3, 8, 143, 139, 58] }), array_bool: [false, false] }

CharacterStructChrono { id: 1, uuid: c9384d8e-bbf4-401e-a395-17cee77022fb, name: "B", font_size: 16, meta: Object {"notes": String("some notes here")}, decimal: 3.141, big_decimal: BigDecimal("3141.000"), created: 2020-08-20T00:00:00, inet: V4(Ipv4Network { addr: 127.0.0.1, prefix: 8 }), mac_address: MacAddress(MacAddress { bytes: [96, 3, 8, 143, 139, 58] }), array_bool: [true, false] }

Select all characters:
CharacterStructTime { id: 2, uuid: 6c95bcb8-3411-484c-9b33-f70c97c2a13a, name: "C", font_size: 24, meta: Object {"notes": String("some notes here")}, decimal: 3.141, big_decimal: BigDecimal("3141.000"), created: 2020-08-20 0:00:00.0, inet: V4(Ipv4Network { addr: 127.0.0.1, prefix: 8 }), mac_address: MacAddress(MacAddress { bytes: [96, 3, 8, 143, 139, 58] }), array_bool: [false, false] }

CharacterStructTime { id: 1, uuid: c9384d8e-bbf4-401e-a395-17cee77022fb, name: "B", font_size: 16, meta: Object {"notes": String("some notes here")}, decimal: 3.141, big_decimal: BigDecimal("3141.000"), created: 2020-08-20 0:00:00.0, inet: V4(Ipv4Network { addr: 127.0.0.1, prefix: 8 }), mac_address: MacAddress(MacAddress { bytes: [96, 3, 8, 143, 139, 58] }), array_bool: [true, false] }


Delete character: Ok(1)
```
