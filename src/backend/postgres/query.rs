use super::*;
use crate::extension::postgres::*;

impl QueryBuilder<PostgresQueryBuilder> for PostgresQueryBuilder {
    fn placeholder(&self) -> (&str, bool) {
        ("$", true)
    }

    fn prepare_returning<'a>(
        &self,
        returning: &'a [SelectExpr<'a, Self>],
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<PostgresQueryBuilder>),
    ) {
        if !returning.is_empty() {
            write!(sql, " RETURNING ").unwrap();
            returning.iter().fold(true, |first, expr| {
                if !first {
                    write!(sql, ", ").unwrap()
                }
                self.prepare_select_expr(expr, sql, collector);
                false
            });
        }
    }

    fn if_null_function(&self) -> &'static str {
        "COALESCE"
    }

    fn write_string_quoted(&self, string: &str, buffer: &mut String) {
        let escaped = escape_string(string);
        let string = if escaped.find('\\').is_some() {
            "E'".to_owned() + &escaped + "'"
        } else {
            "'".to_owned() + &escaped + "'"
        };
        write!(buffer, "{}", string).unwrap()
    }

    fn prepare_bin_oper<'a>(
        &self,
        bin_oper: &BinOper,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<PostgresQueryBuilder>),
    ) {
        match bin_oper {
            BinOper::Matches => write!(sql, "@@").unwrap(),
            BinOper::Contains => write!(sql, "@>").unwrap(),
            BinOper::Contained => write!(sql, "<@").unwrap(),
            BinOper::Concatenate => write!(sql, "||").unwrap(),
            _ => self.prepare_bin_oper_common(bin_oper, sql, collector),
        }
    }

    fn prepare_function<'a>(
        &self,
        function: &Function,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(&'a dyn QueryValue<PostgresQueryBuilder>),
    ) {
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
                }
            )
            .unwrap(),
            _ => self.prepare_function_common(function, sql, collector),
        }
    }
}
