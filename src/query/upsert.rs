use crate::Value;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UpsertExpr<Target,Action> {
    target: Target,
    action: Action,
}

impl<Target,Action> UpsertExpr<Target,Action> {
    pub fn prepare_upsert(&self, sql: &mut dyn fmt::Write, collector: &mut dyn FnMut(Value)) {
        write!(sql, " ON CONFLICT").unwrap();
        self.prepare_target(sql,collector);
        self.prepare_action(sql,collector);
    }

    fn prepare_target(&self, s: &mut dyn fmt::Write, collector: &mut dyn FnMut(Value)){

    }

    fn prepare_action(&self, s: &mut dyn fmt::Write, collector: &mut dyn FnMut(Value)){

    }
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

    pub fn do_update_set(self,sets:Set) -> UpsertExpr {
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

