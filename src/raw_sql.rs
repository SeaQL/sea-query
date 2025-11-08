pub mod seaql;

use crate::{QueryBuilder, write_int};

#[derive(Debug)]
pub struct RawSqlQueryBuilder {
    sql: String,
    parameter_index: usize,
    placeholder: &'static str,
    numbered: bool,
}

impl RawSqlQueryBuilder {
    pub fn new<T: QueryBuilder>(backend: T) -> Self {
        let (placeholder, numbered) = backend.placeholder();
        Self {
            sql: Default::default(),
            parameter_index: 1,
            placeholder,
            numbered,
        }
    }

    pub fn push_fragment(&mut self, sql: &str) -> &mut Self {
        self.sql.push_str(sql);
        self
    }

    pub fn push_parameters(&mut self, n: usize) -> &mut Self {
        for i in 0..n {
            if i > 0 {
                self.sql.push_str(", ");
            }
            self.sql.push_str(self.placeholder);
            if self.numbered {
                write_int(&mut self.sql, self.parameter_index);
                self.parameter_index += 1;
            }
        }
        self
    }

    pub fn push_tuple_parameter_groups(&mut self, len: usize, tuple_arity: usize) -> &mut Self {
        for i in 0..len {
            if i > 0 {
                self.sql.push_str(", ");
            }
            self.sql.push('(');
            self.push_parameters(tuple_arity);
            self.sql.push(')');
        }
        self
    }

    pub fn finish(self) -> String {
        self.sql
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::PostgresQueryBuilder;

    #[derive(Default)]
    struct Values(Vec<String>);

    impl Values {
        fn bind<V: std::fmt::Debug>(&mut self, v: V) {
            self.0.push(format!("{v:?}"));
        }
    }

    #[test]
    fn test_raw_sql_0() {
        let mut builder = RawSqlQueryBuilder::new(PostgresQueryBuilder);
        builder.push_fragment("SELECT");
        assert_eq!(builder.finish(), "SELECT");
    }

    #[test]
    fn test_raw_sql_1() {
        let a = 1;
        let b = [2i32, 3];
        let c = [4i32, 5, 6];

        let mut builder = RawSqlQueryBuilder::new(PostgresQueryBuilder);
        builder
            .push_fragment("SELECT")
            .push_fragment(" ")
            .push_parameters(1)
            .push_fragment(", ")
            .push_parameters(b.len())
            .push_fragment(", ")
            .push_parameters(c.len());

        assert_eq!(builder.finish(), "SELECT $1, $2, $3, $4, $5, $6");

        let mut values = Values::default();
        values.bind(a);
        for v in b.iter() {
            values.bind(v);
        }
        for v in c.iter() {
            values.bind(v);
        }

        assert_eq!(values.0, ["1", "2", "3", "4", "5", "6"]);
    }
}
