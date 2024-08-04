use std::ffi::OsStr;

#[derive(Debug, thiserror::Error)]
pub enum AcgeError {
	#[error("Invalid image file extension [{0}]")]
	InvalidImageFileExtension(String),

	#[error("File missing component in path necessary for parsing")]
	NoOsStrToIndoc,

	#[error("Missing files to use for filtering")]
	NoFilesToFilter,

	#[error("OsStr provided is not UTF-8 encodable")]
	OsStrNonUtf8,
}

/// Trait for getting a common prefix between two sequences.
pub trait CommonPrefix {
	fn common_prefix(self, other: Self) -> Self;
}

impl CommonPrefix for &str {
	fn common_prefix(self, other: Self) -> Self {
		for ((self_offset, self_char), other_char) in self.char_indices().zip(other.chars()) {
			if self_char != other_char {
				return self.split_at(self_offset).0;
			}
		}

		if self.len() < other.len() {
			self
		} else {
			other
		}
	}
}

impl CommonPrefix for String {
	fn common_prefix(self, other: Self) -> Self {
		<&str as CommonPrefix>::common_prefix(&self, &other).to_string()
	}
}

/// For converting non str types into a str easily.
/// E.g. `Option<&OsStr>` -> `Result<&str, AcgeError>`
pub trait IndocStr<'a> {
	fn indoc_str(self) -> Result<&'a str, AcgeError>;
}

impl<'a> IndocStr<'a> for &'a OsStr {
	fn indoc_str(self) -> Result<&'a str, AcgeError> {
		self.to_str().ok_or(AcgeError::OsStrNonUtf8)
	}
}

impl<'a> IndocStr<'a> for Option<&'a OsStr> {
	fn indoc_str(self) -> Result<&'a str, AcgeError> {
		let os_str = self.ok_or(AcgeError::NoOsStrToIndoc)?;

		os_str.to_str().ok_or(AcgeError::OsStrNonUtf8)
	}
}

/// Version of [IndocStr] but for [String] instead.
pub trait IndocString {
	fn indoc_string(self) -> Result<String, AcgeError>;
}

impl<'a, T: IndocStr<'a>> IndocString for T {
	fn indoc_string(self) -> Result<String, AcgeError> {
		self.indoc_str().map(|str| str.to_string())
	}
}
