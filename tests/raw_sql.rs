use sea_query::Values;

#[test]
fn test_raw_sql_1() {
    struct B {
        i: String,
    }

    fn call_pg(a: i32) {
        let b = B {
            i: "hello".to_owned(),
        };
        let c = [1, 2, 3];
        let query = sea_query::raw_sql!(
            seaql::postgres::query,
            r#"SELECT {a}, {b.i} FROM "bar" WHERE "id" in ({..c})"#
        );
        assert_eq!(
            query.sql,
            r#"SELECT $1, $2 FROM "bar" WHERE "id" in ($3, $4, $5)"#
        );
        assert_eq!(
            query.values,
            Values(vec![
                12.into(),
                "hello".into(),
                1.into(),
                2.into(),
                3.into(),
            ])
        );
    }

    call_pg(12);

    fn call_sqlite(a: i32) {
        let b = B {
            i: "hello".to_owned(),
        };
        let c = [1, 2, 3];
        let query = sea_query::raw_sql!(
            seaql::sqlite::query,
            r#"SELECT {a}, {b.i} FROM "bar" WHERE "id" in ({..c})"#
        );
        assert_eq!(
            query.sql,
            r#"SELECT ?, ? FROM "bar" WHERE "id" in (?, ?, ?)"#
        );
        assert_eq!(
            query.values,
            Values(vec![
                12.into(),
                "hello".into(),
                1.into(),
                2.into(),
                3.into(),
            ])
        );
    }

    call_sqlite(12);
}

#[test]
fn test_raw_sql_1a() {
    let a = 1;
    struct B {
        b: i32,
    }
    let b = B { b: 2 };
    let c = "A";
    let d = vec![3, 4, 5];

    let query = sea_query::raw_query!(
        PostgresQueryBuilder,
        r#"SELECT ("size_w" + {a}) * {b.b} FROM "glyph"
        WHERE "image" LIKE {c} AND "id" IN ({..d})"#
    );
    assert_eq!(
        query.sql,
        r#"SELECT ("size_w" + $1) * $2 FROM "glyph"
        WHERE "image" LIKE $3 AND "id" IN ($4, $5, $6)"#
    );
    assert_eq!(
        query.values,
        Values(vec![
            1.into(),
            2.into(),
            "A".into(),
            3.into(),
            4.into(),
            5.into()
        ])
    );
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

    let s;
    let query = sea_query::raw_sql!(seaql::postgres::query, s = r#"SELECT {a.b.c}"#);

    assert_eq!(query.sql, r#"SELECT $1"#);
    assert_eq!(query.values, Values(vec![42.into()]));
}

#[test]
fn test_raw_sql_3() {
    // this is not real SQL but doesn't matter

    let a: i32 = 1;
    let b: u8 = 2;
    let c = [3u8, 4, 5]; // bytes are bind as 1 item
    let c: &[u8] = &c; // sea-query doesn't support [u8] array as value
    let d = vec![3u8, 4, 5]; // bytes are bind as 1 item
    let e = vec![5i32, 6, 7]; // vec is expanded
    let f = &e; // just a borrow

    let sql;
    let query = sea_query::raw_sql!(
        seaql::sqlite::query,
        sql = r#"A = {a}, B = {b}, C = {c}, D = ({d}), E = ({..e}), F = ({..f})"#
    );
    assert_eq!(
        sql,
        r#"A = ?, B = ?, C = ?, D = (?), E = (?, ?, ?), F = (?, ?, ?)"#
    );
    assert_eq!(
        query.values,
        Values(vec![
            1i32.into(),
            2u8.into(),
            vec![3u8, 4, 5].into(), // bytes
            vec![3u8, 4, 5].into(), // bytes
            5.into(),
            6.into(),
            7.into(),
            5.into(),
            6.into(),
            7.into(),
        ])
    );
}

#[test]
fn test_raw_sql_4() {
    // just to test no-op
    let sql;
    let query = sea_query::raw_sql!(
        seaql::sqlite::query,
        sql = r#"SELECT "character".*, "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id""#
    );

    assert_eq!(
        query.sql,
        r#"SELECT "character".*, "font"."name" FROM "character" INNER JOIN "font" ON "character"."font_id" = "font"."id""#
    );
    assert_eq!(query.values, Values(vec![]));
}

#[test]
fn test_raw_sql_5() {
    // the example in readme
    let a = 1;
    let b = 2;
    let c = "A";
    let d = vec![3i32, 4, 5];

    let sql;
    let query = sea_query::raw_sql!(
        seaql::sqlite::query,
        sql = r#"SELECT ("size_w" + {a}) * {b} FROM "glyph" WHERE "image" LIKE {c} AND "id" IN ({..d})"#
    );
    assert_eq!(
        sql,
        r#"SELECT ("size_w" + ?) * ? FROM "glyph" WHERE "image" LIKE ? AND "id" IN (?, ?, ?)"#
    );
    assert_eq!(
        query.values,
        Values(vec![
            1.into(),
            2.into(),
            "A".into(),
            3.into(),
            4.into(),
            5.into()
        ])
    );

    let query = sea_query::raw_sql!(
        seaql::postgres::query,
        r#"SELECT ("size_w" + {a}) * {b} FROM "glyph" WHERE "image" LIKE {c} AND "id" IN ({..d})"#
    );
    assert_eq!(
        query.sql,
        r#"SELECT ("size_w" + $1) * $2 FROM "glyph" WHERE "image" LIKE $3 AND "id" IN ($4, $5, $6)"#
    );
    assert_eq!(
        query.values,
        Values(vec![
            1.into(),
            2.into(),
            "A".into(),
            3.into(),
            4.into(),
            5.into()
        ])
    );
}

#[test]
fn test_raw_sql_6() {
    let a = Some(1);
    let b = Option::<i32>::None;
    let c = Some("c".to_owned());
    let d = Option::<String>::None;

    let query = sea_query::raw_sql!(seaql::sqlite::query, r#"SELECT {a}, {b}, {c}, {d}"#);
    assert_eq!(query.sql, r#"SELECT ?, ?, ?, ?"#);
    assert_eq!(
        query.values,
        Values(vec![
            1.into(),
            Option::<i32>::None.into(),
            "c".into(),
            Option::<String>::None.into(),
        ])
    );
}

#[test]
fn test_raw_sql_7() {
    let var = (1, "2".to_owned(), 3);

    let query = sea_query::raw_sql!(
        seaql::postgres::query,
        r#"SELECT {var.0}, {var.1}, {var.2}, {var.0:2}"#
    );
    assert_eq!(query.sql, r#"SELECT $1, $2, $3, $4, $5, $6"#);
    assert_eq!(
        query.values,
        Values(vec![
            1.into(),
            "2".into(),
            3.into(),
            1.into(),
            "2".into(),
            3.into(),
        ])
    );
}

#[test]
fn test_raw_sql_8() {
    let values = (1, "2");

    let query = sea_query::raw_sql!(
        seaql::mysql::query,
        "INSERT INTO `glyph` (`aspect`, `image`) VALUES ({values.0:1})"
    );
    assert_eq!(
        query.sql,
        "INSERT INTO `glyph` (`aspect`, `image`) VALUES (?, ?)"
    );
    assert_eq!(query.values, Values(vec![1.into(), "2".into()]));
}

#[test]
fn test_raw_sql_9() {
    // test vector of tuple expansion
    let values = vec![(1, "2", 3), (4, "5", 6)];
    let z = 42;

    let query = sea_query::raw_sql!(
        seaql::postgres::query,
        r#"INSERT INTO "glyph" ("aspect", "image", "font_size") VALUES {..(values.0:2),} SELECT {z}"#
    );
    assert_eq!(
        query.sql,
        r#"INSERT INTO "glyph" ("aspect", "image", "font_size") VALUES ($1, $2, $3), ($4, $5, $6) SELECT $7"#
    );
    assert_eq!(
        query.values,
        Values(vec![
            1.into(),
            "2".into(),
            3.into(),
            4.into(),
            "5".into(),
            6.into(),
            42.into(),
        ])
    );
}

#[test]
fn test_raw_sql_10() {
    // array expansion, but without tuple expansion
    let values = vec![(1, "2", 3), (4, "5", 6)];
    let z = 42;

    let query = sea_query::raw_sql!(
        seaql::postgres::query,
        r#"INSERT INTO "glyph" ("aspect", "image") VALUES {..(values.0, values.1, values.2),} SELECT {z}"#
    );
    assert_eq!(
        query.sql,
        r#"INSERT INTO "glyph" ("aspect", "image") VALUES ($1, $2, $3), ($4, $5, $6) SELECT $7"#
    );
    assert_eq!(
        query.values,
        Values(vec![
            1.into(),
            "2".into(),
            3.into(),
            4.into(),
            "5".into(),
            6.into(),
            42.into(),
        ])
    );
}

#[test]
fn test_raw_sql_11() {
    struct Item {
        a: i32,
        b: String,
        c: u16,
    }
    // array expansion, but without tuple expansion
    let values = vec![
        Item {
            a: 11,
            b: "1b".to_owned(),
            c: 13,
        },
        Item {
            a: 21,
            b: "2b".to_owned(),
            c: 23,
        },
    ];
    let z = 42;

    let query = sea_query::raw_sql!(
        seaql::postgres::query,
        r#"INSERT INTO "glyph" ("aspect", "image") VALUES {..(values.a, values.b, values.c),} SELECT {z}"#
    );
    assert_eq!(
        query.sql,
        r#"INSERT INTO "glyph" ("aspect", "image") VALUES ($1, $2, $3), ($4, $5, $6) SELECT $7"#
    );
    assert_eq!(
        query.values,
        Values(vec![
            11.into(),
            "1b".into(),
            13u16.into(),
            21.into(),
            "2b".into(),
            23u16.into(),
            42.into(),
        ])
    );
}
