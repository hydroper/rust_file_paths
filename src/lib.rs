/*!
Work with generic file paths.

# Versions

Functions that are involved with absolute paths are available in three
different submodules:

- [`common`]
- [`argumented`]
- [`native`]

In the Windows operating system, absolute paths may either start with a drive letter followed by
a colon or an UNC path prefix (`\\`).

Here is which one you should choose:

- The `common` submodule if you require absolute paths that
  start with a path separator only.
- The `argumented` submodule if you require absolute paths
  interpreted according to a given platform.
- The `native` submodule if you require absolute paths based
  on the target platform.

# Example

```
use file_paths::common as file_paths;
assert_eq!("a", file_paths::resolve("a/b", ".."));
assert_eq!("a", file_paths::resolve_one("a/b/.."));
assert_eq!("a/b/c/d/e", file_paths::resolve_n(["a/b", "c/d", "e/f", ".."]));
assert_eq!("../../c/d", file_paths::relative("/a/b", "/c/d"));
```
*/

mod reg_exp;
use reg_exp::*;

pub mod common;
pub mod argumented;
pub mod native;

static STARTS_WITH_PATH_SEPARATOR: StaticRegExp = static_reg_exp!(r"^[/\\]");

/// Changes the extension of a path and returns a new string.
/// This method adds any lacking dot (`.`) prefix automatically to the
/// `extension` argument.
///
/// This method allows multiple dots per extension. If that is not
/// desired, use [`change_last_extension`].
///
/// # Example
/// 
/// ```
/// assert_eq!("a.y", file_paths::change_extension("a.x", ".y"));
/// assert_eq!("a.z", file_paths::change_extension("a.x.y", ".z"));
/// assert_eq!("a.z.w", file_paths::change_extension("a.x.y", ".z.w"));
/// ```
///
pub fn change_extension(path: &str, extension: &str) -> String {
    let extension = (if extension.starts_with(".") { "" } else { "." }).to_owned() + extension;
    if reg_exp_find!(r"(\.[^\.]+)+$", path).is_none() {
        return path.to_owned() + &extension;
    }
    reg_exp_replace!(r"(\.[^\.]+)+$", path, |_, _| &extension).into_owned()
}

/// Changes only the last extension of a path and returns a new string.
/// This method adds any lacking dot (`.`) prefix automatically to the
/// `extension` argument.
///
/// # Exceptions
///
/// Panics if the extension contains more than one dot.
///
pub fn change_last_extension(path: &str, extension: &str) -> String {
    let extension = (if extension.starts_with(".") { "" } else { "." }).to_owned() + extension;
    assert!(
        extension[1..].find('.').is_none(),
        "The argument to file_paths::change_last_extension() must only contain one extension; got {}",
        extension
    );
    if reg_exp_find!(r"(\..+)$", path).is_none() {
        return path.to_owned() + &extension;
    }
    reg_exp_replace!(r"(\..+)$", path, |_, _| &extension).into_owned()
}

/// Adds prefix dot to extension if missing.
fn extension_arg(extension: &str) -> String {
    (if extension.starts_with(".") { "" } else { "." }).to_owned() + extension
}

/// Checks if a file path has a specific extension.
/// This method adds any lacking dot (`.`) prefix automatically to the
/// `extension` argument.
pub fn has_extension(path: &str, extension: &str) -> bool {
    let extension = (if extension.starts_with(".") { "" } else { "." }).to_owned() + extension;
    path.ends_with(&extension_arg(&extension))
}

/// Checks if a file path has any of multiple specific extensions.
/// This method adds any lacking dot (`.`) prefix automatically to each
/// extension argument.
pub fn has_extensions<'a, T: IntoIterator<Item = &'a str>>(path: &str, extensions: T) -> bool {
    extensions.into_iter().any(|ext| has_extension(path, ext))
}

/// Returns the base name of a file path.
///
/// # Example
/// 
/// ```
/// assert_eq!("qux.html", file_paths::base_name("foo/qux.html"));
/// ```
pub fn base_name(path: &str) -> String {
    path.split('/').last().map_or("", |s| s).to_owned()
}

/// Returns the base name of a file path, removing any of the specified extensions.
/// This method adds any lacking dot (`.`) prefix automatically to each
/// extension argument.
///
/// # Example
/// 
/// ```
/// assert_eq!("qux", file_paths::base_name_without_ext("foo/qux.html", [".html"]));
/// ```
pub fn base_name_without_ext<'a, T>(path: &str, extensions: T) -> String
    where T: IntoIterator<Item = &'a str>
{
    let extensions = extensions.into_iter().map(|s| extension_arg(s)).collect::<Vec<String>>();
    path.split('/').last().map_or("".to_owned(), |base| {
        reg_exp_replace!(r"(\.[^\.]+)+$", base, |_, prev_ext: &str| {
            (if extensions.iter().any(|ext| ext == &prev_ext) { "" } else { prev_ext }).to_owned()
        }).into_owned()
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert!(has_extensions("a.x", [".x", ".y"]));
        assert_eq!("a.y", change_extension("a.x", ".y"));
        assert_eq!("a.z", change_extension("a.x.y", ".z"));
        assert_eq!("a.z.w", change_extension("a.x.y", ".z.w"));

        assert_eq!("qux.html", base_name("foo/qux.html"));
        assert_eq!("qux", base_name_without_ext("foo/qux.html", [".html"]));
    }
}