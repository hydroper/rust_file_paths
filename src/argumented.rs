/*!
This module contains a layer over the common submodule for
handling paths with a _PlatformPathVariant_ variant.
*/

use super::{
    STARTS_WITH_PATH_SEPARATOR,
    reg_exp::*,
    PlatformPathVariant
};

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

pub fn resolve(path1: &str, path2: &str, manipulation: PlatformPathVariant) -> String {
    match manipulation {
        PlatformPathVariant::Common => {
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

pub fn resolve_one(path: &str, manipulation: PlatformPathVariant) -> String {
    resolve_n([path], manipulation)
}

pub fn is_absolute(path: &str, manipulation: PlatformPathVariant) -> bool {
    match manipulation {
        PlatformPathVariant::Common => STARTS_WITH_PATH_SEPARATOR.is_match(path),
        PlatformPathVariant::Windows => STARTS_WITH_WINDOWS_PATH_PREFIX_OR_SLASH.is_match(path),
    }
}

pub fn relative(from_path: &str, to_path: &str, manipulation: PlatformPathVariant) -> String {
    match manipulation {
        PlatformPathVariant::Common =>
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