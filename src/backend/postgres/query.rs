use super::*;
use crate::extension::postgres::*;

impl OperLeftAssocDecider for PostgresQueryBuilder {
    fn well_known_left_associative(&self, op: &BinOper) -> bool {
        let common_answer = common_well_known_left_associative(op);
        let pg_specific_answer = matches!(op, BinOper::PgOperator(PgBinOper::Concatenate));
        common_answer || pg_specific_answer
    }
}

impl PrecedenceDecider for PostgresQueryBuilder {
    fn inner_expr_well_known_greater_precedence(&self, inner: &Expr, outer_oper: &Oper) -> bool {
        let common_answer = common_inner_expr_well_known_greater_precedence(inner, outer_oper);
        let pg_specific_answer = match inner {
            Expr::Binary(_, inner_bin_oper, _) => {
                let inner_oper: Oper = (*inner_bin_oper).into();
                if inner_oper.is_arithmetic() || inner_oper.is_shift() {
                    is_ilike(inner_bin_oper)
                } else if is_pg_comparison(inner_bin_oper) {
                    outer_oper.is_logical()
                } else {
                    false
                }
            }
            _ => false,
        };
        common_answer || pg_specific_answer
    }
}

impl QueryBuilder for PostgresQueryBuilder {
    fn placeholder(&self) -> (&'static str, bool) {
        ("$", true)
    }

    fn prepare_expr(&self, simple_expr: &Expr, sql: &mut impl SqlWriter) {
        match simple_expr {
            Expr::AsEnum(type_name, expr) => {
                sql.write_str("CAST(").unwrap();
                self.prepare_expr_common(expr, sql);
                let q = self.quote();
                let type_name = &type_name.0;
                let (ty, sfx) = if let Some(base) = type_name.strip_suffix("[]") {
                    (base, "[]")
                } else {
                    (type_name.as_ref(), "")
                };
                sql.write_str(" AS ").unwrap();
                sql.write_char(q.left()).unwrap();
                sql.write_str(ty).unwrap();
                sql.write_char(q.right()).unwrap();
                sql.write_str(sfx).unwrap();
                sql.write_char(')').unwrap();
            }
            _ => QueryBuilder::prepare_expr_common(self, simple_expr, sql),
        }
    }

    fn prepare_select_distinct(&self, select_distinct: &SelectDistinct, sql: &mut impl SqlWriter) {
        match select_distinct {
            SelectDistinct::All => sql.write_str("ALL").unwrap(),
            SelectDistinct::Distinct => sql.write_str("DISTINCT").unwrap(),
            SelectDistinct::DistinctOn(cols) => {
                sql.write_str("DISTINCT ON (").unwrap();

                let mut cols = cols.iter();
                join_io!(
                    cols,
                    col,
                    join {
                        sql.write_str(", ").unwrap();
                    },
                    do {
                        self.prepare_column_ref(col, sql);
                    }
                );

                sql.write_str(")").unwrap();
            }
            _ => {}
        };
    }

    fn prepare_bin_oper(&self, bin_oper: &BinOper, sql: &mut impl SqlWriter) {
        match bin_oper {
            BinOper::PgOperator(oper) => sql
                .write_str(match oper {
                    PgBinOper::ILike => "ILIKE",
                    PgBinOper::NotILike => "NOT ILIKE",
                    PgBinOper::Matches => "@@",
                    PgBinOper::Contains => "@>",
                    PgBinOper::Contained => "<@",
                    PgBinOper::Concatenate => "||",
                    PgBinOper::Overlap => "&&",
                    PgBinOper::Similarity => "%",
                    PgBinOper::WordSimilarity => "<%",
                    PgBinOper::StrictWordSimilarity => "<<%",
                    PgBinOper::SimilarityDistance => "<->",
                    PgBinOper::WordSimilarityDistance => "<<->",
                    PgBinOper::StrictWordSimilarityDistance => "<<<->",
                    PgBinOper::GetJsonField => "->",
                    PgBinOper::CastJsonField => "->>",
                    PgBinOper::Regex => "~",
                    PgBinOper::RegexCaseInsensitive => "~*",
                    #[cfg(feature = "postgres-vector")]
                    PgBinOper::EuclideanDistance => "<->",
                    #[cfg(feature = "postgres-vector")]
                    PgBinOper::NegativeInnerProduct => "<#>",
                    #[cfg(feature = "postgres-vector")]
                    PgBinOper::CosineDistance => "<=>",
                })
                .unwrap(),
            _ => self.prepare_bin_oper_common(bin_oper, sql),
        }
    }

    fn prepare_query_statement(&self, query: &SubQueryStatement, sql: &mut impl SqlWriter) {
        query.prepare_statement(self, sql);
    }

    fn prepare_explain_statement(&self, explain: &ExplainStatement, sql: &mut impl SqlWriter) {
        fn write_sep(sql: &mut impl SqlWriter, first: &mut bool) {
            if !*first {
                sql.write_str(", ").unwrap();
            } else {
                *first = false;
            }
        }

        // https://www.postgresql.org/docs/current/sql-explain.html
        // Specifies whether the selected option should be turned on or off. You can write TRUE, ON, or 1 to enable the option, and FALSE, OFF, or 0 to disable it. The boolean value can also be omitted, in which case TRUE is assumed.
        fn write_false(sql: &mut impl SqlWriter, value: bool) {
            if !value {
                sql.write_str(" 0").unwrap();
            }
        }

        sql.write_str("EXPLAIN").unwrap();

        let has_options = explain.analyze.is_some()
            || explain.pg_opts.verbose.is_some()
            || explain.pg_opts.costs.is_some()
            || explain.pg_opts.settings.is_some()
            || explain.pg_opts.generic_plan.is_some()
            || explain.pg_opts.buffers.is_some()
            || explain.pg_opts.serialize.is_some()
            || explain.pg_opts.wal.is_some()
            || explain.pg_opts.timing.is_some()
            || explain.pg_opts.summary.is_some()
            || explain.pg_opts.memory.is_some()
            || explain.format.is_some();

        if has_options {
            sql.write_str(" (").unwrap();
            let mut first = true;

            if let Some(analyze) = explain.analyze {
                write_sep(sql, &mut first);
                sql.write_str("ANALYZE").unwrap();
                write_false(sql, analyze);
            }

            if let Some(verbose) = explain.pg_opts.verbose {
                write_sep(sql, &mut first);
                sql.write_str("VERBOSE").unwrap();
                write_false(sql, verbose);
            }

            if let Some(costs) = explain.pg_opts.costs {
                write_sep(sql, &mut first);
                sql.write_str("COSTS").unwrap();
                write_false(sql, costs);
            }

            if let Some(settings) = explain.pg_opts.settings {
                write_sep(sql, &mut first);
                sql.write_str("SETTINGS").unwrap();
                write_false(sql, settings);
            }

            if let Some(generic_plan) = explain.pg_opts.generic_plan {
                write_sep(sql, &mut first);
                sql.write_str("GENERIC_PLAN").unwrap();
                write_false(sql, generic_plan);
            }

            if let Some(buffers) = explain.pg_opts.buffers {
                write_sep(sql, &mut first);
                sql.write_str("BUFFERS").unwrap();
                write_false(sql, buffers);
            }

            if let Some(serialize) = explain.pg_opts.serialize {
                write_sep(sql, &mut first);
                sql.write_str("SERIALIZE ").unwrap();
                sql.write_str(serialize.as_str()).unwrap();
            }

            if let Some(wal) = explain.pg_opts.wal {
                write_sep(sql, &mut first);
                sql.write_str("WAL").unwrap();
                write_false(sql, wal);
            }

            if let Some(timing) = explain.pg_opts.timing {
                write_sep(sql, &mut first);
                sql.write_str("TIMING").unwrap();
                write_false(sql, timing);
            }

            if let Some(summary) = explain.pg_opts.summary {
                write_sep(sql, &mut first);
                sql.write_str("SUMMARY").unwrap();
                write_false(sql, summary);
            }

            if let Some(memory) = explain.pg_opts.memory {
                write_sep(sql, &mut first);
                sql.write_str("MEMORY").unwrap();
                write_false(sql, memory);
            }

            if let Some(format) = explain.format {
                write_sep(sql, &mut first);
                sql.write_str("FORMAT ").unwrap();
                sql.write_str(format.as_str()).unwrap();
            }
            sql.write_str(")").unwrap();
        }

        if let Some(statement) = &explain.statement {
            sql.write_str(" ").unwrap();
            statement.write_to(self, sql);
        }
    }

    fn prepare_function_name(&self, function: &Func, sql: &mut impl SqlWriter) {
        match function {
            Func::PgFunction(function) => sql
                .write_str(match function {
                    PgFunc::ToTsquery => "TO_TSQUERY",
                    PgFunc::ToTsvector => "TO_TSVECTOR",
                    PgFunc::PhrasetoTsquery => "PHRASETO_TSQUERY",
                    PgFunc::PlaintoTsquery => "PLAINTO_TSQUERY",
                    PgFunc::WebsearchToTsquery => "WEBSEARCH_TO_TSQUERY",
                    PgFunc::TsRank => "TS_RANK",
                    PgFunc::TsRankCd => "TS_RANK_CD",
                    PgFunc::StartsWith => "STARTS_WITH",
                    PgFunc::GenRandomUUID => "GEN_RANDOM_UUID",
                    PgFunc::JsonBuildObject => "JSON_BUILD_OBJECT",
                    PgFunc::JsonAgg => "JSON_AGG",
                    PgFunc::ArrayAgg => "ARRAY_AGG",
                    PgFunc::DateTrunc => "DATE_TRUNC",
                    PgFunc::Any => "ANY",
                    PgFunc::Some => "SOME",
                    PgFunc::All => "ALL",
                })
                .unwrap(),
            _ => self.prepare_function_name_common(function, sql),
        }
    }

    fn prepare_table_sample(&self, select: &SelectStatement, sql: &mut impl SqlWriter) {
        let Some(table_sample) = select.table_sample else {
            return;
        };

        match table_sample.method {
            SampleMethod::BERNOULLI => sql.write_str(" TABLESAMPLE BERNOULLI").unwrap(),
            SampleMethod::SYSTEM => sql.write_str(" TABLESAMPLE SYSTEM").unwrap(),
        }
        sql.write_str(" (").unwrap();
        write!(sql, "{}", table_sample.percentage).unwrap();
        sql.write_char(')').unwrap();
        if let Some(repeatable) = table_sample.repeatable {
            sql.write_str(" REPEATABLE (").unwrap();
            write!(sql, "{repeatable}").unwrap();
            sql.write_char(')').unwrap();
        }
    }

    fn prepare_order_expr(&self, order_expr: &OrderExpr, sql: &mut impl SqlWriter) {
        if !matches!(order_expr.order, Order::Field(_)) {
            self.prepare_expr(&order_expr.expr, sql);
        }
        self.prepare_order(order_expr, sql);
        match order_expr.nulls {
            None => (),
            Some(NullOrdering::Last) => sql.write_str(" NULLS LAST").unwrap(),
            Some(NullOrdering::First) => sql.write_str(" NULLS FIRST").unwrap(),
        }
    }

    fn prepare_value(&self, value: Value, sql: &mut impl SqlWriter) {
        sql.push_param(value, self as _);
    }

    fn write_string_quoted(&self, string: &str, buffer: &mut impl Write) {
        if self.needs_escape(string) {
            buffer.write_str("E'").unwrap();
        } else {
            buffer.write_str("'").unwrap();
        }
        self.write_escaped(buffer, string);
        buffer.write_str("'").unwrap();
    }

    fn write_bytes(&self, bytes: &[u8], buffer: &mut impl Write) {
        buffer.write_str("'\\x").unwrap();
        for b in bytes {
            write!(buffer, "{b:02X}").unwrap();
        }
        buffer.write_str("'").unwrap();
    }

    fn if_null_function(&self) -> &str {
        "COALESCE"
    }
}

fn is_pg_comparison(b: &BinOper) -> bool {
    matches!(
        b,
        BinOper::PgOperator(PgBinOper::Contained)
            | BinOper::PgOperator(PgBinOper::Contains)
            | BinOper::PgOperator(PgBinOper::Similarity)
            | BinOper::PgOperator(PgBinOper::WordSimilarity)
            | BinOper::PgOperator(PgBinOper::StrictWordSimilarity)
            | BinOper::PgOperator(PgBinOper::Matches)
    )
}

fn is_ilike(b: &BinOper) -> bool {
    matches!(
        b,
        BinOper::PgOperator(PgBinOper::ILike) | BinOper::PgOperator(PgBinOper::NotILike)
    )
}
