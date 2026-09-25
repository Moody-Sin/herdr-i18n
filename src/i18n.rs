use std::{collections::HashMap, sync::OnceLock};

type Catalog = HashMap<&'static str, &'static str>;

pub(crate) const LOCALIZED_BUILD: bool = true;

static ZH_CN: OnceLock<Result<Catalog, serde_json::Error>> = OnceLock::new();

fn catalog() -> Option<&'static Catalog> {
    match ZH_CN.get_or_init(|| serde_json::from_str(include_str!("i18n/zh-CN.json"))) {
        Ok(catalog) => Some(catalog),
        Err(_) => None,
    }
}

pub(crate) fn initialize() {
    if let Err(error) = ZH_CN.get_or_init(|| serde_json::from_str(include_str!("i18n/zh-CN.json")))
    {
        eprintln!("warning: cannot load Simplified Chinese text catalog: {error}");
    }
}

pub(crate) fn text(english: &'static str) -> &'static str {
    catalog()
        .and_then(|catalog| catalog.get(english).copied())
        .unwrap_or(english)
}

pub(crate) fn text_ref(english: &str) -> std::borrow::Cow<'_, str> {
    catalog()
        .and_then(|catalog| catalog.get(english).copied())
        .map(std::borrow::Cow::Borrowed)
        .unwrap_or_else(|| std::borrow::Cow::Borrowed(english))
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
