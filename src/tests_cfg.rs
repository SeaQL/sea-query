//! Configurations for test cases and examples. Not intended for actual use.

pub use std::fmt::Write as FmtWrite;

#[cfg(feature = "with-json")]
pub use serde_json::json;

use crate::Iden;

/// Representation of a database table named `Character`.
///
/// A `Enum` implemented [`Iden`] used in rustdoc and test to demonstrate the library usage.
///
/// [`Iden`]: crate::types::Iden
#[derive(Debug)]
pub enum Character {
    Table,
    Id,
    Character,
    FontSize,
    SizeW,
    SizeH,
    FontId,
    CreatedAt,
}

/// A shorthand for [`Character`]
pub type Char = Character;

impl Iden for Character {
    fn unquoted(&self, s: &mut dyn FmtWrite) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "character",
                Self::Id => "id",
                Self::Character => "character",
                Self::FontSize => "font_size",
                Self::SizeW => "size_w",
                Self::SizeH => "size_h",
                Self::FontId => "font_id",
                Self::CreatedAt => "created_at",
            }
        )
        .unwrap();
    }
}

/// Representation of a database table named `Font`.
///
/// A `Enum` implemented [`Iden`] used in rustdoc and test to demonstrate the library usage.
///
/// [`Iden`]: crate::types::Iden
#[derive(Debug)]
pub enum Font {
    Table,
    Id,
    Name,
    Variant,
    Language,
}

impl Iden for Font {
    fn unquoted(&self, s: &mut dyn FmtWrite) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "font",
                Self::Id => "id",
                Self::Name => "name",
                Self::Variant => "variant",
                Self::Language => "language",
            }
        )
        .unwrap();
    }
}

/// Representation of a database table named `Glyph`.
///
/// A `Enum` implemented [`Iden`] used in rustdoc and test to demonstrate the library usage.
///
/// [`Iden`]: crate::types::Iden
#[derive(Debug)]
pub enum Glyph {
    Table,
    Id,
    Image,
    Aspect,
}

impl Iden for Glyph {
    fn unquoted(&self, s: &mut dyn FmtWrite) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "glyph",
                Self::Id => "id",
                Self::Image => "image",
                Self::Aspect => "aspect",
            }
        )
        .unwrap();
    }
}
