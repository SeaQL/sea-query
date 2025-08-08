use crate::QueryBuilder;
use std::fmt::Write;

#[derive(Debug)]
pub struct RawSqlQueryBuilder {
    sql: String,
    parameter_index: usize,
    placeholder: &'static str,
    numbered: bool,
}

pub trait U8Parameter: Sized {
    fn p_len(&self) -> usize {
        1
    }

    fn iter_p(self) -> Option<Self> {
        Some(self)
    }
}

pub trait SingleParameter: Sized {
    fn p_len(&self) -> usize {
        1
    }

    fn iter_p(self) -> Option<Self> {
        Some(self)
    }
}

pub trait ArrayParameter: Sized {
    fn p_len(&self) -> usize;

    fn iter_p(self) -> Self {
        self
    }
}

impl SingleParameter for bool {}
impl SingleParameter for i8 {}
impl SingleParameter for i16 {}
impl SingleParameter for i32 {}
impl SingleParameter for i64 {}
impl SingleParameter for u16 {}
impl SingleParameter for u32 {}
impl SingleParameter for u64 {}
impl SingleParameter for f32 {}
impl SingleParameter for f64 {}
impl SingleParameter for char {}
impl SingleParameter for &str {}
impl SingleParameter for String {}

impl U8Parameter for &u8 {}
impl U8Parameter for &[u8] {}
impl U8Parameter for Vec<u8> {}
impl<const N: usize> U8Parameter for [u8; N] {}

impl<T> ArrayParameter for &[T]
where
    T: SingleParameter,
{
    fn p_len(&self) -> usize {
        self.len()
    }
}

impl<T, const N: usize> ArrayParameter for [T; N]
where
    T: SingleParameter,
{
    fn p_len(&self) -> usize {
        N
    }
}

impl<T> ArrayParameter for Vec<T>
where
    T: SingleParameter,
{
    fn p_len(&self) -> usize {
        self.len()
    }
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
                write!(&mut self.sql, "{}", self.parameter_index).unwrap();
                self.parameter_index += 1;
            }
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
        let b = vec![2i32, 3];
        let c = [4i32, 5, 6];

        let mut builder = RawSqlQueryBuilder::new(PostgresQueryBuilder);
        builder
            .push_fragment("SELECT")
            .push_fragment(" ")
            .push_parameters((&a).p_len())
            .push_fragment(", ")
            .push_parameters((&b).p_len())
            .push_fragment(", ")
            .push_parameters((&c).p_len());

        assert_eq!(builder.finish(), "SELECT $1, $2, $3, $4, $5, $6");

        let mut values = Values::default();
        for v in a.iter_p().into_iter() {
            values.bind(v);
        }
        for v in b.iter_p().into_iter() {
            values.bind(v);
        }
        for v in c.iter_p().into_iter() {
            values.bind(v);
        }

        assert_eq!(values.0, ["1", "2", "3", "4", "5", "6"]);
    }

    #[test]
    fn test_raw_sql_2() {
        let a = 0u8;
        let b = [1u8, 1u8];
        let c = vec![2u8, 2u8];
        let d = &b;

        assert_eq!((&a).p_len(), 1);
        assert_eq!((&b).p_len(), 1);
        assert_eq!((&c).p_len(), 1);
        assert_eq!((&d).p_len(), 1);

        let mut values = Values::default();
        for v in a.iter_p().into_iter() {
            values.bind(v);
        }
        for v in b.iter_p().into_iter() {
            values.bind(v);
        }
        for v in c.iter_p().into_iter() {
            values.bind(v);
        }
        for v in d.iter_p().into_iter() {
            values.bind(v);
        }

        assert_eq!(values.0, ["0", "[1, 1]", "[2, 2]", "[1, 1]"]);
    }

    #[test]
    fn test_raw_sql_3() {
        let a = 'a';
        let b = "bb";
        let c = "ccc".to_string();

        assert_eq!((&a).p_len(), 1);
        assert_eq!((&b).p_len(), 1);
        assert_eq!((&c).p_len(), 1);

        let mut values = Values::default();
        for v in a.iter_p().into_iter() {
            values.bind(v);
        }
        for v in b.iter_p().into_iter() {
            values.bind(v);
        }
        for v in c.iter_p().into_iter() {
            values.bind(v);
        }

        assert_eq!(values.0, ["'a'", "\"bb\"", "\"ccc\""]);
    }
}
