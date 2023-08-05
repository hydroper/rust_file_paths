/*!
Work with file paths based on the target platform.
*/

use super::argumented;
use argumented::PlatformPathVariant;

fn native_variant() -> PlatformPathVariant {
    PlatformPathVariant::native()
}

/// Resolves `path2` relative to `path1`. This methodd
/// has the same behavior from [`crate::common::resolve`],
/// except it can handle absolute paths such as from the Windows operating system.
pub fn resolve(path1: &str, path2: &str) -> String {
    argumented::resolve(path1, path2, native_variant())
}

/// Resolves multiple paths with the same behavior from
/// [`resolve`].
pub fn resolve_n<'a, T: IntoIterator<Item = &'a str>>(paths: T) -> String {
    argumented::resolve_n(paths, native_variant())
}

/// Resolves a single path with the same behavior from
/// [`resolve_n`].
pub fn resolve_one(path: &str) -> String {
    argumented::resolve_one(path, native_variant())
}

/// Determines if a path is absolute.
pub fn is_absolute(path: &str) -> bool {
    argumented::is_absolute(path, native_variant())
}

/// Finds the relative path from `from_path` and `to_path`.
/// This method has the same behavior from [`crate::common::relative`],
/// except that it can handle absolute paths such as from the Windows operating system.
/// If the paths have a different prefix, this function returns
/// `resolve_one(to_path)`.
///
/// # Exception
/// 
/// Panics if given paths are not absolute.
///
pub fn relative(from_path: &str, to_path: &str) -> String {
    argumented::relative(from_path, to_path, native_variant())
}

pub use argumented::{  
    change_extension,
    change_last_extension,
    has_extension,
    has_extensions,
    base_name,
    base_name_without_ext,
};