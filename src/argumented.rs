/*!
Work with file paths in a cross-platform way.

This an argumented version of the `file_paths` crate.
Some of the methods in this module take a _PlatformPathVariant_ argument.
*/

use super::STARTS_WITH_PATH_SEPARATOR;

/// Indicates which kind of manipulation to perform in a path.
/// For example, it is given as the third for argument for `relative`.
///
/// Currently, only two variants are defined, seen that there is
/// no known operating system with different path support other than Windows:
/// 
/// - `Default`
/// - `Windows`
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum PlatformPathVariant {
    /// Indicates that the path is manipulated in a generic way,
    /// that is, the same behavior from the [`file_paths`] module.
    Default,
    /// Indicates that the path is manipulated compatibly with the Windows operating system.
    Windows,
}

impl PlatformPathVariant {
    /// Returns the variant that represents the build's target platform.
    pub fn native() -> Self {
        #[cfg(target_os = "windows")] {
            Self::Windows
        }
        #[cfg(not(target_os = "windows"))] {
            Self::Default
        }
    }
}

pub use super::{  
    change_extension,
    change_last_extension,
    has_extension,
    has_extensions,
    base_name,
    base_name_without_ext,
};

use super::reg_exp::*;

static STARTS_WITH_WINDOWS_PATH_PREFIX: StaticRegExp = static_reg_exp!(r#"(?x)
    ^ (
        (\\\\)       | # UNC prefix
        ([A-Za-z]\:)   # drive prefix
    )
"#);

static STARTS_WITH_WINDOWS_PATH_PREFIX_OR_SLASH: StaticRegExp = static_reg_exp!(r#"(?x)
    ^ (
        (\\\\)             | # UNC prefix
        ([A-Za-z]\:)       | # drive prefix
        [\/\\] ([^\\] | $)   # slash
    )
"#);

static UNC_PREFIX: &str = r"\\";

/// Resolves `path2` relative to `path1`. This methodd
/// has the same behavior from [`crate::common::resolve`],
/// except that if given `manipulation` is not `Default`,
/// it can handle absolute paths such as from the Windows operating system.
pub fn resolve(path1: &str, path2: &str, manipulation: PlatformPathVariant) -> String {
    match manipulation {
        PlatformPathVariant::Default => {
            crate::common::resolve(path1, path2)
        },
        PlatformPathVariant::Windows => {
            let paths = [path1, path2].map(|p| p.to_owned());
            let prefixed: Vec<String> = paths.iter().filter(|path| STARTS_WITH_WINDOWS_PATH_PREFIX.is_match(path)).cloned().collect();
            if prefixed.is_empty() {
                return crate::common::resolve(path1, path2);
            }
            let prefix = STARTS_WITH_WINDOWS_PATH_PREFIX.find(prefixed.last().unwrap().as_ref()).map(|m| m.as_str().to_owned()).unwrap();
            let paths: Vec<String> = paths.iter().map(|path| STARTS_WITH_WINDOWS_PATH_PREFIX.replace(path.as_ref(), |_: &RegExpCaptures| "/").into_owned()).collect();
            let r = crate::common::resolve(&paths[0], &paths[1]);
            if prefix == UNC_PREFIX {
                return UNC_PREFIX.to_owned() + &r[1..];
            }
            prefix + &r
        },
    }
}

/// Resolves multiple paths with the same behavior from
/// [`resolve`].
pub fn resolve_n<'a, T: IntoIterator<Item = &'a str>>(paths: T, manipulation: PlatformPathVariant) -> String {
    let paths = paths.into_iter().collect::<Vec<&'a str>>();
    if paths.is_empty() {
        return "".to_owned();
    }
    if paths.len() == 1 {
        return resolve(paths[0], "", manipulation);
    }
    let initial_path = resolve(paths[0], paths[1], manipulation);
    paths[2..].iter().fold(initial_path, |a, b| resolve(&a, b, manipulation))
}

/// Resolves a single path with the same behavior from
/// [`resolve_n`].
pub fn resolve_one(path: &str, manipulation: PlatformPathVariant) -> String {
    resolve_n([path], manipulation)
}

/// Determines if a path is absolute. If manipulation is `Default`,
/// absolute paths only start with a path separator.
pub fn is_absolute(path: &str, manipulation: PlatformPathVariant) -> bool {
    match manipulation {
        PlatformPathVariant::Default => STARTS_WITH_PATH_SEPARATOR.is_match(path),
        PlatformPathVariant::Windows => STARTS_WITH_WINDOWS_PATH_PREFIX_OR_SLASH.is_match(path),
    }
}

/// Finds the relative path from `from_path` and `to_path`.
/// This method has the same behavior from [`crate::common::relative`],
/// except that if given `manipulation` is not `Default`,
/// it can handle absolute paths such as from the Windows operating system.
/// If the paths have a different prefix, this function returns
/// `resolve_one(to_path, manipulation)`.
///
/// # Exception
/// 
/// Panics if given paths are not absolute.
///
pub fn relative(from_path: &str, to_path: &str, manipulation: PlatformPathVariant) -> String {
    match manipulation {
        PlatformPathVariant::Default =>
            crate::common::relative(from_path, to_path),
        PlatformPathVariant::Windows => {
            assert!(
                [from_path.to_owned(), to_path.to_owned()].iter().all(|path| is_absolute(path, manipulation)),
                "file_paths::argumented::relative() requires absolute paths as arguments"
            );
            let mut paths = [from_path, to_path].map(|s| s.to_owned());
            let prefixes: Vec<String> = paths.iter().map(|path| STARTS_WITH_WINDOWS_PATH_PREFIX_OR_SLASH.find(path.as_ref()).unwrap().as_str().into()).collect();
            let prefix = prefixes[0].clone();
            if prefix != prefixes[1] {
                return resolve_one(to_path, manipulation);
            }
            /*
            for i in 0..2 {
                paths[i] = paths[i][prefix.len()..].to_owned();
                if !STARTS_WITH_PATH_SEPARATOR.is_match(paths[i].as_ref()) {
                    paths[i] = "/".to_owned() + paths[i].as_ref();
                }
            }
            */
            for path in &mut paths {
                *path = path[prefix.len()..].to_owned();
                if !STARTS_WITH_PATH_SEPARATOR.is_match(path.as_ref()) {
                    *path = "/".to_owned() + path.as_ref();
                }
            }
            crate::common::relative(paths[0].as_ref(), paths[1].as_ref())
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(r"\\Whack/a/Box", resolve_n(["foo", r"\\Whack////a//Box", "..", "Box"], PlatformPathVariant::Windows));
        assert_eq!("C:/a", resolve("C:/", "a", PlatformPathVariant::Windows));
        assert_eq!("D:/", resolve("C:/", "D:/", PlatformPathVariant::Windows));
        assert_eq!("D:/a", resolve_one("D:/a", PlatformPathVariant::Windows));
        assert_eq!("C:/a/f/b", resolve("a", "C:/a///f//b", PlatformPathVariant::Windows));
        assert_eq!("", relative("C:/", "C:/", PlatformPathVariant::Windows));
        assert_eq!("", relative("C:/foo", "C:/foo", PlatformPathVariant::Windows));
        assert_eq!(r"\\foo", relative("C:/", r"\\foo", PlatformPathVariant::Windows));
        assert_eq!("../../foo", relative(r"\\a/b", r"\\foo", PlatformPathVariant::Windows));
        assert_eq!("D:/", relative("C:/", r"D:", PlatformPathVariant::Windows));
    }
}