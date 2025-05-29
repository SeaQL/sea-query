//! Configurations for test cases and examples. Not intended for actual use.

use std::fmt;

#[cfg(feature = "with-json")]
pub use serde_json::json;

use crate::{Iden, IdenImpl};

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

impl Iden for Character {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
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
                Self::Ascii => "ascii",
                Self::CreatedAt => "created_at",
                Self::UserData => "user_data",
            }
        )
        .unwrap();
    }
}

impl From<Character> for IdenImpl {
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

        Self::new(str)
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
    fn unquoted(&self, s: &mut dyn fmt::Write) {
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

impl From<Font> for IdenImpl {
    fn from(value: Font) -> Self {
        let str = match value {
            Font::Table => "font",
            Font::Id => "id",
            Font::Name => "name",
            Font::Variant => "variant",
            Font::Language => "language",
        };
        Self::new(str)
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

impl Iden for Glyph {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "glyph",
                Self::Id => "id",
                Self::Image => "image",
                Self::Aspect => "aspect",
                Self::Tokens => "tokens",
            }
        )
        .unwrap();
    }
}

impl From<Glyph> for IdenImpl {
    fn from(value: Glyph) -> Self {
        let str = match value {
            Glyph::Table => "glyph",
            Glyph::Id => "id",
            Glyph::Image => "image",
            Glyph::Aspect => "aspect",
            Glyph::Tokens => "tokens",
        };
        Self::new(str)
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

impl Iden for Task {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "task",
                Self::Id => "id",
                Self::IsDone => "is_done",
            }
        )
        .unwrap();
    }
}

impl From<Task> for IdenImpl {
    fn from(value: Task) -> Self {
        Self::new(match value {
            Task::Table => "task",
            Task::Id => "id",
            Task::IsDone => "is_done",
        })
    }
}
