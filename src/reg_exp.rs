pub use lazy_regex::{
    regex as reg_exp,
    lazy_regex as static_reg_exp,
    regex::{
        Regex as RegExp,
        Match as RegExpMatch,
        Error as RegExpError,
        Captures as RegExpCaptures,
        CaptureMatches as RegExpCaptureMatches,
        CaptureNames as RegExpCaptureNames,
        CaptureLocations as RegExpCaptureLocations,
        SubCaptureMatches as RegExpSubCaptureMatches,
    },
    regex::Replacer as RegExpReplacer,

    regex_captures as reg_exp_captures,
    regex_find as reg_exp_find,
    regex_is_match as reg_exp_is_match,
    regex_replace as reg_exp_replace,
    regex_replace_all as reg_exp_replace_all,
};

pub type StaticRegExp = lazy_regex::Lazy<RegExp>;