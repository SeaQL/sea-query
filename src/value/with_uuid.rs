use super::*;

type_to_value!(Uuid, Uuid, Uuid);

macro_rules! fmt_uuid_to_box_value {
    ( $type: ty, $conversion_fn: ident ) => {
        impl From<$type> for Value {
            fn from(x: $type) -> Value {
                Value::uuid(x.into_uuid())
            }
        }

        impl Nullable for $type {
            fn null() -> Value {
                Value::uuid(None)
            }
        }

        impl ValueType for $type {
            fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                match v {
                    Value::Uuid(Some(x)) => Ok(x.$conversion_fn()),
                    _ => Err(ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($type).to_owned()
            }

            fn array_type() -> ArrayType {
                ArrayType::Uuid
            }

            fn column_type() -> ColumnType {
                ColumnType::Uuid
            }
        }
    };
}

fmt_uuid_to_box_value!(uuid::fmt::Braced, braced);
fmt_uuid_to_box_value!(uuid::fmt::Hyphenated, hyphenated);
fmt_uuid_to_box_value!(uuid::fmt::Simple, simple);
fmt_uuid_to_box_value!(uuid::fmt::Urn, urn);

impl Value {
    pub fn is_uuid(&self) -> bool {
        matches!(self, Self::Uuid(_))
    }
    pub fn as_ref_uuid(&self) -> Option<&Uuid> {
        match self {
            Self::Uuid(v) => v.as_ref(),
            _ => panic!("not Value::Uuid"),
        }
    }
}
