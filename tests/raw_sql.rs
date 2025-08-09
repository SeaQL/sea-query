#[test]
fn test_raw_sql_1() {
    struct B {
        i: String,
    }

    fn call(a: i32) {
        let b = B {
            i: "hello".to_owned(),
        };
        let c = [1, 2, 3];
        let query = sea_query::raw_sql!(
            seaql::postgres,
            r#"SELECT {a}, {b.i} FROM "bar" WHERE "id" in ({..c})"#
        );
        assert_eq!(
            format!("{query:?}"),
            [
                r#"sql!(SELECT $1, $2 FROM "bar" WHERE "id" in ($3, $4, $5))"#,
                r#"    .params(12, "hello", 1, 2, 3)"#,
            ]
            .join("\n")
        );
    }

    call(12);
}

#[test]
fn test_raw_sql_2() {
    struct A {
        b: B,
    }

    struct B {
        c: i32,
    }

    let a = A { b: B { c: 42 } };

    let query = sea_query::raw_sql!(seaql::postgres, r#"SELECT {..a.b.c}"#);
    assert_eq!(
        format!("{query:?}"),
        [r#"sql!(SELECT $1)"#, r#"    .params(42)"#].join("\n")
    );
}

#[test]
fn test_raw_sql_3() {
    let a: i32 = 1;
    let b: u8 = 2;
    let c = [3u8, 4, 5]; // bytes are bind as 1 item
    let d = vec![3u8, 4, 5]; // bytes are bind as 1 item
    let e = vec![5i32, 6, 7]; // vec is expanded
    let f = &e; // just a borrow

    let query = sea_query::raw_sql!(
        seaql::sqlite,
        r#"A = {a}, B = {b}, C = {c}, D = ({d}), E = ({e}), F = ({f})"#
    );
    assert_eq!(
        format!("{query:?}"),
        [
            "sql!(A = ?, B = ?, C = ?, D = (?), E = (?, ?, ?), F = (?, ?, ?))",
            "    .params(1, 2, [3, 4, 5], [3, 4, 5], 5, 6, 7, 5, 6, 7)",
            //                 ^^^^^^^^^  ^^^^^^^^^
            //                 | bytes |  | bytes |
        ]
        .join("\n")
    );
}

#[test]
fn test_raw_sql_4() {
    // just to test no-op
    let query = sea_query::raw_sql!(
        seaql::sqlite,
        r#"SELECT "character".*, "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id""#
    );
    assert_eq!(
        format!("{query:?}"),
        [
            r#"sql!(SELECT "character".*, "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id")"#,
            "    .params()",
        ]
        .join("\n")
    );
}

#[test]
fn test_raw_sql_5() {
    // the example in readme
    let a = 1;
    let b = 2;
    let c = "A";
    let d = vec![3i32, 4, 5];

    let query = sea_query::raw_sql!(
        seaql::sqlite,
        r#"SELECT ("size_w" + {a}) * {b} FROM "glyph" WHERE "image" LIKE {c} AND "id" IN ({d})"#
    );
    assert_eq!(
        format!("{query:?}"),
        [
            r#"sql!(SELECT ("size_w" + ?) * ? FROM "glyph" WHERE "image" LIKE ? AND "id" IN (?, ?, ?))"#,
            r#"    .params(1, 2, "A", 3, 4, 5)"#,
        ]
        .join("\n")
    );

    // postgres has to expand array manually
    let query = sea_query::raw_sql!(
        seaql::postgres,
        r#"SELECT ("size_w" + {a}) * {b} FROM "glyph" WHERE "image" LIKE {c} AND "id" IN ({..d})"#
    );
    assert_eq!(
        format!("{query:?}"),
        [
            r#"sql!(SELECT ("size_w" + $1) * $2 FROM "glyph" WHERE "image" LIKE $3 AND "id" IN ($4, $5, $6))"#,
            r#"    .params(1, 2, "A", 3, 4, 5)"#,
        ]
        .join("\n")
    );
}
