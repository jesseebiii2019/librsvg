//! `userSpaceOnUse` or `objectBoundingBox` values.

use cssparser::Parser;

use crate::error::*;
use crate::parsers::ParseToParseError;

/// Defines the units to be used for things that can consider a
/// coordinate system in terms of the current transformation, or in
/// terms of the current object's bounding box.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CoordUnits {
    UserSpaceOnUse,
    ObjectBoundingBox,
}

impl ParseToParseError for CoordUnits {
    fn parse_to_parse_error<'i>(parser: &mut Parser<'i, '_>) -> Result<Self, CssParseError<'i>> {
        Ok(parse_identifiers!(
            parser,
            "userSpaceOnUse" => CoordUnits::UserSpaceOnUse,
            "objectBoundingBox" => CoordUnits::ObjectBoundingBox,
        )?)
    }
}

/// Creates a newtype around `CoordUnits`, with a default value.
///
/// SVG attributes that can take `userSpaceOnUse` or
/// `objectBoundingBox` values often have different default values
/// depending on the type of SVG element.  We use this macro to create
/// a newtype for each SVG element and attribute that requires values
/// of this type.  The newtype provides an `impl Default` with the
/// specified `$default` value.
#[macro_export]
macro_rules! coord_units {
    ($name:ident, $default:expr) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $name(pub CoordUnits);

        impl Default for $name {
            fn default() -> Self {
                $name($default)
            }
        }

        impl From<$name> for CoordUnits {
            fn from(u: $name) -> Self {
                u.0
            }
        }

        impl $crate::parsers::ParseToParseError for $name {
            fn parse_to_parse_error<'i>(
                parser: &mut ::cssparser::Parser<'i, '_>,
            ) -> Result<Self, $crate::error::CssParseError<'i>> {
                Ok($name($crate::coord_units::CoordUnits::parse_to_parse_error(parser)?))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    coord_units!(MyUnits, CoordUnits::ObjectBoundingBox);

    #[test]
    fn parsing_invalid_strings_yields_error() {
        assert!(MyUnits::parse_str_to_parse_error("").is_err());
        assert!(MyUnits::parse_str_to_parse_error("foo").is_err());
    }

    #[test]
    fn parses_paint_server_units() {
        assert_eq!(
            MyUnits::parse_str_to_parse_error("userSpaceOnUse"),
            Ok(MyUnits(CoordUnits::UserSpaceOnUse))
        );
        assert_eq!(
            MyUnits::parse_str_to_parse_error("objectBoundingBox"),
            Ok(MyUnits(CoordUnits::ObjectBoundingBox))
        );
    }

    #[test]
    fn has_correct_default() {
        assert_eq!(MyUnits::default(), MyUnits(CoordUnits::ObjectBoundingBox));
    }

    #[test]
    fn converts_to_coord_units() {
        assert_eq!(
            CoordUnits::from(MyUnits(CoordUnits::ObjectBoundingBox)),
            CoordUnits::ObjectBoundingBox
        );
    }
}
