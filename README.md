# File Paths

Work with generic file paths, including relativity and resolution. This crate includes three versions:

- _Common:_ functions simply consider absolute paths as starting with a path separator.
- _Argumented:_ functions consider absolute paths according to given _PlatformPathVariant_.
- _Native:_ functions consider absolute paths according to the build target.