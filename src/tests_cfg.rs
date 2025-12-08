//! Configurations for test cases and examples. Not intended for actual use.

#[cfg(feature = "with-json")]
pub use serde_json::json;

use crate::IdenStatic;

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
    Ascii,
    CreatedAt,
    UserData,
}

/// A shorthand for [`Character`]
pub type Char = Character;

impl IdenStatic for Character {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Table => "character",
            Self::Id => "id",
            Self::Character => "character",
            Self::FontSize => "font_size",
            Self::SizeW => "size_w",
            Self::SizeH => "size_h",
            Self::FontId => "font_id",
            Self::Ascii => "ascii",
            Self::CreatedAt => "created_at",
            Self::UserData => "user_data",
        }
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

impl IdenStatic for Font {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Table => "font",
            Self::Id => "id",
            Self::Name => "name",
            Self::Variant => "variant",
            Self::Language => "language",
        }
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
    Tokens,
}

impl IdenStatic for Glyph {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Table => "glyph",
            Self::Id => "id",
            Self::Image => "image",
            Self::Aspect => "aspect",
            Self::Tokens => "tokens",
        }
    }
}

/// Representation of a database table named `Task`.
///
/// A `Enum` implemented [`Iden`] used in rustdoc and test to demonstrate the library usage.
///
/// [`Iden`]: crate::types::Iden
#[derive(Debug)]
pub enum Task {
    Table,
    Id,
    IsDone,
}

impl IdenStatic for Task {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Table => "task",
            Self::Id => "id",
            Self::IsDone => "is_done",
        }
    }
}
