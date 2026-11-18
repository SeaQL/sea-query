pub(crate) mod extension;
pub(crate) mod foreign_key;
pub(crate) mod index;
pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod trigger;
pub(crate) mod types;

use super::*;
use std::ops::ControlFlow;

fn must_escape(b: u8) -> bool {
    const ESCAPE: [bool; 256] = {
        let mut table = [false; 256];
        let mut b = 0;
        while b < 256 {
            table[b] = (b as u8).is_ascii_control() || b == b'\'' as usize || b == b'\\' as usize;
            b += 1;
        }
        table
    };
    ESCAPE[b as usize]
}

#[inline]
fn escape_mask32(chunk: &[u8; 32]) -> u32 {
    use wide::{CmpEq, u8x32};

    let bytes = u8x32::new(*chunk);
    // min(x, 31) == x when x <= 31.
    let ascii_controls =
        bytes.min(u8x32::splat(31)).cmp_eq(bytes) | bytes.cmp_eq(u8x32::splat(127));
    let single_quotes = bytes.cmp_eq(u8x32::splat(b'\''));
    let backslashes = bytes.cmp_eq(u8x32::splat(b'\\'));

    (ascii_controls | single_quotes | backslashes).move_mask() as u32
}

#[inline]
fn escape_mask64(chunk: &[u8; 64]) -> u64 {
    let (chunks, _) = chunk.as_chunks::<32>();
    u64::from(escape_mask32(&chunks[0])) | (u64::from(escape_mask32(&chunks[1])) << 32)
}

fn visit_escapes<T>(
    string: &str,
    mut visit: impl FnMut(usize, u8) -> ControlFlow<T>,
) -> ControlFlow<T> {
    let (chunks, tail) = string.as_bytes().as_chunks::<64>();
    for (chunk_i, chunk) in chunks.iter().enumerate() {
        let mut mask = escape_mask64(chunk);
        while mask != 0 {
            let byte_i = mask.trailing_zeros() as usize;
            visit(chunk_i * 64 + byte_i, chunk[byte_i])?;
            mask &= mask - 1;
        }
    }

    let mut tail = tail;
    let mut offset = chunks.len() * 64;

    // Remaining <= 63 bytes
    // Benchmarks show that this approach is faster than using loops of 64 bytes + padding.
    if let Some((chunk, rest)) = tail.split_first_chunk::<32>() {
        let mut mask = escape_mask32(chunk);
        while mask != 0 {
            let byte_i = mask.trailing_zeros() as usize;
            visit(offset + byte_i, chunk[byte_i])?;
            mask &= mask - 1;
        }
        tail = rest;
        offset += 32;
    }

    for (tail_i, &b) in tail.iter().enumerate() {
        if must_escape(b) {
            visit(offset + tail_i, b)?;
        }
    }

    ControlFlow::Continue(())
}

fn first_escape_position(string: &str) -> Option<usize> {
    visit_escapes(string, {
        #[inline(always)]
        |i, _| ControlFlow::Break(i)
    })
    .break_value()
}

#[inline(always)]
fn write_escape(buffer: &mut impl Write, b: u8) {
    match b {
        b'\x08' => buffer.write_str(r"\b"),
        b'\x0C' => buffer.write_str(r"\f"),
        b'\n' => buffer.write_str(r"\n"),
        b'\r' => buffer.write_str(r"\r"),
        b'\t' => buffer.write_str(r"\t"),
        b'\\' => buffer.write_str(r"\\"),
        b'\'' => buffer.write_str(r"\'"),
        b'\0' => buffer.write_str(r"\0"),
        _ => {
            let octal = [
                b'\\',
                b'0' + (b >> 6),
                b'0' + ((b >> 3) & 7),
                b'0' + (b & 7),
            ];
            buffer.write_str(std::str::from_utf8(&octal).unwrap())
        }
    }
    .unwrap();
}

fn write_escaped_scalar(buffer: &mut impl Write, string: &str) {
    let mut start = 0;
    for (i, b) in string.bytes().enumerate() {
        if !must_escape(b) {
            continue;
        }
        if start != i {
            buffer.write_str(&string[start..i]).unwrap();
        }
        write_escape(buffer, b);
        start = i + 1;
    }
    if start != string.len() {
        buffer.write_str(&string[start..]).unwrap();
    }
}

#[inline(never)] // inline will be slower because code generation issue
fn write_escaped_simd(buffer: &mut impl Write, string: &str) {
    // Every escaped byte is ASCII, so slicing around it stays on char boundaries.
    let mut start = 0;

    let _ = visit_escapes(string, {
        #[inline(always)]
        |i, b| {
            if start != i {
                buffer.write_str(&string[start..i]).unwrap();
            }
            write_escape(buffer, b);
            start = i + 1;
            ControlFlow::<()>::Continue(())
        }
    });

    if start != string.len() {
        buffer.write_str(&string[start..]).unwrap();
    }
}

/// Postgres query builder.
#[derive(Default, Debug)]
pub struct PostgresQueryBuilder;

const QUOTE: Quote = Quote(b'"', b'"');

impl GenericBuilder for PostgresQueryBuilder {}

impl SchemaBuilder for PostgresQueryBuilder {}

impl QuotedBuilder for PostgresQueryBuilder {
    fn quote(&self) -> Quote {
        QUOTE
    }
}

// https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-BACKSLASH-TABLE
impl EscapeBuilder for PostgresQueryBuilder {
    fn needs_escape(&self, s: &str) -> bool {
        first_escape_position(s).is_some()
    }

    fn write_escaped(&self, buffer: &mut impl Write, string: &str) {
        if string.len() < 16 {
            write_escaped_scalar(buffer, string);
        } else {
            write_escaped_simd(buffer, string);
        }
    }

    fn unescape_string(&self, string: &str) -> String {
        let mut chars = string.chars().peekable();
        let mut result = String::with_capacity(string.len());

        while let Some(c) = chars.next() {
            if c != '\\' {
                result.push(c);
                continue;
            }

            let Some(next) = chars.next() else {
                result.push('\\');
                continue;
            };

            match next {
                'b' => result.push('\x08'),
                'f' => result.push('\x0C'),
                'n' => result.push('\n'),
                'r' => result.push('\r'),
                't' => result.push('\t'),
                '0' => result.push('\0'),
                '\'' => result.push('\''),
                '\\' => result.push('\\'),
                'u' => {
                    let mut hex = String::new();
                    for _ in 0..4 {
                        if let Some(h) = chars.next() {
                            hex.push(h);
                        }
                    }
                    if let Ok(code) = u32::from_str_radix(&hex, 16) {
                        if let Some(ch) = std::char::from_u32(code) {
                            result.push(ch);
                        }
                    }
                }
                'U' => {
                    let mut hex = String::new();
                    for _ in 0..8 {
                        if let Some(h) = chars.next() {
                            hex.push(h);
                        }
                    }
                    if let Ok(code) = u32::from_str_radix(&hex, 16) {
                        if let Some(ch) = std::char::from_u32(code) {
                            result.push(ch);
                        }
                    }
                }
                c @ '0'..='7' => {
                    let mut oct = String::new();
                    oct.push(c);
                    for _ in 0..2 {
                        if let Some(next_o) = chars.peek() {
                            if ('0'..='7').contains(next_o) {
                                oct.push(chars.next().unwrap());
                            }
                        }
                    }
                    if let Ok(val) = u8::from_str_radix(&oct, 8) {
                        result.push(val as char);
                    }
                }
                other => {
                    result.push('\\');
                    result.push(other);
                }
            }
        }

        result
    }
}

impl TableRefBuilder for PostgresQueryBuilder {}

#[cfg(test)]
mod tests {
    use crate::{EscapeBuilder, PostgresQueryBuilder, QueryBuilder};

    #[test]
    fn test_write_escaped() {
        let escaper = PostgresQueryBuilder;

        let control_chars: String = (0u8..=31).map(|b| b as char).collect();

        let escaped = escaper.escape_string(&control_chars);

        assert!(escaped.contains(r"\b")); // 0x08
        assert!(escaped.contains(r"\f")); // 0x0C
        assert!(escaped.contains(r"\n")); // 0x0A
        assert!(escaped.contains(r"\r")); // 0x0D
        assert!(escaped.contains(r"\t")); // 0x09
        assert!(escaped.contains(r"\0")); // 0x00

        for b in 0u8..=31 {
            let c = b as char;
            if !matches!(c, '\x00' | '\x08' | '\x09' | '\x0A' | '\x0C' | '\x0D') {
                let octal = format!("\\{b:03o}");
                assert!(escaped.contains(&octal));
            }
        }
    }

    #[test]
    fn test_write_escaped_keeps_other_chars() {
        let escaper = PostgresQueryBuilder;

        assert_eq!(escaper.escape_string(""), "");
        assert_eq!(escaper.escape_string("plain"), "plain");
        assert_eq!(escaper.escape_string("'"), r"\'");
        assert_eq!(
            escaper.escape_string("'你'😀\\é\n\x7F"),
            r"\'你\'😀\\é\n\177"
        );
        assert!(!escaper.needs_escape("你😀é \"plain\""));
        assert!(escaper.needs_escape("é\x7F"));
    }

    #[test]
    fn string_literals_escape_ascii_at_utf8_boundaries() {
        let escapes = [
            r"\0", r"\001", r"\002", r"\003", r"\004", r"\005", r"\006", r"\007", r"\b", r"\t",
            r"\n", r"\013", r"\f", r"\r", r"\016", r"\017", r"\020", r"\021", r"\022", r"\023",
            r"\024", r"\025", r"\026", r"\027", r"\030", r"\031", r"\032", r"\033", r"\034",
            r"\035", r"\036", r"\037",
        ];
        let escaper = PostgresQueryBuilder;
        for padding in 0..=128 {
            let prefix = format!("{}你😀é", "x".repeat(padding));
            let mut quoted = String::new();
            escaper.write_string_quoted(&prefix, &mut quoted);
            assert_eq!(quoted, format!("'{prefix}'"));
            assert!(!escaper.needs_escape(&prefix));

            for (byte, escaped) in
                (0..32)
                    .zip(escapes)
                    .chain([(127, r"\177"), (b'\'', r"\'"), (b'\\', r"\\")])
            {
                let byte = char::from(byte);
                for (string, expected) in [
                    (format!("{prefix}{byte}"), format!("{prefix}{escaped}")),
                    (
                        format!("{prefix}{byte}\\'尾😀abcdefghijklmnop{byte}\\'é字"),
                        format!(r"{prefix}{escaped}\\\'尾😀abcdefghijklmnop{escaped}\\\'é字"),
                    ),
                ] {
                    quoted.clear();
                    escaper.write_string_quoted(&string, &mut quoted);
                    assert_eq!(quoted, format!("E'{expected}'"));
                    assert_eq!(escaper.escape_string(&string), expected);
                    assert!(escaper.needs_escape(&string));
                }
            }
        }
    }

    #[test]
    fn test_unescape_string() {
        let escaper = PostgresQueryBuilder;

        let escaped = r"\b\f\n\r\t\0\'\\\101\102\103\u4F60\U0001F600";
        let unescaped = escaper.unescape_string(escaped);

        let expected = "\x08\x0C\n\r\t\0'\\ABC你😀";

        assert_eq!(unescaped, expected);

        let escaped_expected = escaper.escape_string(expected);

        // We don't convert ASCII chars back to octal in escaping
        assert_eq!(r"\b\f\n\r\t\0\'\\ABC你😀", escaped_expected);
    }
}
