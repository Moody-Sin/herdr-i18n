use std::{borrow::Cow, collections::HashMap, sync::OnceLock};

// The catalog must own its strings. A `HashMap<&str, &str>` looks appealing
// because `include_str!` yields `&'static str`, but serde's borrowed string
// deserialization cannot unescape: an entry containing `\n`, `\r`, or a `\u`
// escape fails to parse, and because the whole catalog is deserialized as one
// value that single entry makes every lookup fall back to English. Owning the
// strings costs one allocation at startup and makes escaped entries work.
type Catalog = HashMap<String, String>;

pub(crate) const LOCALIZED_BUILD: bool = true;

static ZH_CN: OnceLock<Result<Catalog, serde_json::Error>> = OnceLock::new();

fn catalog() -> Option<&'static Catalog> {
    ZH_CN
        .get_or_init(|| serde_json::from_str(include_str!("i18n/zh-CN.json")))
        .as_ref()
        .ok()
}

pub(crate) fn initialize() {
    if let Err(error) = ZH_CN.get_or_init(|| serde_json::from_str(include_str!("i18n/zh-CN.json")))
    {
        eprintln!("warning: cannot load Simplified Chinese text catalog: {error}");
    }
}

pub(crate) fn text(english: &'static str) -> &'static str {
    catalog()
        .and_then(|catalog| catalog.get(english))
        .map(String::as_str)
        .unwrap_or(english)
}

pub(crate) fn text_ref(english: &str) -> Cow<'_, str> {
    catalog()
        .and_then(|catalog| catalog.get(english))
        .map(|translated| Cow::Borrowed(translated.as_str()))
        .unwrap_or(Cow::Borrowed(english))
}

pub(crate) fn format(english: &'static str, args: &[(&'static str, String)]) -> String {
    let mut message = text(english).to_owned();
    for (name, value) in args {
        message = message.replace(&format!("{{{name}}}"), value);
    }
    message
}

#[macro_export]
macro_rules! tr {
    ($english:literal) => {{
        static TEXT: ::std::sync::OnceLock<&'static str> = ::std::sync::OnceLock::new();
        *TEXT.get_or_init(|| $crate::i18n::text($english))
    }};
}

#[macro_export]
macro_rules! trf {
    ($english:literal $(, $name:ident = $value:expr)* $(,)?) => {
        $crate::i18n::format(
            $english,
            &[$((stringify!($name), ($value).to_string())),*],
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The catalog is deserialized as one value, so a single malformed entry
    /// silently disables translation for the entire interface. Escaped
    /// sequences are the realistic way to trigger that, because a borrowed
    /// string cannot be unescaped.
    #[test]
    fn catalog_parses() {
        let parsed: Result<Catalog, _> = serde_json::from_str(include_str!("i18n/zh-CN.json"));

        assert!(
            parsed.is_ok(),
            "zh-CN.json must deserialize, otherwise every lookup falls back to English: {:?}",
            parsed.err()
        );
    }

    #[test]
    fn escaped_sequences_deserialize() {
        let catalog: Catalog = serde_json::from_str(r#"{"line\nbreak": "换行"}"#).unwrap();

        assert_eq!(catalog.get("line\nbreak").map(String::as_str), Some("换行"));
    }

    #[test]
    fn unknown_keys_fall_back_to_english() {
        let key = "this key is deliberately absent from the catalog";

        assert_eq!(text(key), key);
        assert_eq!(text_ref(key), key);
    }
}
