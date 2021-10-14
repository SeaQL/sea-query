pub struct UpSert<T, A> {
    on_conflict: T,
    do_action: A,
}

impl<T, A> UpSert<T, A> {
    fn do_conflict_nothing() -> Self {
        todo!()
    }

    fn do_conflict<Target>(target: Target) -> Self {
        todo!()
    }

    fn do_conflict_on_constraint<Target>(target: Target) -> Self {
        todo!()
    }

    fn do_nothing(self) -> Self {
        todo!()
    }

    fn do_action(self) -> Self {
        todo!()
    }
}
