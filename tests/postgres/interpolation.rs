use super::*;
use pretty_assertions::assert_eq;

#[test]
fn custom_values_in_arrays_are_bound() {
    let (statement, values) = Query::select()
        .expr(Expr::cust_with_values("ARRAY[$1, $1]", [42]))
        .build(PostgresQueryBuilder);

    assert_eq!(statement, "SELECT ARRAY[$1, $2]");
    assert_eq!(values, Values(vec![42.into(), 42.into()]));
}

#[test]
fn custom_values_preserve_dollar_quoted_strings() {
    for (expression, statement) in [
        (
            "txt ~ $$$$[$1]$$$$ AND id = $1",
            "SELECT $1, txt ~ $$[$1]$$ AND id = $2",
        ),
        (
            "txt ~ $$tag$[$1]$$tag$ AND id = $1",
            "SELECT $1, txt ~ $tag$[$1]$tag$ AND id = $2",
        ),
        (
            "txt ~ $$文_tag$[$1]$$文_tag$ AND id = $1",
            "SELECT $1, txt ~ $文_tag$[$1]$文_tag$ AND id = $2",
        ),
        (
            "txt ~ $$tag$$[$1]$$tag$ AND id = $1",
            "SELECT $1, txt ~ $tag$$[$1]$tag$ AND id = $2",
        ),
        (
            "txt ~ $$tag$foo$tag$ AND id = $1",
            "SELECT $1, txt ~ $tag$foo$tag$ AND id = $2",
        ),
        (
            "txt ~ $$$$foo$$ AND id = $1",
            "SELECT $1, txt ~ $$foo$$ AND id = $2",
        ),
        (
            "txt ~ $$tag$foo$$tag$ AND id = $1",
            "SELECT $1, txt ~ $tag$foo$$tag$ AND id = $2",
        ),
        (
            "txt ~ $$$$a + $$ x $$$$ AND id = $1",
            "SELECT $1, txt ~ $$a + $ x $$ AND id = $2",
        ),
        ("$$$$ _$$ x $$$$ + $1", "SELECT $1, $$ _$ x $$ + $2"),
    ] {
        assert_eq!(
            Query::select()
                .expr(Expr::val(7))
                .expr(Expr::cust_with_values(expression, [42]))
                .build(PostgresQueryBuilder),
            (statement.into(), Values(vec![7.into(), 42.into()]))
        );
    }
}

#[test]
fn custom_values_bind_after_dollar_quoted_strings() {
    for (expression, statement) in [
        (
            "txt ~ $$tag$--$tag$ AND id = $1",
            "SELECT $1, txt ~ $tag$--$tag$ AND id = $2",
        ),
        (
            "txt ~ $$$$/*$$ AND id = $1",
            "SELECT $1, txt ~ $$/*$$ AND id = $2",
        ),
        (
            "txt ~ $$$$'$$ || '$1' || $1",
            "SELECT $1, txt ~ $$'$$ || '$1' || $2",
        ),
        ("$$$$'$$ _$$ + $1", "SELECT $1, $$'$$ _$$ + $2"),
    ] {
        assert_eq!(
            Query::select()
                .expr(Expr::val(7))
                .expr(Expr::cust_with_values(expression, [42]))
                .build(PostgresQueryBuilder),
            (statement.into(), Values(vec![7.into(), 42.into()]))
        );
    }
}

#[test]
fn custom_values_escape_dollar_signs() {
    assert_eq!(
        Query::select()
            .expr(Expr::cust_with_values("$$ $1 $$", [42]))
            .build(PostgresQueryBuilder),
        ("SELECT $ $1 $".into(), Values(vec![42.into()]))
    );

    for (expression, statement, inlined) in [
        ("$$1 + $1", "SELECT $1, $1 + $2", "SELECT 7, $1 + 42"),
        (
            "'文字' || $$1 || $1",
            "SELECT $1, '文字' || $1 || $2",
            "SELECT 7, '文字' || $1 || 42",
        ),
    ] {
        let query = Query::select()
            .expr(Expr::val(7))
            .expr(Expr::cust_with_values(expression, [42]))
            .to_owned();

        assert_eq!(
            query.build(PostgresQueryBuilder),
            (statement.into(), Values(vec![7.into(), 42.into()]))
        );
        assert_eq!(query.to_string(PostgresQueryBuilder), inlined);
    }
}

#[test]
fn dollar_quoted_strings_preserve_parameter_text() {
    for (sql, expected) in [
        ("SELECT $$[$1]$$, $1", "SELECT $$[$1]$$, 42"),
        ("SELECT $$a$$$$b$$, $1", "SELECT $$a$$$$b$$, 42"),
        (
            "SELECT $tag$a$tag$$other$b$other$, $1",
            "SELECT $tag$a$tag$$other$b$other$, 42",
        ),
        ("SELECT $$$$, $1", "SELECT $$$$, 42"),
        (
            r#"SELECT $_body1$[$1] $$ $other$ $_BODY1$ ' \ -- $_body1$, $1"#,
            r#"SELECT $_body1$[$1] $$ $other$ $_BODY1$ ' \ -- $_body1$, 42"#,
        ),
        ("SELECT $文字$[$1]$文字$, $1", "SELECT $文字$[$1]$文字$, 42"),
    ] {
        assert_eq!(
            inject_parameters(sql, &[42.into()], &PostgresQueryBuilder),
            expected
        );
    }
}

#[test]
fn custom_expr_interpolation_inside_identifiers_is_preserved() {
    assert_eq!(
        Query::select()
            .expr(Expr::cust_with_expr("_$$$1", Expr::cust("name")))
            .build(PostgresQueryBuilder),
        ("SELECT _$name".into(), Values(vec![]))
    );
}

#[test]
fn dollar_signs_in_identifiers_are_preserved() {
    assert_eq!(
        inject_parameters(
            "SELECT _$$, $1, 😀$$, $1",
            &[42.into()],
            &PostgresQueryBuilder
        ),
        "SELECT _$$, 42, 😀$$, 42"
    );
}

#[test]
fn parameters_after_single_quoted_backslashes_are_bound() {
    assert_eq!(
        Query::select()
            .expr(Expr::cust_with_values(r"'\' || $1", ["needle"]))
            .build(PostgresQueryBuilder),
        (r"SELECT '\' || $1".into(), Values(vec!["needle".into()]))
    );
}

#[test]
fn escape_string_prefixes_follow_legacy_token_boundaries() {
    for (expression, statement, inlined) in [
        (
            r"_E'a\'b' + $1",
            r"SELECT $1, _E'a\'b' + $2",
            r"SELECT 7, _E'a\'b' + 42",
        ),
        (
            r"a😀E'a\'b' + $1",
            r"SELECT $1, a😀E'a\'b' + $2",
            r"SELECT 7, a😀E'a\'b' + 42",
        ),
        (
            r"_$$E'a\'b' + $1",
            r"SELECT $1, _$E'a\'b' + $2",
            r"SELECT 7, _$E'a\'b' + 42",
        ),
    ] {
        let query = Query::select()
            .expr(Expr::val(7))
            .expr(Expr::cust_with_values(expression, [42]))
            .to_owned();

        assert_eq!(
            query.build(PostgresQueryBuilder),
            (statement.into(), Values(vec![7.into(), 42.into()]))
        );
        assert_eq!(
            inject_parameters(statement, &[7.into(), 42.into()], &PostgresQueryBuilder),
            inlined
        );
    }
}

#[test]
fn custom_expression_writes_text_and_parameters_in_order() {
    #[derive(Debug, PartialEq)]
    enum Event {
        Text(String),
        Parameter(Value),
    }

    #[derive(Default)]
    struct TrackingWriter {
        sql: String,
        events: Vec<Event>,
    }

    impl std::fmt::Display for TrackingWriter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&self.sql)
        }
    }

    impl std::fmt::Write for TrackingWriter {
        fn write_str(&mut self, text: &str) -> std::fmt::Result {
            self.events.push(Event::Text(text.to_owned()));
            self.sql.push_str(text);
            Ok(())
        }
    }

    impl SqlWriter for TrackingWriter {
        fn push_param<T: QueryBuilder>(&mut self, value: Value, _: &T) {
            self.events.push(Event::Parameter(value));
            self.sql.push_str("$1");
        }

        fn as_writer(&mut self) -> &mut dyn std::fmt::Write {
            self
        }
    }

    let expression = Expr::cust_with_values("a + $$1 + $bad + $1 + z", [42]);
    let mut writer = TrackingWriter::default();
    PostgresQueryBuilder.prepare_expr(&expression, &mut writer);
    assert_eq!(
        writer.events,
        [
            Event::Text("a + $".into()),
            Event::Text("1 + ".into()),
            Event::Text(" + ".into()),
            Event::Parameter(42.into()),
            Event::Text(" + z".into()),
        ]
    );

    let expression = Expr::cust_with_values("pre $1 mid $2 tail", [42]);
    let mut writer = TrackingWriter::default();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        PostgresQueryBuilder.prepare_expr(&expression, &mut writer);
    }));
    assert!(result.is_err());
    assert_eq!(
        writer.events,
        [
            Event::Text("pre ".into()),
            Event::Parameter(42.into()),
            Event::Text(" mid ".into()),
        ]
    );
}
