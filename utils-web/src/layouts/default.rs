use crate::partials::footer::AppFooter;
use crate::partials::nav::AppNavigation;
use leptos::{component, view, Children, IntoView};

/// the default application layout, with footer, viewbox, and navigation
#[component]
pub fn Layout(children: Children, class: &'static str) -> impl IntoView {
    view! {
        <AppNavigation/>
        <main class="container my-8">{children()}</main>
        <AppFooter/>
    }
}
