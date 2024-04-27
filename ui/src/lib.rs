pub mod components;
pub mod custom_animated_show;
pub mod items;

pub trait Style {
    fn style(&self) -> String;
}

impl Style for String {
    fn style(&self) -> String {
        self.to_string()
    }
}

impl Style for Option<String> {
    fn style(&self) -> String {
        self.as_ref().map(|s| s.style()).unwrap_or_default()
    }
}

impl Style for &str {
    fn style(&self) -> String {
        self.to_string()
    }
}

#[macro_export]
macro_rules! cn {
    ($($styles: expr),*) => {
        {
            use $crate::Style;
            let mut result = String::new();
            $(
                result.push_str(" ");
                result.push_str(&$styles.style());
            )*
            result
        }
    };
}
