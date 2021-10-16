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
    fn do_conflict_nothing() -> UpsertExpr {
        todo!()
    }

    fn do_conflict<Target>(target: Target) -> UpsertExpr {
        todo!()
    }

    fn do_conflict_on_constraint<Target>(target: Target) -> Self {
        todo!()
    }

    fn do_nothing(&mut self) -> UpsertExpr {
        todo!()
    }

    fn do_action(self) -> UpsertExpr {
        todo!()
    }
}
