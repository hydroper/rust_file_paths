/*!
Work with file paths by text only.

In the Windows operating system, absolute paths may either start with a drive letter followed by
a colon or an UNC path prefix (`\\`). Therefore, this crate provides
a `Path` that is based on a variant ([_PlatformPathVariant_]), which you don't need to always
specify. This variant indicates whether to interpret Windows absolute paths
or not.

There are two _PlatformPathVariant_ variants currently:

- _Common_
- _Windows_

The constant `PlatformPathVariant::NATIVE` is one of these variants
based on the target platform. For the Windows operating system, it
is always _Windows_. For other platforms, it's always _Common_.

# Example

```
use file_paths::Path;

assert_eq!("a", Path::new_common("a/b").resolve("..").to_string());
assert_eq!("a", Path::new_common("a/b/..").to_string());
assert_eq!("a/b/c/d/e", Path::from_n_common(["a/b", "c/d", "e/f", ".."]).to_string());
assert_eq!("../../c/d", Path::new_common("/a/b").relative("/c/d"));
```
*/

mod reg_exp;
use reg_exp::*;

pub(crate) mod common;
pub(crate) mod argumented;

/// Indicates if special absolute paths are considered.
///
/// Currently, only two variants are defined, considering that there is
/// no known operating system with different path support other than Windows:
/// 
/// - `Common`
/// - `Windows`
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum PlatformPathVariant {
    /// Indicates that the path is manipulated in a generic way,
    /// that is, the same behavior from the [`file_paths`] module.
    Common,
    /// Indicates that the path is manipulated compatibly with the Windows operating system.
    Windows,
}

impl PlatformPathVariant {
    /// The variant that represents the build's target platform.
    pub const NATIVE: Self = {
        #[cfg(target_os = "windows")] {
            Self::Windows
        }
        #[cfg(not(target_os = "windows"))] {
            Self::Common
        }
    };
}

/// The `Path` structure represents a textual path based
/// on a [_PlatformPathVariant_].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Path(String, PlatformPathVariant);

impl Path {
    /// Constructs a `Path` with a given `variant`. This method
    /// will resolve the specified path.
    pub fn new(path: &str, variant: PlatformPathVariant) -> Self {
        Self(argumented::resolve_one(path.into(), variant), variant)
    }

    /// Constructs a `Path` whose variant is `Common`. This method
    /// will resolve the specified path.
    pub fn new_common(path: &str) -> Self {
        Self(argumented::resolve_one(path.into(), PlatformPathVariant::Common), PlatformPathVariant::Common)
    }

    /// Constructs a `Path` whose variant is chosen according to the target platform.
    /// This method will resolve the specified path.
    pub fn new_native(path: &str) -> Self {
        Self(argumented::resolve_one(path.into(), PlatformPathVariant::NATIVE), PlatformPathVariant::NATIVE)
    }

    /// Constructs a `Path` from multiple paths and a given `variant`.
    pub fn from_n<'a, T: IntoIterator<Item = &'a str>>(paths: T, variant: PlatformPathVariant) -> Self {
        Self(argumented::resolve_n(paths, variant), variant)
    }

    /// Constructs a `Path` from multiple paths and a `Common` variant.
    pub fn from_n_common<'a, T: IntoIterator<Item = &'a str>>(paths: T) -> Self {
        Self::from_n(paths, PlatformPathVariant::Common)
    }

    /// Constructs a `Path` from multiple paths and a variant based on
    /// the target platform.
    pub fn from_n_native<'a, T: IntoIterator<Item = &'a str>>(paths: T) -> Self {
        Self::from_n(paths, PlatformPathVariant::NATIVE)
    }

    /// Returns the variant this `Path` object is based on.
    pub fn variant(&self) -> PlatformPathVariant {
        self.1
    }

    /// Indicates whether the `Path` is absolute or not.
    pub fn is_absolute(&self) -> bool {
        argumented::is_absolute(&self.0, self.1)
    }

    /// Resolves `path2` relative to `path1`.
    ///
    /// Behavior:
    /// - Eliminates the portions `..` and `.`.
    /// - If `path2` is absolute, this function returns a resolution of solely `path2`.
    /// - All path separators that are backslashes (`\`) are replaced by forward ones (`/`).
    /// - If any path is absolute, this function returns an absolute path.
    /// - Any empty portion and trailing path separators, such as in `a/b/` and `a//b` are eliminated.
    pub fn resolve(&self, path2: &str) -> Path {
        Path(argumented::resolve(&self.0, path2, self.1), self.1)
    }

    /// Resolves multiple paths relative to this path. The
    /// behavior is similiar to [`.resolve`]. If the given
    /// set has no items, an empty string is returned.
    pub fn resolve_n<'a, T: IntoIterator<Item = &'a str>>(&self, paths: T) -> Path {
        Path(argumented::resolve(&self.0, &argumented::resolve_n(paths, self.1), self.1), self.1)
    }

    /**
    Finds the relative path from this path to `to_path`.

    # Behavior:

    - If the paths refer to the same path, this function returns
    an empty string.
    - The function ensures that both paths are absolute and resolves
    any `..` and `.` portions inside.
    - If both paths have different prefix, `to_path` is returned.

    # Panics

    Panics if given paths are not absolute.

    # Example

    ```
    use file_paths::Path;
    assert_eq!("", Path::new_common("/a/b").relative("/a/b"));
    assert_eq!("c", Path::new_common("/a/b").relative("/a/b/c"));
    assert_eq!("../../c/d", Path::new_common("/a/b").relative("/c/d"));
    assert_eq!("../c", Path::new_common("/a/b").relative("/a/c"));
    ```
    */
    pub fn relative(&self, to_path: &str) -> String {
        argumented::relative(&self.0, to_path, self.1)
    }

    /// Changes the extension of a path and returns a new string.
    /// This method adds any lacking dot (`.`) prefix automatically to the
    /// `extension` argument.
    ///
    /// This method allows multiple dots per extension. If that is not
    /// desired, use [`.change_last_extension`].
    ///
    /// # Example
    /// 
    /// ```
    /// use file_paths::Path;
    /// assert_eq!("a.y", Path::new_common("a.x").change_extension(".y").to_string());
    /// assert_eq!("a.z", Path::new_common("a.x.y").change_extension(".z").to_string());
    /// assert_eq!("a.z.w", Path::new_common("a.x.y").change_extension(".z.w").to_string());
    /// ```
    ///
    pub fn change_extension(&self, extension: &str) -> Path {
        Self(change_extension(&self.0, extension), self.1)
    }

    /// Changes only the last extension of a path and returns a new string.
    /// This method adds any lacking dot (`.`) prefix automatically to the
    /// `extension` argument.
    ///
    /// # Panics
    ///
    /// Panics if the extension contains more than one dot.
    ///
    pub fn change_last_extension(&self, extension: &str) -> Path {
        Self(change_last_extension(&self.0, extension), self.1)
    }

    /// Checks if a file path has a specific extension.
    /// This method adds any lacking dot (`.`) prefix automatically to the
    /// `extension` argument.
    pub fn has_extension(&self, extension: &str) -> bool {
        has_extension(&self.0, extension)
    }

    /// Checks if a file path has any of multiple specific extensions.
    /// This method adds any lacking dot (`.`) prefix automatically to each
    /// extension argument.
    pub fn has_extensions<'a, T: IntoIterator<Item = &'a str>>(&self, extensions: T) -> bool {
        has_extensions(&self.0, extensions)
    }

    /// Returns the base name of a file path.
    ///
    /// # Example
    /// 
    /// ```
    /// use file_paths::Path;
    /// assert_eq!("qux.html", Path::new_common("foo/qux.html").base_name());
    /// ```
    pub fn base_name(&self) -> String {
        base_name(&self.0)
    }

    /// Returns the base name of a file path, removing any of the specified extensions.
    /// This method adds any lacking dot (`.`) prefix automatically to each
    /// extension argument.
    ///
    /// # Example
    /// 
    /// ```
    /// use file_paths::Path;
    /// assert_eq!("qux", Path::new_common("foo/qux.html").base_name_without_ext([".html"]));
    /// ```
    pub fn base_name_without_ext<'a, T>(&self, extensions: T) -> String
        where T: IntoIterator<Item = &'a str>
    {
        base_name_without_ext(&self.0, extensions)
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

static STARTS_WITH_PATH_SEPARATOR: StaticRegExp = static_reg_exp!(r"^[/\\]");

fn change_extension(path: &str, extension: &str) -> String {
    let extension = (if extension.starts_with('.') { "" } else { "." }).to_owned() + extension;
    if reg_exp_find!(r"(\.[^\.]+)+$", path).is_none() {
        return path.to_owned() + &extension;
    }
    reg_exp_replace!(r"(\.[^\.]+)+$", path, |_, _| &extension).into_owned()
}

fn change_last_extension(path: &str, extension: &str) -> String {
    let extension = (if extension.starts_with('.') { "" } else { "." }).to_owned() + extension;
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
    (if extension.starts_with('.') { "" } else { "." }).to_owned() + extension
}

fn has_extension(path: &str, extension: &str) -> bool {
    let extension = (if extension.starts_with('.') { "" } else { "." }).to_owned() + extension;
    path.ends_with(&extension_arg(&extension))
}

fn has_extensions<'a, T: IntoIterator<Item = &'a str>>(path: &str, extensions: T) -> bool {
    extensions.into_iter().any(|ext| has_extension(path, ext))
}

fn base_name(path: &str) -> String {
    path.split('/').last().map_or("", |s| s).to_owned()
}

fn base_name_without_ext<'a, T>(path: &str, extensions: T) -> String
    where T: IntoIterator<Item = &'a str>
{
    let extensions = extensions.into_iter().map(extension_arg).collect::<Vec<String>>();
    path.split('/').last().map_or("".to_owned(), |base| {
        reg_exp_replace!(r"(\.[^\.]+)+$", base, |_, prev_ext: &str| {
            (if extensions.iter().any(|ext| ext == prev_ext) { "" } else { prev_ext }).to_owned()
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