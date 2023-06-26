use super::*;
use crate::extension::postgres::*;

impl QueryBuilder for PostgresQueryBuilder {
    fn placeholder(&self) -> (&str, bool) {
        ("$", true)
    }

    fn prepare_simple_expr(&self, simple_expr: &SimpleExpr, sql: &mut dyn SqlWriter) {
        match simple_expr {
            SimpleExpr::AsEnum(type_name, expr) => {
                let simple_expr = expr.clone().cast_as(SeaRc::clone(type_name));
                self.prepare_simple_expr_common(&simple_expr, sql);
            }
            _ => QueryBuilder::prepare_simple_expr_common(self, simple_expr, sql),
        }
    }

    fn prepare_select_distinct(&self, select_distinct: &SelectDistinct, sql: &mut dyn SqlWriter) {
        match select_distinct {
            SelectDistinct::All => write!(sql, "ALL").unwrap(),
            SelectDistinct::Distinct => write!(sql, "DISTINCT").unwrap(),
            SelectDistinct::DistinctOn(cols) => {
                write!(sql, "DISTINCT ON (").unwrap();
                cols.iter().fold(true, |first, column_ref| {
                    if !first {
                        write!(sql, ", ").unwrap();
                    }
                    self.prepare_column_ref(column_ref, sql);
                    false
                });
                write!(sql, ")").unwrap();
            }
            _ => {}
        };
    }

    fn prepare_bin_oper(&self, bin_oper: &BinOper, sql: &mut dyn SqlWriter) {
        match bin_oper {
            BinOper::PgOperator(oper) => write!(
                sql,
                "{}",
                match oper {
                    PgBinOper::ILike => "ILIKE",
                    PgBinOper::NotILike => "NOT ILIKE",
                    PgBinOper::Matches => "@@",
                    PgBinOper::Contains => "@>",
                    PgBinOper::Contained => "<@",
                    PgBinOper::Concatenate => "||",
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
                }
            )
            .unwrap(),
            _ => self.prepare_bin_oper_common(bin_oper, sql),
        }
    }

    fn prepare_query_statement(&self, query: &SubQueryStatement, sql: &mut dyn SqlWriter) {
        query.prepare_statement(self, sql);
    }

    fn prepare_function(&self, function: &Function, sql: &mut dyn SqlWriter) {
        match function {
            Function::PgFunction(function) => write!(
                sql,
                "{}",
                match function {
                    PgFunction::ToTsquery => "TO_TSQUERY",
                    PgFunction::ToTsvector => "TO_TSVECTOR",
                    PgFunction::PhrasetoTsquery => "PHRASETO_TSQUERY",
                    PgFunction::PlaintoTsquery => "PLAINTO_TSQUERY",
                    PgFunction::WebsearchToTsquery => "WEBSEARCH_TO_TSQUERY",
                    PgFunction::TsRank => "TS_RANK",
                    PgFunction::TsRankCd => "TS_RANK_CD",
                    PgFunction::StartsWith => "STARTS_WITH",
                    PgFunction::GenRandomUUID => "GEN_RANDOM_UUID",
                    #[cfg(feature = "postgres-array")]
                    PgFunction::Any => "ANY",
                    #[cfg(feature = "postgres-array")]
                    PgFunction::Some => "SOME",
                    #[cfg(feature = "postgres-array")]
                    PgFunction::All => "ALL",
                }
            )
            .unwrap(),
            _ => self.prepare_function_common(function, sql),
        }
    }

    fn prepare_order_expr(&self, order_expr: &OrderExpr, sql: &mut dyn SqlWriter) {
        if !matches!(order_expr.order, Order::Field(_)) {
            self.prepare_simple_expr(&order_expr.expr, sql);
        }
        self.prepare_order(order_expr, sql);
        match order_expr.nulls {
            None => (),
            Some(NullOrdering::Last) => write!(sql, " NULLS LAST").unwrap(),
            Some(NullOrdering::First) => write!(sql, " NULLS FIRST").unwrap(),
        }
    }

    fn prepare_value(&self, value: &Value, sql: &mut dyn SqlWriter) {
        sql.push_param(value.clone(), self as _);
    }

    fn write_string_quoted(&self, string: &str, buffer: &mut String) {
        let escaped = self.escape_string(string);
        let string = if escaped.find('\\').is_some() {
            "E'".to_owned() + &escaped + "'"
        } else {
            "'".to_owned() + &escaped + "'"
        };
        write!(buffer, "{string}").unwrap()
    }

    fn if_null_function(&self) -> &str {
        "COALESCE"
    }
}
