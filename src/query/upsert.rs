use crate::Value;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UpsertExpr {
    // on_conflict: T,
// do_action: A,
}

impl UpsertExpr {
    pub fn prepare_upsert(&self, s: &mut dyn fmt::Write, collector: &mut dyn FnMut(Value)) {}
}

pub struct Upsert<T, A> {
    on_conflict: T,
    do_action: A,
}

impl<T, A> Upsert<T, A> {
    pub fn do_conflict_nothing() -> UpsertExpr {
        UpsertExpr {}
    }

    pub fn do_conflict<Target>(target: Target) -> Self {
        todo!()
    }

    pub fn do_conflict_on_constraint<Target>(target: Target) -> Self {
        todo!()
    }

    pub fn do_nothing(&mut self) -> UpsertExpr {
        todo!()
    }

    pub fn do_action(self) -> UpsertExpr {
        todo!()
    }
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

