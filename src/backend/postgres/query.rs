use super::*;
use crate::extension::postgres::*;

impl QueryBuilder for PostgresQueryBuilder {
    fn placeholder(&self) -> (&str, bool) {
        ("$", true)
    }

    fn prepare_returning(
        &self,
        returning: &Vec<SelectExpr>,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
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

    fn if_null_function(&self) -> &str {
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

    fn prepare_function(
        &self,
        function: &Function,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        match function {
            Function::PgFunction(function) => {
                write!(
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
                .unwrap()
            }
            _ => self.prepare_function_common(function, sql, collector),
        }
    }
}
