use std::sync::Arc;

/// A string prop that accepts static strings, owned `String`s, or reactive
/// closures (`move || ...`). Use this instead of `Option<String>` when the
/// value may need to update reactively (e.g. i18n labels that change when
/// the locale changes).
///
/// # Examples
///
/// ```rust,ignore
/// // Static &str
/// aria_label="Toggle menu"
///
/// // Owned String
/// aria_label=some_string
///
/// // Reactive closure — re-evaluated when signals inside it change
/// aria_label=move || t_string!(i18n, nav.toggle)
/// ```
#[derive(Clone)]
pub struct StringProp(Arc<dyn Fn() -> String + Send + Sync>);

impl StringProp {
    pub fn get(&self) -> String {
        (self.0)()
    }
}

impl<S: Into<String>, F: Fn() -> S + Send + Sync + 'static> From<F> for StringProp {
    fn from(f: F) -> Self {
        StringProp(Arc::new(move || f().into()))
    }
}

impl From<&str> for StringProp {
    fn from(s: &str) -> Self {
        let s = s.to_string();
        StringProp(Arc::new(move || s.clone()))
    }
}

impl From<String> for StringProp {
    fn from(s: String) -> Self {
        StringProp(Arc::new(move || s.clone()))
    }
}

impl Default for StringProp {
    fn default() -> Self {
        StringProp(Arc::new(|| String::new()))
    }
}
