#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LineSeparatorType {
    Windows,
    Unix,
    UnixMixed,
}

impl LineSeparatorType {
    pub(crate) fn separator(&self) -> &'static str {
        match self {
            LineSeparatorType::Windows => "\r\n",
            LineSeparatorType::Unix => "\n",
            LineSeparatorType::UnixMixed => "\n",
        }
    }
}

impl From<LineSeparatorType> for &str {
    fn from(sep_type: LineSeparatorType) -> Self {
        sep_type.separator()
    }
}

impl PartialEq<str> for LineSeparatorType {
    fn eq(&self, other: &str) -> bool {
        let separator: &str = self.separator();
        if self == &LineSeparatorType::UnixMixed {
            let win_separator: &str = LineSeparatorType::Windows.into();
            other == win_separator || other == separator
        } else {
            other == separator
        }
    }
}

#[cfg(windows)]
pub const DEFAULT_LINE_SEPARATOR: LineSeparatorType = LineSeparatorType::Windows;

#[cfg(not(windows))]
pub const DEFAULT_LINE_SEPARATOR: LineSeparatorType = LineSeparatorType::Unix;

#[cfg(test)]
mod should {
    use rstest::*;

    use crate::line_separator::{DEFAULT_LINE_SEPARATOR, LineSeparatorType};

    #[rstest]
    #[case::windows(LineSeparatorType::Windows, "\r\n")]
    #[case::unix(LineSeparatorType::Unix, "\n")]
    #[case::mixed(LineSeparatorType::UnixMixed, "\n")]
    #[case::mixed(LineSeparatorType::UnixMixed, "\r\n")]
    fn compare_with_str(#[case] line_type: LineSeparatorType, #[case] expected: &str) {
        println!("Checking '{:?}'", line_type);
        assert_eq!(&line_type, expected);
    }

    #[test]
    fn convert_to_str() {
        let _sep: &str = LineSeparatorType::Windows.into();
        let _sep: &str = LineSeparatorType::Unix.into();
        let _sep: &str = LineSeparatorType::UnixMixed.into();
    }

    #[test]
    #[cfg(windows)]
    fn return_correct_default_for_windows_platform() {
        assert_eq!(LineSeparatorType::Windows, DEFAULT_LINE_SEPARATOR);
    }

    #[test]
    #[cfg(not(windows))]
    fn return_correct_default_for_other_platforms() {
        assert_eq!(LineSeparatorType::Unix, DEFAULT_LINE_SEPARATOR);
    }
}