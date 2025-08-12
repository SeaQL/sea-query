use crate::{Value, Values};

#[derive(Debug)]
pub struct Query {
    pub sql: String,
    pub values: Values,
}

pub fn query(sql: &str) -> Query {
    Query {
        sql: sql.to_owned(),
        values: Values(Default::default()),
    }
}

impl Query {
    pub fn bind<V: Into<Value> + Clone>(mut self, v: &V) -> Self {
        self.values.0.push(v.to_owned().into());
        self
    }

    /// Matches the signature of [`SqlWriterValues::into_parts`]
    pub fn into_parts(self) -> (String, Values) {
        (self.sql, self.values)
    }
}

pub struct DebugQuery {
    pub sql: String,
    pub params: Vec<String>,
}

pub fn debug(sql: &str) -> DebugQuery {
    DebugQuery {
        sql: sql.to_owned(),
        params: Default::default(),
    }
}

impl DebugQuery {
    /// This can bind virtually any type for debug purpose
    pub fn bind<V: std::fmt::Debug>(mut self, v: V) -> Self {
        self.params.push(format!("{v:?}"));
        self
    }
}

impl std::fmt::Debug for DebugQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sql!(")?;
        write!(f, "{}", self.sql)?;
        writeln!(f, ")")?;
        write!(f, "    .params(")?;
        for (i, p) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{p}")?;
        }
        write!(f, ")")
    }
}
