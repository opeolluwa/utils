use crate::partials::footer::AppFooter;
use crate::partials::nav::AppNavigation;
use leptos::{component, view, Children, IntoView, ReadSignal};

/// the default application layout, with footer, viewbox, and navigation
#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <AppNavigation/>
        <main class="container my-8">{children()}</main>
        <AppFooter/>
    }
}
