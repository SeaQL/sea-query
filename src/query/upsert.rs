use crate::{DynIden, IntoIden, SimpleExpr, Value};
use std::fmt;

#[derive(Debug,Clone)]
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

#[derive(Debug,Clone)]
pub enum ActionExpr {
    None,
    Column {
        set: Vec<String>,
        exclude: Vec<String>,
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

    // pub fn do_nothing(&mut self) -> UpsertExpr {
    //     todo!()
    // }

    // pub fn do_update_set(self, sets: Set) -> UpsertExpr {
    //     todo!()
    // }
}

#[ignore]
#[test]
fn test_on_conflict() {
    use crate::{tests_cfg::*, *};

    // let query = Query::insert()
    //     .into_table(Glyph::Table)
    //     .columns(vec![Glyph::Image])
    //     .upsert(Upsert::do_conflict_nothing())
    //     .upsert(Upsert::do_conflict("").do_nothing())
    //     .upsert(Upsert::do_conflict_on_constraint("").do_action())
    //     .upsert(Upsert::do_conflict("").do_action())
    //     .values_panic(vec!["12A".into()])
    //     .returning(Query::select().column(Glyph::Id).take())
    //     .to_owned();
    //
    // assert_eq!(
    //     query.to_string(PostgresQueryBuilder),
    //     r#"INSERT INTO "glyph" ("image") VALUES ('12A') ON CONFLICT ("image") RETURNING "id""#
    // );
}

