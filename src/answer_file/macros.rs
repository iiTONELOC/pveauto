/// A macro to define string-backed enums with full TOML/Serde support.
///
/// You provide:
/// - the enum + variant => `"string"` mappings
/// - the error type to use for `FromStr::Err`
/// - the error value to return on invalid input
///
/// ## Behavior
///
/// This macro generates an enum that:
/// - Is backed by **explicit string values** (not variant names)
/// - Parses from strings via `FromStr`
/// - Displays as its string value via `Display`
/// - **Deserializes from TOML/Serde using the string values**
/// - **Serializes to TOML/Serde using the string values**
///
/// This guarantees:
/// - Round-trip safety (`from_toml → to_toml → from_toml`)
/// - Invalid values fail **at deserialization time**, not later validation
/// - TOML uses stable, spec-defined strings instead of Rust variant names
///
/// ## Example
/// ```rs
/// string_enum!(
///     #[derive(Debug, Clone, PartialEq, Eq)]
///     pub enum RebootMode {
///         Reboot  => "reboot",
///         PowerOff => "power-off",
///     },
///     GlobalConfigError,
///     GlobalConfigError::RebootMode
/// );
/// ```
///
/// ## Which Generates:
/// ```rs
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// pub enum RebootMode {
///     Reboot,
///     PowerOff,
/// }
///
/// impl RebootMode {
///     /// Returns the canonical string representation used in TOML and display output.
///     pub fn as_str(&self) -> &'static str {
///         match self {
///             Self::Reboot => "reboot",
///             Self::PowerOff => "power-off",
///         }
///     }
/// }
///
/// // Parses from the canonical string form
/// impl ::std::str::FromStr for RebootMode {
///     type Err = GlobalConfigError;
///
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         match s {
///             "reboot" => Ok(Self::Reboot),
///             "power-off" => Ok(Self::PowerOff),
///             _ => Err(GlobalConfigError::RebootMode),
///         }
///     }
/// }
///
/// // Displays as the canonical string form
/// impl ::std::fmt::Display for RebootMode {
///     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
///         f.write_str(self.as_str())
///     }
/// }
///
/// // Deserializes from TOML/Serde using the string value
/// // Invalid values fail immediately with the provided error
///
/// // Serializes to TOML/Serde using the string value
/// // (never the Rust variant name)
/// ```
macro_rules! string_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident => $str:expr),+ $(,)?
        },
        $err_ty:ty,
        $err_val:expr $(,)?
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant),+
        }

        impl $name {
            #[inline]
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $str),+
                }
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $err_ty;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($str => Ok(Self::$variant)),+,
                    _ => Err($err_val),
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        /// Deserializes from a string using `FromStr`.
        /// Invalid input fails immediately with the configured error.
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                s.parse().map_err(::serde::de::Error::custom)
            }
        }

        /// Serializes using the canonical string representation.
        /// This guarantees stable, spec-compliant TOML output.
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
    };
}

/// A macro to define error enums with associated string error codes.
///
/// ## Behavior
///
/// This macro generates:
/// - A strongly-typed error enum
/// - A stable string error code for each variant
/// - `Display` implementation that emits the error code
/// - `std::error::Error` implementation
///
/// Intended for:
/// - Config validation errors
/// - TOML parsing failures
/// - User-facing, machine-stable error reporting
///
/// ## Example
/// ```rs
/// config_error_enum!(
///     #[derive(Debug, PartialEq)]
///     pub enum ExampleError {
///         VariantOne => "example.error.one",
///         VariantTwo => "example.error.two",
///     }
/// );
/// ```
///
/// ## Which Generates:
/// ```rs
/// #[derive(Debug, PartialEq)]
/// pub enum ExampleError {
///     VariantOne,
///     VariantTwo,
/// }
///
/// impl ExampleError {
///     pub fn code(&self) -> &'static str {
///         match self {
///             Self::VariantOne => "example.error.one",
///             Self::VariantTwo => "example.error.two",
///         }
///     }
/// }
///
/// impl fmt::Display for ExampleError {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         f.write_str(self.code())
///     }
/// }
/// ```
macro_rules! config_error_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident => $code:expr),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant),+
        }

        impl $name {
            pub fn code(&self) -> &'static str {
                match self {
                    $(Self::$variant => $code),+
                }
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(self.code())
            }
        }

        impl std::error::Error for $name {}
    };
}

pub(crate) use config_error_enum;
pub(crate) use string_enum;
