// TODO: replace join_io with this function
// Make write a sperater a bit faster
#[cfg(feature = "postgres-array")]
pub(crate) fn join_write<T, B: std::fmt::Write>(
    buf: &mut B,
    items: impl IntoIterator<Item = T>,
    mut join: impl FnMut(&mut B) -> std::fmt::Result,
    mut r#do: impl FnMut(&mut B, T) -> std::fmt::Result,
) -> std::fmt::Result {
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
