use std::fmt;

// Make write a sperater a bit faster
pub(crate) fn join_write<T, B: fmt::Write>(
    buf: &mut B,
    items: impl IntoIterator<Item = T>,
    mut join: impl (FnMut(&mut B) -> fmt::Result),
    mut r#do: impl (FnMut(&mut B, T) -> fmt::Result),
) -> fmt::Result {
    let mut iter = items.into_iter();
    if let Some(first) = iter.next() {
        r#do(buf, first)?;
        for item in iter {
            join(buf)?;
            r#do(buf, item)?;
        }
    }

    Ok(())
}
