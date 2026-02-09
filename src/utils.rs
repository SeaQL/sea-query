use std::fmt;

// A helper to make write separator easier and faster
pub(crate) struct JoinWrite<'a, T, I, B, F1, F2, F3, F4>
where
    I: IntoIterator<Item = T>,
    B: fmt::Write,
    F1: FnMut(&mut B) -> fmt::Result,
    F2: FnMut(&mut B, T) -> fmt::Result,
    F3: FnMut(&mut B) -> fmt::Result,
    F4: FnMut(&mut B) -> fmt::Result,
{
    pub buf: &'a mut B,
    pub items: I,
    pub at_first: F1,
    pub r#do: F2,
    pub join: F3,
    pub at_last: F4,
}

impl<T, I, B, F1, F2, F3, F4> JoinWrite<'_, T, I, B, F1, F2, F3, F4>
where
    I: IntoIterator<Item = T>,
    B: fmt::Write,
    F1: FnMut(&mut B) -> fmt::Result,
    F2: FnMut(&mut B, T) -> fmt::Result,
    F3: FnMut(&mut B) -> fmt::Result,
    F4: FnMut(&mut B) -> fmt::Result,
{
    pub fn exec(mut self) -> fmt::Result {
        let mut iter = self.items.into_iter();
        if let Some(first) = iter.next() {
            (self.at_first)(self.buf)?;
            (self.r#do)(self.buf, first)?;
            for item in iter {
                (self.join)(self.buf)?;
                (self.r#do)(self.buf, item)?;
            }
            (self.at_last)(self.buf)?
        }

        Ok(())
    }
}
