//! Configurations for test cases and examples. Not intended for actual use.

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
    Ascii,
    CreatedAt,
    UserData,
}

/// A shorthand for [`Character`]
pub type Char = Character;

impl From<Character> for Iden {
    fn from(value: Character) -> Self {
        let str = match value {
            Character::Table => "character",
            Character::Id => "id",
            Character::Character => "character",
            Character::FontSize => "font_size",
            Character::SizeW => "size_w",
            Character::SizeH => "size_h",
            Character::FontId => "font_id",
            Character::Ascii => "ascii",
            Character::CreatedAt => "created_at",
            Character::UserData => "user_data",
        };

        Self::from(str)
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

impl From<Font> for Iden {
    fn from(value: Font) -> Self {
        let str = match value {
            Font::Table => "font",
            Font::Id => "id",
            Font::Name => "name",
            Font::Variant => "variant",
            Font::Language => "language",
        };
        Self::from(str)
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

impl From<Glyph> for Iden {
    fn from(value: Glyph) -> Self {
        let str = match value {
            Glyph::Table => "glyph",
            Glyph::Id => "id",
            Glyph::Image => "image",
            Glyph::Aspect => "aspect",
            Glyph::Tokens => "tokens",
        };
        Self::from(str)
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

impl From<Task> for Iden {
    fn from(value: Task) -> Self {
        Self::from(match value {
            Task::Table => "task",
            Task::Id => "id",
            Task::IsDone => "is_done",
        })
    }
}
