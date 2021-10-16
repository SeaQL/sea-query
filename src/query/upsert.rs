use crate::{DynIden, IntoIden, SimpleExpr, Value};
use std::fmt;

#[derive(Debug, Clone)]
pub enum ConflictExpr {
    None,
    Sql(String),
    Constraint {
        key: String,
        filter: Vec<SimpleExpr>,
    },
    Column {
        conflict: Vec<DynIden>,
        filter: Vec<SimpleExpr>,
    },
}

#[derive(Debug, Clone)]
pub enum ActionExpr {
    None,
    Set {
        column: Vec<SimpleExpr>,
        excluded: Vec<DynIden>,
    },
}

#[derive(Debug, Clone)]
pub struct UpsertExpr {
    pub conflict: ConflictExpr,
    pub action: ActionExpr,
}

impl Default for UpsertExpr {
    fn default() -> Self {
        Self::do_conflict_nothing()
    }
}

impl UpsertExpr {
    pub fn do_conflict_nothing() -> Self {
        Self { conflict: ConflictExpr::None, action: ActionExpr::None }
    }

    pub fn do_conflict<C, I, F>(conflict: I, filter: F) -> Self
        where
            C: IntoIden,
            I: IntoIterator<Item=C>,
            F: IntoIterator<Item=SimpleExpr>,
    {
        Self {
            conflict: ConflictExpr::Column {
                conflict: conflict.into_iter().map(|c| c.into_iden()).collect(),
                filter: filter.into_iter().collect(),
            },
            action: ActionExpr::None,
        }
    }

    pub fn do_conflict_sql<S:>(sql: S) -> Self
        where S: Into<String>
    {
        Self {
            conflict: ConflictExpr::Sql(sql.into()),
            action: ActionExpr::None,
        }
    }

    pub fn do_conflict_on_constraint<S, F>(key: S, filter: F) -> Self
        where
            S: Into<String>,
            F: IntoIterator<Item=SimpleExpr>,
    {
        Self {
            conflict: ConflictExpr::Constraint {
                key: key.into(),
                filter: filter.into_iter().collect(),
            },
            action: ActionExpr::None,
        }
    }

    pub fn do_nothing(&mut self) -> &mut Self {
        self.action = ActionExpr::None;
        self
    }

    // pub fn do_update_set(&mut self, column: Set) -> &mut self {
    //
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::{tests_cfg::*, *};

    #[test]
    fn test_do_conflict_nothing() {
        let query = Query::insert()
            .into_table(Glyph::Table)
            .columns(vec![Glyph::Image])
            .upsert(UpsertExpr::do_conflict_nothing())
            .returning(Query::select().column(Glyph::Id).take())
            .to_owned().to_string(PostgresQueryBuilder);

        println!("{}", query);
    }

    #[test]
    fn test_on_conflict() {
        let query = Query::insert()
            .into_table(Glyph::Table)
            .columns(vec![Glyph::Image])
            .upsert(UpsertExpr::do_conflict(
                vec![Glyph::Image, Glyph::Id],
                vec![Expr::col(Glyph::Image).eq(5)],
            ))
            .returning(Query::select().column(Glyph::Id).take())
            .to_owned().to_string(PostgresQueryBuilder);
        println!("{}", query);
    }

    #[test]
    fn test_on_conflict_sql() {
        let query = Query::insert()
            .into_table(Glyph::Table)
            .columns(vec![Glyph::Image])
            .upsert(UpsertExpr::do_conflict_sql("(id) WHERE id > 0"))
            .returning(Query::select().column(Glyph::Id).take())
            .to_owned().to_string(PostgresQueryBuilder);
        println!("{}", query);
    }

    #[test]
    fn test_on_conflict_on_constraint() {
        let query = Query::insert()
            .into_table(Glyph::Table)
            .columns(vec![Glyph::Image])
            .upsert(UpsertExpr::do_conflict_on_constraint(
                "image",
                vec![Expr::col(Glyph::Image).eq(5)],
            ))
            .returning(Query::select().column(Glyph::Id).take())
            .to_owned().to_string(PostgresQueryBuilder);
        println!("{}", query);
    }
}


