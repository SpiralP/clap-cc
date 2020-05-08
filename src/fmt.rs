use std::fmt;

struct ANSIString {
    s: String,
}
impl ANSIString {
    pub fn from<S: Into<String>>(s: S) -> Self { Self { s: s.into() } }
}
impl fmt::Display for ANSIString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", &self.s) }
}

enum Color {
    Green,
    RedBold,
    Yellow,
}
impl Color {
    pub fn paint(&self, s: &str) -> ANSIString {
        let color_code = match self {
            Color::Green => classicube_helpers::color::LIME,
            Color::RedBold => classicube_helpers::color::RED,
            Color::Yellow => classicube_helpers::color::YELLOW,
        };

        ANSIString::from(format!("{}{}", color_code, s))
    }
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ColorWhen {
    Auto,
    Always,
    Never,
}

#[doc(hidden)]
pub struct ColorizerOption {
    pub use_stderr: bool,
    pub when: ColorWhen,
}

#[doc(hidden)]
pub struct Colorizer {
    when: ColorWhen,
}

macro_rules! color {
    ($_self:ident, $c:ident, $m:expr) => {
        match $_self.when {
            ColorWhen::Auto => Format::$c($m),
            ColorWhen::Always => Format::$c($m),
            ColorWhen::Never => Format::None($m),
        }
    };
}

impl Colorizer {
    pub fn new(option: ColorizerOption) -> Colorizer { Colorizer { when: option.when } }

    pub fn good<T>(&self, msg: T) -> Format<T>
    where
        T: fmt::Display + AsRef<str>,
    {
        debugln!("Colorizer::good;");
        color!(self, Good, msg)
    }

    pub fn warning<T>(&self, msg: T) -> Format<T>
    where
        T: fmt::Display + AsRef<str>,
    {
        debugln!("Colorizer::warning;");
        color!(self, Warning, msg)
    }

    pub fn error<T>(&self, msg: T) -> Format<T>
    where
        T: fmt::Display + AsRef<str>,
    {
        debugln!("Colorizer::error;");
        color!(self, Error, msg)
    }

    pub fn none<T>(&self, msg: T) -> Format<T>
    where
        T: fmt::Display + AsRef<str>,
    {
        debugln!("Colorizer::none;");
        Format::None(msg)
    }
}

impl Default for Colorizer {
    fn default() -> Self {
        Colorizer::new(ColorizerOption {
            use_stderr: true,
            when: ColorWhen::Auto,
        })
    }
}

/// Defines styles for different types of error messages. Defaults to Error=Red, Warning=Yellow,
/// and Good=Green
#[derive(Debug)]
#[doc(hidden)]
pub enum Format<T> {
    /// Defines the style used for errors, defaults to Red
    Error(T),
    /// Defines the style used for warnings, defaults to Yellow
    Warning(T),
    /// Defines the style used for good values, defaults to Green
    Good(T),
    /// Defines no formatting style
    None(T),
}

impl<T: AsRef<str>> Format<T> {
    fn format(&self) -> ANSIString {
        match *self {
            Format::Error(ref e) => Color::RedBold.paint(e.as_ref()),
            Format::Warning(ref e) => Color::Yellow.paint(e.as_ref()),
            Format::Good(ref e) => Color::Green.paint(e.as_ref()),
            Format::None(ref e) => ANSIString::from(e.as_ref()),
        }
    }
}

#[cfg(not(feature = "color"))]
#[cfg_attr(feature = "lints", allow(match_same_arms))]
impl<T: fmt::Display> Format<T> {
    fn format(&self) -> &T {
        match *self {
            Format::Error(ref e) => e,
            Format::Warning(ref e) => e,
            Format::Good(ref e) => e,
            Format::None(ref e) => e,
        }
    }
}

impl<T: AsRef<str>> fmt::Display for Format<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", &self.format()) }
}

#[cfg(not(feature = "color"))]
impl<T: fmt::Display> fmt::Display for Format<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", &self.format()) }
}

#[cfg(all(test, feature = "color"))]
mod test {
    use super::*;

    #[test]
    fn colored_output() {
        let err = Format::Error("error");
        assert_eq!(
            &*format!("{}", err),
            &*format!("{}", Color::RedBold.paint("error"))
        );
        let good = Format::Good("good");
        assert_eq!(
            &*format!("{}", good),
            &*format!("{}", Color::Green.paint("good"))
        );
        let warn = Format::Warning("warn");
        assert_eq!(
            &*format!("{}", warn),
            &*format!("{}", Color::Yellow.paint("warn"))
        );
        let none = Format::None("none");
        assert_eq!(
            &*format!("{}", none),
            &*format!("{}", ANSIString::from("none"))
        );
    }
}
