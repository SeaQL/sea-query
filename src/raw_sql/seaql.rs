#[derive(Default)]
pub struct Query {
    sql: String,
    params: Vec<String>,
}

pub fn query(sql: String) -> Query {
    Query {
        sql,
        params: Default::default(),
    }
}

impl Query {
    pub fn bind<V: std::fmt::Debug>(&mut self, v: V) {
        self.params.push(format!("{v:?}"));
    }
}

impl std::fmt::Debug for Query {
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
