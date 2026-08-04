use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{self, Debug, Display};
use serde::de::Expected;

pub struct Error {
    imp: ErrorImpl,
}

pub(crate) fn erase<E: serde::de::Error>(err: E) -> Error {
    serde::de::Error::custom(err)
}

pub(crate) fn unerase<E: serde::de::Error>(err: Error) -> E {
    err.as_serde()
}

impl serde::de::StdError for Error {}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = self.as_serde::<serde::de::value::Error>();
        Display::fmt(&error, formatter)
    }
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = self.as_serde::<serde::de::value::Error>();
        Debug::fmt(&error, formatter)
    }
}

enum ErrorImpl {
    Custom(String),
    InvalidType {
        unexpected: Unexpected,
        expected: String,
    },
    InvalidValue {
        unexpected: Unexpected,
        expected: String,
    },
    InvalidLength {
        len: usize,
        expected: String,
    },
    UnknownVariant {
        variant: String,
        expected: &'static [&'static str],
    },
    UnknownField {
        field: String,
        expected: &'static [&'static str],
    },
    MissingField {
        field: &'static str,
    },
    DuplicateField {
        field: &'static str,
    },
}

enum Unexpected {
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Char(char),
    Str(String),
    Bytes(Vec<u8>),
    Unit,
    Option,
    NewtypeStruct,
    Seq,
    Map,
    Enum,
    UnitVariant,
    NewtypeVariant,
    TupleVariant,
    StructVariant,
    Other(String),
}

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        let imp = ErrorImpl::Custom(msg.to_string());
        Self { imp }
    }

    fn invalid_type(unexpected: serde::de::Unexpected<'_>, expected: &dyn Expected) -> Self {
        let imp = ErrorImpl::InvalidType {
            unexpected: Unexpected::from_serde(unexpected),
            expected: expected.to_string(),
        };
        Self { imp }
    }

    fn invalid_value(unexpected: serde::de::Unexpected<'_>, expected: &dyn Expected) -> Self {
        let imp = ErrorImpl::InvalidValue {
            unexpected: Unexpected::from_serde(unexpected),
            expected: expected.to_string(),
        };
        Self { imp }
    }

    fn invalid_length(len: usize, expected: &dyn Expected) -> Self {
        let imp = ErrorImpl::InvalidLength {
            len,
            expected: expected.to_string(),
        };
        Self { imp }
    }

    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        let imp = ErrorImpl::UnknownVariant {
            variant: variant.to_owned(),
            expected,
        };
        Self { imp }
    }

    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        let imp = ErrorImpl::UnknownField {
            field: field.to_owned(),
            expected,
        };
        Self { imp }
    }

    fn missing_field(field: &'static str) -> Self {
        let imp = ErrorImpl::MissingField { field };
        Self { imp }
    }

    fn duplicate_field(field: &'static str) -> Self {
        let imp = ErrorImpl::DuplicateField { field };
        Self { imp }
    }
}

impl Error {
    fn as_serde<E: serde::de::Error>(&self) -> E {
        match &self.imp {
            ErrorImpl::Custom(msg) => E::custom(msg),
            ErrorImpl::InvalidType {
                unexpected,
                expected,
            } => E::invalid_type(unexpected.as_serde(), &expected.as_str()),
            ErrorImpl::InvalidValue {
                unexpected,
                expected,
            } => E::invalid_value(unexpected.as_serde(), &expected.as_str()),
            ErrorImpl::InvalidLength { len, expected } => {
                E::invalid_length(*len, &expected.as_str())
            }
            ErrorImpl::UnknownVariant { variant, expected } => {
                E::unknown_variant(variant, expected)
            }
            ErrorImpl::UnknownField { field, expected } => E::unknown_field(field, expected),
            ErrorImpl::MissingField { field } => E::missing_field(field),
            ErrorImpl::DuplicateField { field } => E::duplicate_field(field),
        }
    }
}

impl Unexpected {
    fn from_serde(unexpected: serde::de::Unexpected<'_>) -> Self {
        match unexpected {
            serde::de::Unexpected::Bool(value) => Self::Bool(value),
            serde::de::Unexpected::Unsigned(value) => Self::Unsigned(value),
            serde::de::Unexpected::Signed(value) => Self::Signed(value),
            serde::de::Unexpected::Float(value) => Self::Float(value),
            serde::de::Unexpected::Char(value) => Self::Char(value),
            serde::de::Unexpected::Str(value) => Self::Str(value.to_owned()),
            serde::de::Unexpected::Bytes(value) => Self::Bytes(value.to_owned()),
            serde::de::Unexpected::Unit => Self::Unit,
            serde::de::Unexpected::Option => Self::Option,
            serde::de::Unexpected::NewtypeStruct => Self::NewtypeStruct,
            serde::de::Unexpected::Seq => Self::Seq,
            serde::de::Unexpected::Map => Self::Map,
            serde::de::Unexpected::Enum => Self::Enum,
            serde::de::Unexpected::UnitVariant => Self::UnitVariant,
            serde::de::Unexpected::NewtypeVariant => Self::NewtypeVariant,
            serde::de::Unexpected::TupleVariant => Self::TupleVariant,
            serde::de::Unexpected::StructVariant => Self::StructVariant,
            serde::de::Unexpected::Other(msg) => Self::Other(msg.to_owned()),
        }
    }

    fn as_serde(&self) -> serde::de::Unexpected<'_> {
        match self {
            Self::Bool(value) => serde::de::Unexpected::Bool(*value),
            Self::Unsigned(value) => serde::de::Unexpected::Unsigned(*value),
            Self::Signed(value) => serde::de::Unexpected::Signed(*value),
            Self::Float(value) => serde::de::Unexpected::Float(*value),
            Self::Char(value) => serde::de::Unexpected::Char(*value),
            Self::Str(value) => serde::de::Unexpected::Str(value),
            Self::Bytes(value) => serde::de::Unexpected::Bytes(value),
            Self::Unit => serde::de::Unexpected::Unit,
            Self::Option => serde::de::Unexpected::Option,
            Self::NewtypeStruct => serde::de::Unexpected::NewtypeStruct,
            Self::Seq => serde::de::Unexpected::Seq,
            Self::Map => serde::de::Unexpected::Map,
            Self::Enum => serde::de::Unexpected::Enum,
            Self::UnitVariant => serde::de::Unexpected::UnitVariant,
            Self::NewtypeVariant => serde::de::Unexpected::NewtypeVariant,
            Self::TupleVariant => serde::de::Unexpected::TupleVariant,
            Self::StructVariant => serde::de::Unexpected::StructVariant,
            Self::Other(msg) => serde::de::Unexpected::Other(msg),
        }
    }
}
