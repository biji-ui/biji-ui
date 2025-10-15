mod app;

use app::*;
use leptos::prelude::*;

pub mod components;
pub mod icons;
pub mod pages;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    })
}
